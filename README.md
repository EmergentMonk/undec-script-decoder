# Grok Physics Models TUI

🔬 A Text User Interface (TUI) for managing and interacting with Grok physics models.

## Overview

This application provides a terminal-based interface for managing Python-based physics models. The models represent various physics concepts including "Ternary Elegance," "E8 triality," and "φ-scaling" as described in the repository description.

## Features

- **Model Management**: Discover and list Python model files in the `models/` directory
- **Interactive Execution**: Run models with JSON input and view formatted output
- **Configuration Management**: Store API keys and settings in `config.toml`
- **Terminal UI**: Navigate using keyboard controls with a clean, organized interface

## Project Structure

```
grok/
├── src/                    # Rust source code
│   ├── main.rs            # Main application entry point
│   ├── config.rs          # Configuration management
│   ├── model_executor.rs  # Python model execution
│   └── ui.rs              # TUI interface
├── models/                # Python physics models
│   ├── ternary_elegance.py
│   ├── e8_triality.py
│   └── phi_scaling.py
├── config.toml           # Configuration file
├── Cargo.toml           # Rust dependencies
└── README.md            # This file
```

## Usage

### Building and Running

```bash
# Build the application
cargo build --release

# Run the application
cargo run
# or
./target/release/grok-tui
```

### Navigation

- **↑/↓**: Navigate through menus and lists
- **Enter**: Select items or confirm actions
- **Esc**: Go back to previous screen
- **Tab**: Switch between input modes (when editing)
- **'q'**: Quit the application

### Model Interaction

1. **Browse Models**: Select "Browse Models" from the main menu
2. **Choose Model**: Select a model from the list
3. **Input Data**: Enter JSON input for the model (e.g., `{"energy": 2.0}`)
4. **Execute**: Press Enter to run the model
5. **View Results**: Review the formatted output

### Adding New Models

1. Place Python files (`.py`) in the `models/` directory
2. Use "Refresh Models" from the main menu to detect new files
3. Models should:
   - Accept JSON input as command line argument
   - Return JSON output to stdout
   - Handle errors gracefully

## Model Examples

### Ternary Elegance
Explores ternary relationships in fundamental constants using the golden ratio.

```bash
python3 models/ternary_elegance.py '{"energy": 2.0}'
```

### E8 Triality
Investigates E8 Lie group triality relationships in physics.

```bash
python3 models/e8_triality.py '{"x": 1.0, "y": 1.0, "z": 1.0}'
```

### Phi Scaling
Analyzes golden ratio scaling patterns in natural phenomena.

```bash
python3 models/phi_scaling.py '{"scale": 2.0, "dimension": 3}'
```

## Configuration

Edit `config.toml` to customize:

- **API Keys**: Store external API credentials
- **Default Model**: Set a preferred default model
- **UI Theme**: Choose interface appearance

## Requirements

- Rust (2021 edition or later)
- Python 3.6+
- Terminal with Unicode support

## Dependencies

### Rust
- `ratatui`: Terminal UI framework
- `crossterm`: Cross-platform terminal manipulation
- `serde` & `toml`: Configuration serialization
- `tokio`: Async runtime
- `clap`: Command line parsing

### Python
- Standard library only (no external dependencies for included models)

## License

MIT License - see LICENSE file for details.
