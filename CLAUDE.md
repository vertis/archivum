# Archivum Development Guide

## Build/Run/Test Commands
- Build: `cargo build` or `cargo build --release`
- Run: `cargo run -- [command] [options]`
- Test: `cargo test` or `cargo test [test_name]`
- Lint: `cargo clippy`
- Format: `cargo fmt`

## Code Style Guidelines
- **Edition**: Rust 2021
- **Naming**: Standard Rust conventions (snake_case for functions/variables)
- **Imports**: Group related imports, organize in alphabetical order
- **Error Handling**: Use Result with descriptive error messages
- **Documentation**: Comment complex functions, include examples for public APIs
- **Structure**: Keep modules focused on single responsibilities
- **Command Pattern**: Follow established command pattern for new subcommands
- **Testing**: Write unit tests as submodules with #[test] annotations
- **Git Commits**: Use conventional commits (feat:, fix:, chore:, etc.)

## Dependencies
- Main: clap, duct, reqwest, serde/serde_json/toml
- Dev: tempfile