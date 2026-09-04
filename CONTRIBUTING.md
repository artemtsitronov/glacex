# Contributing to Glacex

Thanks for contributing to Glacex!

## Getting Started

1. Fork and clone the repository.
2. Ensure you have Rust 1.85+ installed (Rust 2024 edition).
3. Verify that `cargo check` compiles cleanly.

## Development Workflow

### Code Style

- Run `cargo fmt --all` before committing.
- Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any warnings.
- Follow standard Rust naming conventions and idioms.

### Building & Testing

```sh
cargo fmt --all
cargo check --workspace
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

All commands must pass before submitting a pull request.

### Commit Messages

Use clear, descriptive commit messages with standard prefixes: `fix:`, `feat:`, `refactor:`, `docs:`, `chore:`.

## Pull Requests

1. Create a feature branch from `main`.
2. Keep PRs focused on one logical change.
3. Include rustdoc comments on all new public APIs.
4. Add examples or tests for new functionality where practical.
5. Note any breaking API changes in the PR description.

## Reporting Issues

Open an issue on GitHub with reproduction steps, expected vs actual behavior, and your environment (OS, GPU, and Rust version).

## License

By contributing, you agree that your contributions are licensed under the MIT License.
