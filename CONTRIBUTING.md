# Contributing to Glacex

Thanks for wanting to help out with Glacex!

## Getting Started

1. Fork the repo and clone it.
2. Make sure you have Rust 1.88 or newer (2024 edition).
3. Run `cargo check` and make sure it builds.

## Development Workflow

### Code Style

- Run `cargo fmt --all` before you commit.
- Run `cargo clippy --all-targets --all-features -- -D warnings` and fix whatever it complains about.
- Just write normal Rust with the usual naming.

### Building & Testing

```sh
cargo fmt --all
cargo check --workspace
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

All of these need to pass before you open a pull request.

### Commit Messages

Write commit messages that actually say what changed. Start them with `fix:`, `feat:`, `refactor:`, `docs:` or `chore:`.

## Pull Requests

1. Make a new branch off `main`.
2. Keep each PR about one thing.
3. Add rustdoc comments to any new public API.
4. If you can, add an example or a test for new stuff.
5. If you break the API, say so in the PR description.

## Reporting Issues

Open an issue on GitHub. Tell us how to reproduce it, what you expected and what actually happened, and what you're running it on (OS, GPU and Rust version).

## License

If you contribute, your code goes under the MIT License like the rest of the project.

## AI Policy

AI tools (ChatGPT, Codex, Claude, ...) may be used as a **tool** and not as a replacement for human-written code (no vibecoding).

1. **You're responsible for it.** You must know what the LLM wrote, review it, and test it.
2. **Disclose it.** Be transparent about it. If you used AI, please tell us what you wrote, and explain it.
3. **No major AI contributions.** DON'T. VIBE. CODE. I (Artem Tsitronov) would prefer to see bad, honest code rather than perfect AI code. You are allowed to use it for repetitive boring/annoying stuff.

I myself use AI as a tool, but strictly as a tool. Please do the same.
