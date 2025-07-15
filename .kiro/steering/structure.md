# Project Structure

## Root Directory
```
vintage-story-ai-assistant/
├── src/                    # React frontend source
├── src-tauri/             # Rust backend source
├── node_modules/          # Frontend dependencies
├── dist/                  # Built frontend assets
├── package.json           # Frontend dependencies & scripts
├── vite.config.ts         # Vite configuration
├── tailwind.config.js     # Tailwind CSS configuration
├── tsconfig.json          # TypeScript configuration
└── README.md              # Project documentation
```

## Frontend Structure (`src/`)
```
src/
├── App.tsx                # Main application component
├── main.tsx              # React entry point
├── styles.css            # Global styles
└── components/           # Reusable React components
    ├── MessageRenderer.tsx
    ├── Settings.tsx
    └── ChatHistory.tsx
```

## Backend Structure (`src-tauri/`)
```
src-tauri/
├── src/                  # Rust source code
│   ├── main.rs          # Application entry point
│   ├── commands/        # Tauri commands (API endpoints)
│   ├── services/        # Core business logic
│   ├── config.rs        # Configuration management
│   └── errors.rs        # Error handling
├── Cargo.toml           # Rust dependencies
├── tauri.conf.json      # Tauri configuration
├── build.rs             # Build script
├── icons/               # Application icons
└── target/              # Rust build artifacts
```

## Key Architectural Patterns

### Frontend Patterns
- **Component-based**: Modular React components with clear separation of concerns
- **Hook-based state**: Uses React hooks for state management
- **TypeScript interfaces**: Strong typing for all data structures
- **Tailwind utilities**: Utility-first CSS approach

### Backend Patterns
- **Command pattern**: Tauri commands act as API endpoints between frontend and backend
- **Service layer**: Business logic separated into service modules
- **Error handling**: Consistent error handling with `anyhow` and `thiserror`
- **Async/await**: Tokio-based async runtime for non-blocking operations

### Communication
- **Tauri invoke**: Frontend calls backend via `invoke()` function
- **JSON serialization**: All data exchange uses Serde JSON
- **Type safety**: Shared interfaces between frontend and backend

## File Naming Conventions
- **React components**: PascalCase (e.g., `MessageRenderer.tsx`)
- **Rust modules**: snake_case (e.g., `wiki_service.rs`)
- **Configuration files**: kebab-case or standard names
- **Interfaces/Types**: PascalCase with descriptive names