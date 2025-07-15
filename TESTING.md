# Vintage Story AI Assistant - Testing Guide

## Unit Tests

### Backend (Rust)

To run backend tests:
```bash
cd src-tauri
cargo test
```

Test files to create:
- `src-tauri/src/services/ollama_manager_test.rs`
- `src-tauri/src/services/embedding_service_test.rs`
- `src-tauri/src/services/vector_database_test.rs`
- `src-tauri/src/services/chat_service_test.rs`

### Frontend (React)

To run frontend tests:
```bash
npm test
```

Test files to create:
- `src/__tests__/App.test.tsx`
- `src/__tests__/MessageRenderer.test.tsx`
- `src/__tests__/Settings.test.tsx`
- `src/__tests__/ChatHistory.test.tsx`

## Integration Tests

### RAG Pipeline Test
1. Start Ollama service
2. Create test wiki content
3. Generate embeddings
4. Query with test question
5. Verify response contains relevant context

### End-to-End Test Flow
1. Launch application
2. Check Ollama status
3. Update wiki content
4. Send test message
5. Verify response
6. Check chat history persistence
7. Export/import chat session

## Performance Benchmarks

### Response Time Targets
- Embedding generation: < 500ms per chunk
- Vector search: < 200ms
- Full query response: < 5 seconds

### Memory Usage Targets
- Idle: < 500MB
- Active with model loaded: < 2GB
- With full wiki indexed: < 3GB

## Manual Testing Checklist

### Installation Flow
- [ ] Windows installation works
- [ ] macOS installation works
- [ ] Linux installation works
- [ ] Ollama auto-downloads if missing
- [ ] Default model downloads automatically

### Core Functionality
- [ ] Wiki scraping works
- [ ] Embeddings generate correctly
- [ ] Vector search returns relevant results
- [ ] Chat responses use context
- [ ] Error messages display properly

### UI Features
- [ ] Markdown rendering works
- [ ] Code blocks have syntax highlighting
- [ ] Copy button works for code blocks
- [ ] Settings save and persist
- [ ] Chat history saves automatically
- [ ] Export/import chat works
- [ ] Theme switching works

### Edge Cases
- [ ] Works offline after initial setup
- [ ] Handles Ollama crashes gracefully
- [ ] Recovers from network errors
- [ ] Handles large wiki pages
- [ ] Works with minimal RAM (8GB)

## Automated Test Setup

### GitHub Actions Workflow

Create `.github/workflows/test.yml`:

```yaml
name: Test

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test-backend:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Run tests
      run: |
        cd src-tauri
        cargo test
    
    - name: Check formatting
      run: |
        cd src-tauri
        cargo fmt -- --check
    
    - name: Run clippy
      run: |
        cd src-tauri
        cargo clippy -- -D warnings

  test-frontend:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Node
      uses: actions/setup-node@v3
      with:
        node-version: 18
    
    - name: Install dependencies
      run: npm ci
    
    - name: Run tests
      run: npm test
    
    - name: Build
      run: npm run build
```

## Sample Test Implementation

### Backend Test Example

```rust
// src-tauri/src/services/embedding_service_test.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_chunk_splitting() {
        let service = EmbeddingService::new().await;
        let content = "This is a test. ".repeat(100);
        let chunks = service.split_into_chunks(&content);
        
        assert!(!chunks.is_empty());
        assert!(chunks[0].len() <= service.config.chunk_size);
    }
    
    #[tokio::test]
    async fn test_cosine_similarity() {
        let service = EmbeddingService::new().await;
        let vec1 = vec![1.0, 0.0, 0.0];
        let vec2 = vec![1.0, 0.0, 0.0];
        let vec3 = vec![0.0, 1.0, 0.0];
        
        assert_eq!(service.cosine_similarity(&vec1, &vec2), 1.0);
        assert_eq!(service.cosine_similarity(&vec1, &vec3), 0.0);
    }
}
```

### Frontend Test Example

```typescript
// src/__tests__/MessageRenderer.test.tsx
import { render, screen } from '@testing-library/react';
import { MessageRenderer } from '../components/MessageRenderer';

describe('MessageRenderer', () => {
  it('renders markdown correctly', () => {
    const content = '# Hello\n\nThis is **bold** text';
    render(<MessageRenderer content={content} isUser={false} />);
    
    expect(screen.getByText('Hello')).toBeInTheDocument();
    expect(screen.getByText('bold')).toHaveStyle('font-weight: bold');
  });
  
  it('renders code blocks with syntax highlighting', () => {
    const content = '```javascript\nconst x = 42;\n```';
    render(<MessageRenderer content={content} isUser={false} />);
    
    expect(screen.getByText('const')).toBeInTheDocument();
    expect(screen.getByText('x = 42;')).toBeInTheDocument();
  });
});
```
