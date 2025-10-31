# Repository Guidelines

## Project Structure & Module Organization
Archivum is a single binary crate. Core logic lives in `src/`, with high-level orchestration in `main.rs`, GitHub and Gitea clients in `github.rs` and `gitea.rs`, and shell helpers in `git.rs`. CLI subcommands are grouped under `src/commands/`, mirroring the verbs exposed to users (`mirror`, `download`, `upload`, and starred variants). Long-running automation belongs in `actions.rs`. Example configuration defaults sit in `config.toml.example`, while mutable runtime data lands in `mirror/`. Use `scripts/` for cross-platform build helpers; avoid storing tooling binaries outside that directory. Generated artifacts reside in `target/` and should not be checked in.

## Build, Test, and Development Commands
- `cargo check` — fast validation to catch compilation regressions.
- `cargo fmt && cargo clippy -- -D warnings` — enforce formatting and lint cleanliness before opening a PR.
- `cargo test` — run unit and integration tests on the workspace.
- `cargo run -- <command> [-c config.toml]` — execute a CLI flow such as `mirror`, `download`, or `upload`.
- `./scripts/build.sh <target> [true]` — produce release binaries; pass `true` on Linux to prefer musl.

## Coding Style & Naming Conventions
Rust files use rustfmt defaults (4-space indentation, trailing commas, newline at EOF). Functions and variables follow `snake_case`; types and enums use `UpperCamelCase`; constants remain `SCREAMING_SNAKE_CASE`. Keep modules cohesive: config parsing in `config.rs`, network work in the service files, and command wiring inside `commands/`. Favor descriptive error messages via `anyhow`/`thiserror`-style patterns if added; bubble context instead of panicking. Document new public functions with Rustdoc comments when behavior is non-obvious.

## Testing Guidelines
Existing tests live alongside implementation modules under `#[cfg(test)]`. Add new coverage in-place or create files in `tests/` when cross-module behavior is exercised. Name tests with the behavior under scrutiny (e.g., `downloads_missing_repo_returns_error`). When touching network code, prefer injecting stubs or using `tempfile` fixtures to keep tests hermetic. Run `cargo test` locally and note any ignored or flaky cases in the PR description.

## Commit & Pull Request Guidelines
Commits follow Conventional Commit prefixes (`chore:`, `refactor:`, etc.). Group related changes together and keep messages in the imperative mood (“add support for gitea tokens”). For pull requests, provide a concise summary, list validation commands (builds/tests), reference relevant issues, and include screenshots or logs when behavior changes are user-facing. Flag configuration or deployment impacts so reviewers can coordinate rollouts.
