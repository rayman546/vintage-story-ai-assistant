# Vintage Story AI Assistant

A local, offline-first AI assistant for the game Vintage Story, built with Tauri, Rust, and React.

## Features

- **Offline-first**: All AI processing happens locally
- **Vintage Story specific**: Built-in knowledge from the official wiki
- **Cross-platform**: Runs on Windows, macOS, and Linux
- **Local LLM**: Uses Ollama for AI inference
- **RAG System**: Retrieval-Augmented Generation for accurate responses

## Development Setup

### Prerequisites

- Rust (1.85.0+)
- Node.js (18+)
- Tauri CLI (installed automatically)

### Installation

1. Clone this repository
2. Install frontend dependencies:
   ```bash
   npm install
   ```
3. Install Rust dependencies:
   ```bash
   cd src-tauri
   cargo build
   ```

### Running in Development

```bash
# Start the development server
npm run tauri dev
```

### Building for Production

```bash
# Build the application
npm run tauri build
```

## Project Structure

```
vintage-story-ai-assistant/
├── src/                    # React frontend
│   ├── App.tsx            # Main application component
│   ├── main.tsx           # React entry point
│   └── styles.css         # Global styles
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── commands/      # Tauri commands (API endpoints)
│   │   ├── services/      # Core business logic
│   │   ├── config.rs      # Configuration management
│   │   ├── errors.rs      # Error handling
│   │   └── main.rs        # Application entry point
│   ├── Cargo.toml         # Rust dependencies
│   ├── tauri.conf.json    # Tauri configuration
│   └── build.rs           # Build script
├── package.json           # Frontend dependencies
├── vite.config.ts         # Vite configuration
└── tailwind.config.js     # Tailwind CSS configuration
```

## Implementation Status

### ✅ Completed
- Basic Tauri project structure
- Frontend chat interface
- Ollama service integration (basic)
- Command structure for frontend-backend communication

### 🚧 In Progress
- Wiki scraping and processing
- Vector database integration (LanceDB)
- Embedding service implementation
- RAG pipeline

### 📋 Planned
- Model download interface
- Wiki content management
- Advanced chat features
- Cross-platform packaging
- Auto-updater

## Technical Stack

- **Frontend**: React 18 + TypeScript + Tailwind CSS
- **Backend**: Rust + Tauri
- **LLM Engine**: Ollama
- **Vector Database**: LanceDB
- **Build Tool**: Vite
- **Package Manager**: npm

## Contributing

This project is in early development. Contributions are welcome!

## License

MIT License - see LICENSE file for details.
