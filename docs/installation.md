# Installation Guide for WarpBench

This guide provides detailed instructions on how to install WarpBench, a modern HTTP benchmarking tool written in Rust.

## Prerequisites

Before installing WarpBench, ensure you have the following:

- **Rust and Cargo**: WarpBench is built with Rust. You need to have Rust and Cargo installed on your system. If you don't have them installed, follow the instructions on the [Rust website](https://www.rust-lang.org/tools/install).
- **Git**: Required to clone the repository if you are building from source.

## Installing from Source

To build WarpBench from source, follow these steps:

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/haiphamcoder/warpbench.git
   cd warpbench
   ```

2. **Build the Project**:

   ```bash
   cargo build --release
   ```

   The binary will be available at `target/release/warpbench`.

3. **Optional - Install to System**:
   If you want to make the `warpbench` command available system-wide, you can copy the binary to a directory in your PATH, for example:

   ```bash
   sudo cp target/release/warpbench /usr/local/bin/
   ```

## Pre-built Binaries

Pre-built binaries for WarpBench will be available for download from the [GitHub Releases](https://github.com/haiphamcoder/warpbench/releases) page. Check back soon for the latest releases.

## Verifying Installation

After installation, verify that WarpBench is correctly installed by running:

```bash
warpbench --version
```

You should see the version of WarpBench printed to the console.

## Troubleshooting

- **Rust Version Issues**: Ensure you are using a compatible version of Rust. WarpBench requires Rust edition 2024. You can update Rust with:

  ```bash
  rustup update
  ```

- **Build Errors**: If you encounter errors during the build process, ensure all dependencies are installed and your Rust toolchain is up to date.

If you face any other issues, please open an [issue](https://github.com/haiphamcoder/warpbench/issues) on GitHub.

## Next Steps

Once installed, refer to the [Usage Guide](./usage.md) for instructions on how to run benchmarks with WarpBench.
