# Contributing to hatoba

Thank you for your interest in contributing!

## Getting Started

### Prerequisites

- Rust 1.85.0 or later

### Setup

```bash
git clone https://github.com/iidax/hatoba
cd hatoba
cargo build
```

## Before Opening an Issue

- Search [existing issues](https://github.com/iidax/hatoba/issues) to avoid duplicates.
- Use the appropriate issue template:
  - **Bug** — something is broken or behaving unexpectedly
  - **Enhancement** — new feature, improvement, or refactoring

## Submitting a Pull Request

1. Fork the repository and create a branch from `main`.
2. Make your changes.
3. Ensure all checks pass:

```bash
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

4. Open a PR against `main`. Fill in the pull request template.

## Code Style

- Format with `cargo fmt` before committing.
- Fix all `cargo clippy` warnings — CI treats them as errors.
- Keep commits focused; one logical change per PR.

## License

By contributing, you agree that your code will be licensed under the [MIT License](../LICENSE).
