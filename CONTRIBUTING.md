# Contributing to Glacex

Thanks for your interest in contributing to Glacex! This document outlines the process and guidelines.

## Getting Started

1. Fork the repository and clone your fork.
2. Install the Rust toolchain (stable, edition 2024, minimum `rustc 1.85`).
3. Run `cargo check` to verify the build compiles.

## Development Workflow

### Code Style

- Run `cargo fmt` and `cargo fmt --all` before committing.
- Run `cargo clippy --all-targets --all-features -- -D warnings` and fix any warnings.
- Follow standard Rust naming conventions and idioms.

### Building & Testing

```sh
cargo fmt && cargo fmt --all
cargo check --workspace
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

All four must pass before submitting a PR.

### Commit Messages

- Use clear, descriptive commit messages.
- Prefix with a category when appropriate: `fix:`, `feat:`, `refactor:`, `docs:`, `chore:`.

## Submitting a Pull Request

1. Create a feature branch from `main`.
2. Make your changes following the guidelines above.
3. Ensure CI checks pass (fmt, clippy, tests).
4. Open a pull request with a clear description of what changed and why.
5. Reference any related issues.

### PR Guidelines

- Keep PRs focused — one logical change per PR.
- All new public APIs must include rustdoc comments.
- Add examples or tests for new functionality when practical.
- Breaking changes should be clearly noted in the PR description.

## Reporting Issues

- Use GitHub Issues for bug reports and feature requests.
- Include reproduction steps, expected vs. actual behavior, and your environment (OS, Rust version, GPU).

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
