# WarpBench

**WarpBench** is a modern HTTP benchmarking tool written in Rust, designed for high performance and flexibility. It allows developers to test the performance of web servers and APIs with customizable scenarios and detailed metrics.

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/haiphamcoder/warpbench/blob/main/LICENSE)
[![GitHub Repo](https://img.shields.io/badge/GitHub-Repo-green.svg)](https://github.com/haiphamcoder/warpbench)

## Features

- **High Performance**: Built with Rust and Tokio for asynchronous, high-concurrency benchmarking.
- **Customizable Scenarios**: Use Rhai scripting to define complex request patterns and workflows.
- **Detailed Metrics**: Collect and analyze metrics like latency, throughput, and error rates.
- **Command-Line Interface**: Easy-to-use CLI for quick setup and execution.
- **Cross-Platform**: Runs on Linux, macOS, and Windows.

## Installation

### From Source

To build WarpBench from source, you'll need Rust and Cargo installed. Follow these steps:

```bash
git clone https://github.com/haiphamcoder/warpbench.git
cd warpbench
cargo build --release
```

The binary will be available at `target/release/warpbench`.

### Pre-built Binaries

Pre-built binaries will be available for download from the [GitHub Releases](https://github.com/haiphamcoder/warpbench/releases) page (coming soon).

## Usage

Run a simple benchmark against a target URL with default settings:

```bash
warpbench run --url http://example.com
```

### Options

- `--threads`: Number of concurrent threads (default: number of CPU cores).
- `--duration`: Duration of the benchmark in seconds (default: 30).
- `--connections`: Total number of connections to open (default: 100).
- `--script`: Path to a Rhai script for custom request scenarios.

Example with custom options:

```bash
warpbench run --url http://example.com --threads 8 --duration 60 --connections 200
```

### Scripting

WarpBench supports Rhai scripting for advanced benchmarking scenarios. Create a script file (e.g., `scenario.rhai`) and pass it with the `--script` option:

```bash
warpbench run --url http://example.com --script scenario.rhai
```

Example Rhai script:

```rust
fn request() {
    let res = http_get("/api endpoint");
    if res.status == 200 {
        print("Success");
    } else {
        print("Failed");
    }
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request or open an Issue on the [GitHub repository](https://github.com/haiphamcoder/warpbench).

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

- **Author**: Hai Pham Ngoc
- **Email**: <ngochai285nd@gmail.com>
- **GitHub**: [haiphamcoder](https://github.com/haiphamcoder)

---

*WarpBench - Pushing the limits of HTTP benchmarking.*
