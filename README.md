# 🔄 llamaloop

A hypnotic Terminal User Interface (TUI) for orchestrating Ollama models in perfect harmony. Part of the [codeloops.ai](https://codeloops.ai) universe.

## Features

- **Interactive Dashboard** - Real-time system status and model monitoring  
- **Model Library** - Browse both installed and available models from models.dev
- **Smart Model Management** - Install, update, and delete models with visual feedback
- **Tokyo Night Theme** - Beautiful dark theme with sparkle animations ✨
- **Activity Logs** - Stream and monitor Ollama activity logs
- **Keyboard Navigation** - Fast and intuitive keyboard shortcuts
- **Three View Modes** - Switch between All, Installed, or Available models

## Installation

```bash
# Clone the repository
git clone https://github.com/silvabyte/llamaloop.git
cd llamaloop

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Usage

Make sure Ollama is running:
```bash
ollama serve
```

Then enter the loop:
```bash
./target/release/llamaloop
```

Or if installed globally:
```bash
llamaloop
```

## Keyboard Shortcuts

### Navigation
- `Tab` / `Shift+Tab` - Navigate between tabs
- `1-4` - Jump to specific tab (Dashboard, Models, Logs, Chat)
- `↑` / `↓` - Navigate lists
- `Enter` - Select/Confirm
- `Esc` - Cancel/Back

### Commands
- `r` - Refresh data
- `v` - Switch view mode (in Models tab: All/Installed/Available)
- `i` - Install selected model (in Models tab)
- `p` - Pull model by name (in Models tab)
- `Shift+D` - Delete model with confirmation (in Models tab)  
- `?` - Show help
- `Ctrl+C` / `Ctrl+Q` - Quit

### Chat Tab Commands
- `i` or `e` - Enter input mode to type messages
- `Enter` - Send message (when in input mode)
- `Esc` - Exit input mode
- `c` - Clear chat session
- `m` - Change model
- `n` - New chat session

## Screenshots

The TUI features four main tabs:

1. **Dashboard** - Overview of system status, running models, and recent activity
2. **Models** - Manage available Ollama models
3. **Logs** - View streaming logs and system activity
4. **Chat** - Interactive chat interface with streaming responses

## Configuration

llamaloop connects to Ollama at `http://localhost:11434` by default. You can set a custom host:

```bash
OLLAMA_HOST=http://192.168.1.100:11434 llamaloop
```

## Requirements

- Rust 1.70 or later
- Ollama installed and running
- Terminal with Unicode and color support

## Utility Scripts

The `scripts/` directory contains helpful utilities for setting up and configuring Ollama:

### Migration Scripts
- **`migrate_to_ollama.sh`** - Migrates models from LM Studio to Ollama
- **`setup_ollama_network.sh`** - Configures Ollama for network access on your local network
- **`configure_ollama_memory.sh`** - Optimizes memory settings for large models
- **`test_ollama_tools.sh`** - Tests tool/function calling capabilities

Run any script with:
```bash
./scripts/[script-name].sh
```

## Development

```bash
# Run in development mode
cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Check lints
cargo clippy
```

## License

MIT