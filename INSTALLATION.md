# Vintage Story AI Assistant - Installation Guide

## Prerequisites

- Windows 10+, macOS 10.15+, or Linux (Ubuntu 18.04+)
- 8GB RAM minimum (16GB recommended)
- 10GB free disk space
- Internet connection for initial setup

## Quick Start

### 1. Download the Installer

Download the appropriate installer for your platform:

- **Windows**: `VintageStoryAI-Setup.exe`
- **macOS**: `VintageStoryAI.dmg`
- **Linux**: `VintageStoryAI.AppImage`

### 2. Install the Application

#### Windows
1. Double-click the `.exe` file
2. Follow the installation wizard
3. Choose installation directory (default recommended)
4. Click "Install"

#### macOS
1. Open the `.dmg` file
2. Drag the Vintage Story AI Assistant to Applications
3. First run: Right-click and select "Open" to bypass Gatekeeper

#### Linux
1. Make the AppImage executable: `chmod +x VintageStoryAI.AppImage`
2. Run: `./VintageStoryAI.AppImage`

### 3. First Launch Setup

When you first launch the application:

1. **Ollama Installation**: The app will automatically download and install Ollama if not present
2. **Model Download**: The default AI model (phi-3:mini) will be downloaded (~2GB)
3. **Wiki Content**: Click "Update Wiki" to download the Vintage Story knowledge base

## Building from Source

### Requirements

- Rust 1.70+ 
- Node.js 18+
- Git

### Build Steps

```bash
# Clone the repository
git clone https://github.com/yourusername/vintage-story-ai-assistant.git
cd vintage-story-ai-assistant

# Install frontend dependencies
npm install

# Build the application
npm run tauri build
```

The built applications will be in `src-tauri/target/release/bundle/`

## Configuration

### Default Settings

- **AI Model**: phi-3:mini (efficient, 3.8B parameters)
- **Temperature**: 0.7 (balanced creativity)
- **Context Chunks**: 5 (amount of wiki content used)

### Advanced Configuration

Edit `~/.vintage-story-ai/config.json`:

```json
{
  "ollama": {
    "model": "llama3.2:3b",
    "port": 11434
  },
  "embedding": {
    "model": "nomic-embed-text",
    "chunk_size": 512
  }
}
```

## Troubleshooting

### Ollama Installation Failed

**Windows**: Run as Administrator
```powershell
# Manual Ollama install
winget install Ollama.Ollama
```

**macOS/Linux**: Install manually
```bash
curl -fsSL https://ollama.ai/install.sh | sh
```

### Model Download Stuck

1. Close the application
2. Open terminal/command prompt
3. Run: `ollama pull phi3:mini`
4. Restart the application

### Wiki Update Errors

- Check internet connection
- Verify firewall isn't blocking the app
- Try updating again after a few minutes

### High Memory Usage

- Switch to a smaller model in Settings
- Reduce "Context Chunks" in Settings
- Close other applications

## Offline Usage

After initial setup, the application works completely offline:

1. AI models are stored locally
2. Wiki content is cached
3. No internet connection required

## Updating

### Auto-Update
The application checks for updates on launch and can update itself.

### Manual Update
Download the latest installer and run it - your data and settings will be preserved.

## Data Storage

All data is stored locally:

- **Windows**: `%APPDATA%\vintage-story-ai-assistant\`
- **macOS**: `~/Library/Application Support/vintage-story-ai-assistant/`
- **Linux**: `~/.config/vintage-story-ai-assistant/`

This includes:
- Chat history
- Wiki content
- Vector embeddings
- User settings

## Privacy

- No data is sent to external servers
- All AI processing happens locally
- Chat history stays on your device
- Wiki content is downloaded once and cached

## Support

If you encounter issues:

1. Check the [Troubleshooting](#troubleshooting) section
2. Look for error messages in the app
3. Check logs in the data directory
4. Report issues on GitHub

## System Requirements

### Minimum
- CPU: 4 cores
- RAM: 8GB
- Storage: 10GB
- GPU: Not required

### Recommended
- CPU: 6+ cores
- RAM: 16GB
- Storage: 20GB (for multiple models)
- GPU: Optional (speeds up some operations)

## FAQ

**Q: Can I use different AI models?**
A: Yes! Go to Settings and select from available models. You can also download new ones.

**Q: How often should I update the wiki?**
A: Monthly updates are usually sufficient unless major game updates occur.

**Q: Can I export my chat history?**
A: Yes, click the history button and use the export feature for any conversation.

**Q: Does it work with Vintage Story mods?**
A: The base knowledge is vanilla only, but you can ask about general modding concepts.

**Q: Is my data secure?**
A: Yes, everything is stored locally and encrypted by your operating system.