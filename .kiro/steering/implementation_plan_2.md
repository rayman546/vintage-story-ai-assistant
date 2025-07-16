# Implementation Plan 2: Bug Fixes and Missing Components

## Overview
This plan addresses critical bugs and missing components identified in the Vintage Story AI Assistant repository. Issues are prioritized by severity and impact on functionality.

## Priority Levels
- 🔴 **Critical**: Prevents compilation/runtime
- 🟡 **High**: Causes runtime errors or poor UX
- 🟢 **Medium**: Code quality/maintainability issues

---

## 🔴 Critical Issues (Immediate Action Required)

### 1. Fix Tauri Plugin Configuration

**File**: `src-tauri/tauri.conf.json`

**Problem**: Missing plugin configurations causing runtime failures

**Action**:
```json
{
  "$schema": "https://schema.tauri.app/config/2.0.0",
  "productName": "Vintage Story AI Assistant",
  "version": "0.1.0",
  "identifier": "com.vintagestory.ai-assistant",
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:1420",
    "frontendDist": "../dist"
  },
  "plugins": {
    "fs": {
      "all": true,
      "scope": ["**"]
    },
    "http": {
      "all": true,
      "scope": ["https://**", "http://127.0.0.1:11434/**"]
    },
    "shell": {
      "all": true,
      "scope": [
        {
          "name": "ollama",
          "cmd": "ollama",
          "args": true
        },
        {
          "name": "curl",
          "cmd": "curl",
          "args": ["-fsSL", "https://ollama.ai/install.sh"]
        }
      ]
    }
  },
  "app": {
    "security": {
      "csp": "default-src 'self' data: filesystem: asset: https://asset.localhost http://127.0.0.1:11434; script-src 'self' 'unsafe-inline' data: filesystem: asset: https://asset.localhost; style-src 'self' 'unsafe-inline' data: filesystem: asset: https://asset.localhost; img-src 'self' data: filesystem: asset: https://asset.localhost blob:; font-src 'self' data: filesystem: asset: https://asset.localhost"
    },
    "windows": [
      {
        "width": 1200,
        "height": 800,
        "resizable": true,
        "title": "Vintage Story AI Assistant",
        "center": true,
        "minWidth": 800,
        "minHeight": 600
      }
    ]
  }
}
```

### 2. Fix Package.json NPM Script

**File**: `package.json`

**Problem**: Incomplete tauri script

**Action**: Replace line 9:
```json
"tauri": "tauri dev"
```

### 3. Create Missing React Components

**Problem**: Import errors for missing components

#### 3.1. Create MessageRenderer Component

**File**: `src/components/MessageRenderer.tsx`

**Action**: Create file with:
```tsx
import React from 'react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { oneDark } from 'react-syntax-highlighter/dist/esm/styles/prism';

interface MessageRendererProps {
  content: string;
  isUser: boolean;
}

export const MessageRenderer: React.FC<MessageRendererProps> = ({ content, isUser }) => {
  if (isUser) {
    return <div className="whitespace-pre-wrap">{content}</div>;
  }

  return (
    <ReactMarkdown
      remarkPlugins={[remarkGfm]}
      components={{
        code({ node, inline, className, children, ...props }) {
          const match = /language-(\w+)/.exec(className || '');
          return !inline && match ? (
            <SyntaxHighlighter
              style={oneDark}
              language={match[1]}
              PreTag="div"
              {...props}
            >
              {String(children).replace(/\n$/, '')}
            </SyntaxHighlighter>
          ) : (
            <code className={className} {...props}>
              {children}
            </code>
          );
        },
      }}
    >
      {content}
    </ReactMarkdown>
  );
};
```

#### 3.2. Create Settings Component

**File**: `src/components/Settings.tsx`

**Action**: Create file with:
```tsx
import React from 'react';
import { X, Monitor, Moon, Sun } from 'lucide-react';

interface ModelInfo {
  name: string;
  size: number;
  digest: string;
  details: {
    parameter_size: string;
    quantization_level: string;
    family: string;
  };
}

interface SettingsProps {
  isOpen: boolean;
  onClose: () => void;
  selectedModel: string;
  onModelChange: (model: string) => void;
  availableModels: ModelInfo[];
  temperature: number;
  onTemperatureChange: (temp: number) => void;
  maxContextChunks: number;
  onMaxContextChunksChange: (chunks: number) => void;
  theme: 'light' | 'dark';
  onThemeChange: (theme: 'light' | 'dark') => void;
}

export const Settings: React.FC<SettingsProps> = ({
  isOpen,
  onClose,
  selectedModel,
  onModelChange,
  availableModels,
  temperature,
  onTemperatureChange,
  maxContextChunks,
  onMaxContextChunksChange,
  theme,
  onThemeChange,
}) => {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg shadow-xl w-96 max-h-[80vh] overflow-y-auto">
        <div className="flex items-center justify-between p-4 border-b">
          <h2 className="text-lg font-semibold">Settings</h2>
          <button onClick={onClose} className="p-1 hover:bg-gray-100 rounded">
            <X className="w-5 h-5" />
          </button>
        </div>
        
        <div className="p-4 space-y-6">
          {/* Model Selection */}
          <div>
            <label className="block text-sm font-medium mb-2">Model</label>
            <select
              value={selectedModel}
              onChange={(e) => onModelChange(e.target.value)}
              className="w-full p-2 border rounded-lg"
            >
              {availableModels.map((model) => (
                <option key={model.name} value={model.name}>
                  {model.name} ({model.details.parameter_size})
                </option>
              ))}
            </select>
          </div>

          {/* Temperature */}
          <div>
            <label className="block text-sm font-medium mb-2">
              Temperature: {temperature}
            </label>
            <input
              type="range"
              min="0"
              max="2"
              step="0.1"
              value={temperature}
              onChange={(e) => onTemperatureChange(parseFloat(e.target.value))}
              className="w-full"
            />
            <div className="flex justify-between text-xs text-gray-500 mt-1">
              <span>Conservative</span>
              <span>Creative</span>
            </div>
          </div>

          {/* Max Context Chunks */}
          <div>
            <label className="block text-sm font-medium mb-2">
              Max Context Chunks: {maxContextChunks}
            </label>
            <input
              type="range"
              min="1"
              max="10"
              step="1"
              value={maxContextChunks}
              onChange={(e) => onMaxContextChunksChange(parseInt(e.target.value))}
              className="w-full"
            />
          </div>

          {/* Theme */}
          <div>
            <label className="block text-sm font-medium mb-2">Theme</label>
            <div className="flex space-x-2">
              <button
                onClick={() => onThemeChange('light')}
                className={`flex items-center space-x-2 px-3 py-2 rounded-lg border ${
                  theme === 'light' ? 'bg-blue-50 border-blue-300' : 'border-gray-300'
                }`}
              >
                <Sun className="w-4 h-4" />
                <span>Light</span>
              </button>
              <button
                onClick={() => onThemeChange('dark')}
                className={`flex items-center space-x-2 px-3 py-2 rounded-lg border ${
                  theme === 'dark' ? 'bg-blue-50 border-blue-300' : 'border-gray-300'
                }`}
              >
                <Moon className="w-4 h-4" />
                <span>Dark</span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
```

#### 3.3. Create ChatHistory Component

**File**: `src/components/ChatHistory.tsx`

**Action**: Create file with:
```tsx
import React, { useRef } from 'react';
import { X, Trash2, Download, Upload, Plus } from 'lucide-react';

interface ChatSession {
  id: string;
  title: string;
  timestamp: string;
  messages: any[];
}

interface ChatHistoryProps {
  isOpen: boolean;
  onClose: () => void;
  sessions: ChatSession[];
  currentSessionId: string | null;
  onSelectSession: (id: string) => void;
  onDeleteSession: (id: string) => void;
  onNewSession: () => void;
  onExportSession: (id: string) => void;
  onImportSession: (file: File) => void;
}

export const ChatHistory: React.FC<ChatHistoryProps> = ({
  isOpen,
  onClose,
  sessions,
  currentSessionId,
  onSelectSession,
  onDeleteSession,
  onNewSession,
  onExportSession,
  onImportSession,
}) => {
  const fileInputRef = useRef<HTMLInputElement>(null);

  if (!isOpen) return null;

  const handleImport = () => {
    fileInputRef.current?.click();
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      onImportSession(file);
      e.target.value = '';
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg shadow-xl w-96 max-h-[80vh] overflow-hidden">
        <div className="flex items-center justify-between p-4 border-b">
          <h2 className="text-lg font-semibold">Chat History</h2>
          <button onClick={onClose} className="p-1 hover:bg-gray-100 rounded">
            <X className="w-5 h-5" />
          </button>
        </div>
        
        <div className="p-4">
          <div className="flex space-x-2 mb-4">
            <button
              onClick={onNewSession}
              className="flex items-center space-x-1 px-3 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600"
            >
              <Plus className="w-4 h-4" />
              <span>New</span>
            </button>
            <button
              onClick={handleImport}
              className="flex items-center space-x-1 px-3 py-2 border rounded-lg hover:bg-gray-50"
            >
              <Upload className="w-4 h-4" />
              <span>Import</span>
            </button>
          </div>
          
          <input
            ref={fileInputRef}
            type="file"
            accept=".json"
            onChange={handleFileChange}
            className="hidden"
          />
        </div>

        <div className="max-h-96 overflow-y-auto">
          {sessions.length === 0 ? (
            <div className="p-4 text-center text-gray-500">
              No chat history yet
            </div>
          ) : (
            sessions.map((session) => (
              <div
                key={session.id}
                className={`p-3 border-b hover:bg-gray-50 cursor-pointer ${
                  currentSessionId === session.id ? 'bg-blue-50' : ''
                }`}
                onClick={() => onSelectSession(session.id)}
              >
                <div className="flex items-start justify-between">
                  <div className="flex-1 min-w-0">
                    <h3 className="font-medium truncate">{session.title}</h3>
                    <p className="text-sm text-gray-500">
                      {new Date(session.timestamp).toLocaleDateString()}
                    </p>
                    <p className="text-xs text-gray-400">
                      {session.messages.length} messages
                    </p>
                  </div>
                  <div className="flex space-x-1 ml-2">
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        onExportSession(session.id);
                      }}
                      className="p-1 hover:bg-gray-200 rounded"
                    >
                      <Download className="w-4 h-4" />
                    </button>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        onDeleteSession(session.id);
                      }}
                      className="p-1 hover:bg-red-100 rounded text-red-600"
                    >
                      <Trash2 className="w-4 h-4" />
                    </button>
                  </div>
                </div>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
};
```

---

## 🟡 High Priority Issues

### 4. Fix OllamaManager Race Conditions and Memory Leaks

**File**: `src-tauri/src/services/ollama_manager.rs`

#### 4.1. Add Process Cleanup
**Action**: Add Drop implementation:
```rust
impl Drop for OllamaManager {
    fn drop(&mut self) {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}
```

#### 4.2. Fix Streaming Response Error Handling
**Action**: Replace lines 375-385:
```rust
// Process streaming response with proper error handling
while let Some(chunk_bytes) = response.chunk().await? {
    if let Ok(text) = std::str::from_utf8(&chunk_bytes) {
        for line in text.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<serde_json::Value>(line) {
                Ok(json) => {
                    if let Some(status) = json["status"].as_str() {
                        let total = json["total"].as_u64().unwrap_or(100) as f32;
                        let completed = json["completed"].as_u64().unwrap_or(0) as f32;
                        let progress = if total > 0.0 { completed / total } else { 0.0 };
                        progress_callback(progress, status.to_string());
                    }
                }
                Err(e) => {
                    warn!("Failed to parse streaming response line: {} - Error: {}", line, e);
                    continue;
                }
            }
        }
    }
}
```

#### 4.3. Add Download Integrity Check
**Action**: Add after line 243:
```rust
// Verify download integrity (basic size check)
if installer_bytes.len() < 1024 * 1024 {  // Less than 1MB seems suspicious
    return Err(AppError::OllamaError(
        "Downloaded installer appears corrupted (too small)".to_string()
    ));
}
```

### 5. Implement Configuration System

**File**: `src-tauri/src/config.rs`

**Action**: Replace the TODO implementations:
```rust
use std::fs;

impl AppConfig {
    pub fn load() -> crate::errors::AppResult<Self> {
        let config_path = Self::get_config_path();
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| crate::errors::AppError::ConfigError(
                    format!("Failed to read config file: {}", e)
                ))?;
            
            let config: AppConfig = serde_json::from_str(&content)
                .map_err(|e| crate::errors::AppError::ConfigError(
                    format!("Failed to parse config file: {}", e)
                ))?;
            
            Ok(config)
        } else {
            // Create default config and save it
            let default_config = Self::default();
            default_config.save()?;
            Ok(default_config)
        }
    }
    
    pub fn save(&self) -> crate::errors::AppResult<()> {
        let config_path = Self::get_config_path();
        
        // Create directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| crate::errors::AppError::ConfigError(
                    format!("Failed to create config directory: {}", e)
                ))?;
        }
        
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| crate::errors::AppError::ConfigError(
                format!("Failed to serialize config: {}", e)
            ))?;
        
        fs::write(&config_path, content)
            .map_err(|e| crate::errors::AppError::ConfigError(
                format!("Failed to write config file: {}", e)
            ))?;
        
        Ok(())
    }
    
    fn get_config_path() -> PathBuf {
        Self::get_data_dir().join("config.json")
    }
}
```

### 6. Fix React useEffect Dependencies

**File**: `src/App.tsx`

**Action**: Replace lines 87-94:
```tsx
useEffect(() => {
  // Auto-save current session when messages change
  if (currentSessionId && messages.length > 0) {
    setChatSessions(prev => prev.map(session => 
      session.id === currentSessionId 
        ? { ...session, messages } 
        : session
    ));
  }
}, [messages, currentSessionId, setChatSessions]);
```

### 7. Add TypeScript Interfaces

**File**: `src/types.ts` (create new file)

**Action**: Create with:
```typescript
export interface ChatMessage {
  id: string;
  content: string;
  role: 'user' | 'assistant' | 'error';
  timestamp: string;
}

export interface ChatSession {
  id: string;
  title: string;
  timestamp: string;
  messages: ChatMessage[];
}

export interface ChatResponse {
  message: ChatMessage;
  context_used: string[];
}

export interface OllamaStatus {
  is_running: boolean;
  is_installed: boolean;
  version?: string;
  models: ModelInfo[];
}

export interface ModelInfo {
  name: string;
  size: number;
  digest: string;
  details: {
    parameter_size: string;
    quantization_level: string;
    family: string;
  };
}

export interface WikiStatus {
  last_update?: string;
  total_pages: number;
  is_updating: boolean;
  pages_scraped: number;
  errors_encountered: number;
}

export interface SystemStatus {
  ollama_ready: boolean;
  wiki_ready: boolean;
  error_message?: string;
}
```

**File**: `src/App.tsx`

**Action**: Replace line 3 with:
```tsx
import { 
  ChatMessage, ChatSession, ChatResponse, OllamaStatus, 
  ModelInfo, WikiStatus, SystemStatus 
} from "./types";
```

**Action**: Remove duplicate interface definitions in App.tsx

---

## 🟢 Medium Priority Issues

### 8. Fix Vector Database Technology Decision

**Problem**: README mentions LanceDB, Cargo.toml uses sled

**Action**: Choose one technology and update accordingly

**Option A - Stick with sled (simpler)**:
- Update README.md to mention sled instead of LanceDB

**Option B - Switch to LanceDB (more advanced)**:
- Update Cargo.toml dependencies:
```toml
# Replace sled with:
lancedb = "0.1"
arrow = "52.0"
```

### 9. Add Error Boundaries

**File**: `src/components/ErrorBoundary.tsx` (create new)

**Action**: Create:
```tsx
import React, { Component, ErrorInfo, ReactNode } from 'react';
import { AlertTriangle } from 'lucide-react';

interface Props {
  children: ReactNode;
}

interface State {
  hasError: boolean;
  error?: Error;
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false
  };

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('Uncaught error:', error, errorInfo);
  }

  public render() {
    if (this.state.hasError) {
      return (
        <div className="flex h-screen items-center justify-center bg-gray-100">
          <div className="text-center p-8 bg-white rounded-lg shadow-lg max-w-md">
            <AlertTriangle className="w-16 h-16 text-red-500 mx-auto mb-4" />
            <h2 className="text-xl font-semibold mb-2">Something went wrong</h2>
            <p className="text-gray-600 mb-4">
              The application encountered an unexpected error.
            </p>
            <button
              onClick={() => window.location.reload()}
              className="bg-blue-500 text-white px-4 py-2 rounded-lg hover:bg-blue-600"
            >
              Reload Application
            </button>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}
```

### 10. Add Input Validation Commands

**File**: `src-tauri/src/commands/validation.rs` (create new)

**Action**: Create:
```rust
use crate::errors::{AppError, AppResult};

pub fn validate_model_name(name: &str) -> AppResult<()> {
    if name.is_empty() {
        return Err(AppError::ConfigError("Model name cannot be empty".to_string()));
    }
    
    if name.len() > 100 {
        return Err(AppError::ConfigError("Model name too long".to_string()));
    }
    
    // Basic validation for model name format
    if !name.chars().all(|c| c.is_alphanumeric() || c == ':' || c == '-' || c == '_') {
        return Err(AppError::ConfigError(
            "Model name contains invalid characters".to_string()
        ));
    }
    
    Ok(())
}

pub fn validate_message_content(content: &str) -> AppResult<()> {
    if content.is_empty() {
        return Err(AppError::ConfigError("Message cannot be empty".to_string()));
    }
    
    if content.len() > 10000 {
        return Err(AppError::ConfigError("Message too long (max 10000 characters)".to_string()));
    }
    
    Ok(())
}
```

---

## Implementation Order

1. **Critical Issues First** (1-3): Must be completed for basic functionality
2. **High Priority** (4-7): Required for stable operation
3. **Medium Priority** (8-10): Code quality and user experience improvements

## Testing Strategy

After each major fix:
1. Run `npm run tauri dev` to verify compilation
2. Test basic Ollama connection
3. Test UI component rendering
4. Verify configuration persistence

## Notes for Agentic IDE

- Each section is independent and can be implemented separately
- Code snippets are complete and ready to use
- File paths are explicit and accurate
- All imports and dependencies are specified
- Error handling follows the existing pattern in the codebase