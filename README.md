# Mew Programming Language

A cat-themed programming language with a runtime written in Rust. Mew is an interpreted language that provides a fun and friendly programming experience with feline-inspired syntax and error messages.

## Features

- Interactive REPL (Read-Eval-Print Loop) with command history
- File execution support for `.mew` files
- Project initialization and management via CLI
- Auto-update functionality to keep your Mew installation current
- Cat-themed error messages and prompts
- Modern Rust implementation with optimized performance

## Installation

### Manual Installation

1. Ensure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/)
2. Clone this repository:
   ```bash
   git clone https://github.com/mewisme/mew-language.git
   cd mew-language
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

### Automatic Installation

1. For Windows users, run the following command in PowerShell:
    ```powershell
    powershell -c "irm mewis.me/install.ps1 | iex"
    ```

2. For Linux and macOS users
- Linux users — The `unzip` package is required to install Mew. Use `sudo apt install unzip` to install the package.
- Run the following command in your terminal:
    ```bash
    curl -fsSL mewis.me/install.sh | bash
    ```

## Language Documentation

For more information on the language, see the [Language Documentation](docs/SYNTAX.md).

## Usage

### CLI Commands

Mew provides a comprehensive set of CLI commands:

- `mew` - Start the interactive REPL
- `mew run <file>` - Execute a .mew file
- `mew init [name]` - Initialize a new Mew project (creates project structure with mew.toml). Optional name parameter skips the prompt.
- `mew start` - Run the start script defined in mew.toml
- `mew version` - Display the current version
- `mew upgrade` - Check for and install updates

### Running the REPL

Simply run the compiled binary without arguments to start the interactive REPL:

```bash
mew
```

The REPL provides a friendly cat-themed prompt (`🐾 >`) where you can enter Mew code directly.

### Running a Mew File

To execute a `.mew` file:

```bash
mew run path/to/your/file.mew
```

### Examples

Check out the example programs in the [examples](examples) directory:
- `language_features.mew` - Demonstrates core language features
- `objects_examples.mew` - Shows how to work with objects
- `string_examples.mew` - Illustrates string manipulation

## Project Structure

- `src/`
  - `main.rs` - Entry point and CLI interface
  - `lexer.rs` - Tokenization of source code
  - `parser.rs` - Syntax analysis and AST construction
  - `interpreter.rs` - Runtime execution
  - `error.rs` - Error handling
  - `value.rs` - Value representation
  - `lib.rs` - Library exports
  - `cli/` - Command-line interface functionality
    - `commands.rs` - Command definitions
    - `init.rs` - Project initialization
    - `run.rs` - File execution
    - `start.rs` - Project start script execution
    - `upgrade.rs` - Update functionality
    - `version.rs` - Version information
  - `bin/` - Additional binary utilities
- `docs/`
  - `SYNTAX.md` - Language syntax documentation
- `examples/` - Example Mew programs
- `res/`
  - `icon.png` - Application icon
  - `icon.ico` - Application icon (Windows)
  - `icon.icns` - Application icon (macOS)

## Dependencies

- `rustyline` - For REPL functionality
- `logos` - For lexing/tokenization
- `thiserror` - For error handling
- `clap` - For command-line argument parsing
- `reqwest` - For network requests and auto-update functionality
- `semver` - For version management
- `serde` - For serialization/deserialization

## Development

The project uses Rust 2021 edition and includes optimized release settings for maximum performance:

- Maximum optimization level (opt-level = 3)
- Link-time optimization enabled
- Single codegen unit for maximum optimization
- Panic abort for smaller binary size
- Symbol stripping for reduced binary size

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Copyright (C) 2025 MewTheDev

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

Icon based on Microsoft Fluent Emoji. Licensed under MIT.
https://github.com/microsoft/fluentui-emoji

## Author

MewTheDev <mewisadev@gmail.com>
