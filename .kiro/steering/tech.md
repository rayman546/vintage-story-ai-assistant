# Technology Stack

## Architecture
- **Frontend**: React 18 + TypeScript + Tailwind CSS
- **Backend**: Rust + Tauri framework
- **LLM Engine**: Ollama (local inference)
- **Vector Database**: LanceDB (planned) / Sled (current embedded storage)
- **Build Tool**: Vite
- **Package Manager**: npm

## Frontend Stack
- **React 18**: Modern React with hooks and functional components
- **TypeScript**: Strict typing enabled with ES2020 target
- **Tailwind CSS**: Utility-first CSS framework
- **Lucide React**: Icon library
- **React Markdown**: Markdown rendering with syntax highlighting
- **Vite**: Fast development server and build tool

## Backend Stack
- **Tauri 2.1**: Cross-platform desktop app framework
- **Rust**: Systems programming language
- **Tokio**: Async runtime
- **Reqwest**: HTTP client for Ollama API and wiki scraping
- **Scraper**: HTML parsing for wiki content
- **Sled**: Embedded database for local storage
- **Anyhow/Thiserror**: Error handling

## Development Commands

### Setup
```bash
# Install frontend dependencies
npm install

# Install Rust dependencies (from src-tauri/)
cargo build
```

### Development
```bash
# Start development server with hot reload
npm run tauri dev

# Frontend only (for UI development)
npm run dev
```

### Building
```bash
# Build for production
npm run tauri build

# Frontend build only
npm run build
```

### Testing
```bash
# Run Rust tests
cd src-tauri && cargo test

# Frontend tests (if configured)
npm test
```

## Configuration Files
- `tauri.conf.json`: Tauri app configuration
- `vite.config.ts`: Vite build configuration
- `tsconfig.json`: TypeScript compiler options
- `tailwind.config.js`: Tailwind CSS configuration
- `Cargo.toml`: Rust dependencies and build settings