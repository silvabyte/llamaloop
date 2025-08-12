# 🦙 Ollamamon

A beautiful Terminal User Interface (TUI) for managing and monitoring Ollama models with a magical developer experience.

## Features

- **Interactive Dashboard** - Real-time system status and model monitoring
- **Model Management** - List, pull, and delete Ollama models
- **Activity Logs** - Stream and monitor Ollama activity logs
- **Keyboard Navigation** - Fast and intuitive keyboard shortcuts
- **Beautiful UI** - Modern terminal interface with colors and borders

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/ollamamon.git
cd ollamamon

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

Then launch Ollamamon:
```bash
./target/release/ollamamon
```

Or if installed globally:
```bash
ollamamon
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
- `p` - Pull model (in Models tab)
- `d` - Delete model (in Models tab)  
- `?` - Show help
- `Ctrl+C` / `Ctrl+Q` - Quit

## Screenshots

The TUI features four main tabs:

1. **Dashboard** - Overview of system status, running models, and recent activity
2. **Models** - Manage available Ollama models
3. **Logs** - View streaming logs and system activity
4. **Chat** - Interactive chat interface (coming soon)

## Configuration

Ollamamon connects to Ollama at `http://localhost:11434` by default. You can set a custom host:

```bash
OLLAMA_HOST=http://192.168.1.100:11434 ollamamon
```

## Requirements

- Rust 1.70 or later
- Ollama installed and running
- Terminal with Unicode and color support

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