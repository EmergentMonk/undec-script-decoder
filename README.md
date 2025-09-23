# Grok TUI - Terminal User Interface for Grok Model Management

A Rust-based terminal user interface (TUI) application for managing Grok model files and interacting with Grok AI models via API.

## Features

- **Model File Management**: Add and manage .py Grok model files
- **API Key Configuration**: Securely set and manage your Grok API key
- **Model Interaction**: Interface with different Grok models through the API
- **Cross-platform**: Uses ncurses for terminal compatibility

## Installation

### Prerequisites

- Rust (1.70 or later)
- ncurses development libraries

On Ubuntu/Debian:
```bash
sudo apt-get install libncurses5-dev libncursesw5-dev
```

On macOS:
```bash
brew install ncurses
```

### Building

```bash
cargo build --release
```

### Running

```bash
cargo run
```

## Usage

### Main Menu

The application starts with a main menu offering four options:

1. **Manage Model Files (.py)**: Add and view Python model files
2. **Set API Key**: Configure your Grok API key
3. **Interact with Grok Models**: Select and interact with available models
4. **Quit**: Exit the application

### Navigation

- **↑/↓ Arrow Keys**: Navigate between menu items
- **Enter**: Select the highlighted option
- **ESC**: Return to the main menu (from sub-screens)
- **q**: Quit the application (from main menu)

### Configuration

The application stores configuration in:
- Linux/macOS: `~/.config/grok-tui/config.json`
- Windows: `%APPDATA%/grok-tui/config.json`

Configuration includes:
- API key (encrypted storage recommended for production)
- List of managed model files
- Available Grok models

## Example Model File

See `examples/example_grok_model.py` for a sample Python model file demonstrating:
- Ternary Elegance principles
- E8 triality operations
- φ-scaling calculations

## Model Files

The TUI can manage Python files containing Grok models. These files should contain:

- Physics model implementations
- Mathematical operations related to the models
- Any supporting functions or classes

## API Integration

The application supports interaction with Grok models including:
- `grok-beta`: Standard Grok model
- `grok-vision-beta`: Vision-capable Grok model

**Note**: This demo version simulates API interactions. A production version would implement actual HTTP requests to the Grok API endpoints.

## Development

### Project Structure

```
src/
├── main.rs          # Main application logic and TUI implementation
examples/
├── example_grok_model.py  # Sample model file
Cargo.toml          # Rust dependencies
```

### Dependencies

- `pancurses`: Cross-platform ncurses library
- `serde`: Serialization framework for configuration
- `serde_json`: JSON support for configuration files
- `dirs`: Cross-platform directory paths
- `anyhow`: Error handling

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test the TUI application
5. Submit a pull request

## License

MIT License - see LICENSE file for details.
