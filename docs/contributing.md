# Contributing to WarpBench

Thank you for your interest in contributing to WarpBench! This guide will help you get started with the contribution process. WarpBench is an open-source HTTP benchmarking tool, and we welcome contributions from the community to improve its features and performance.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Coding Guidelines](#coding-guidelines)

## Code of Conduct

In the interest of fostering an open and welcoming environment, we expect all contributors to be respectful and considerate of others. By participating in this project, you agree to:

- Be respectful of different viewpoints and experiences.
- Gracefully accept constructive criticism.
- Focus on what is best for the community.
- Show empathy towards other community members.

## How Can I Contribute?

There are many ways to contribute to WarpBench, including:

- **Reporting Bugs**: If you find a bug, please open an issue with detailed steps to reproduce it.
- **Suggesting Enhancements**: Have an idea for a new feature or improvement? Open an issue to discuss it.
- **Code Contributions**: You can contribute code to fix bugs or implement new features.
- **Documentation**: Help improve project documentation or add examples.

### Finding Issues to Work On

Look for issues labeled with "good first issue" if you are new to the project. These are typically straightforward tasks that will help you get familiar with the codebase.

## Development Setup

To set up the development environment for WarpBench:

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/haiphamcoder/warpbench.git
   cd warpbench
   ```

2. **Install Rust and Cargo**:
   Ensure you have Rust and Cargo installed. If not, follow the instructions on the [Rust website](https://www.rust-lang.org/tools/install).

3. **Build and Test**:

   ```bash
   cargo build
   cargo test
   ```

## Pull Request Process

1. **Fork the Repository** (if you are not a direct collaborator).
2. **Create a Branch** for your feature or bug fix:

   ```bash
   git checkout -b feature/YourFeatureName
   ```

   or

   ```bash
   git checkout -b bugfix/YourBugFixName
   ```

3. **Make Your Changes** and commit them with meaningful commit messages. Follow the [conventional commits](https://www.conventionalcommits.org/) format if possible:

   ```bash
   git commit -m "feat: add new benchmarking metric"
   ```

4. **Push Your Changes** to your fork or branch:

   ```bash
   git push origin feature/YourFeatureName
   ```

5. **Open a Pull Request** from your branch to the `main` branch of the WarpBench repository. Ensure your PR description clearly explains the purpose and scope of your changes.

6. **Code Review**: Maintainers will review your PR. Address any feedback by making additional commits to the same branch.

7. **Merge**: Once approved, your PR will be merged.

### PR Requirements

- Ensure your code passes all tests (`cargo test`).
- Update documentation if your changes affect usage or installation.
- Follow the coding guidelines outlined below.

## Coding Guidelines

We aim to maintain a high-quality, consistent codebase:

- **Rustfmt**: Format your code with `cargo fmt` before submitting a PR.
- **Clippy**: Run `cargo clippy --all -- -D warnings` to catch potential issues.
- **Documentation**: Document public APIs and complex logic using Rustdoc comments.
- **Tests**: Add tests for new functionality. Ensure existing tests pass.

### Commit Messages

Use descriptive commit messages that explain the "what" and "why" of your changes. Example:

```text
feat(scripting): add support for custom headers in Rhai scripts

This commit introduces the ability to set custom HTTP headers in Rhai scripts, allowing for more realistic benchmarking scenarios.
```

## Community

Join the discussion or ask questions by opening an [issue](https://github.com/haiphamcoder/warpbench/issues) or contacting the maintainers directly.

Thank you for contributing to WarpBench and helping make it better!
