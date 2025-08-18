# WarpBench

**WarpBench** is a modern HTTP benchmarking tool written in Rust, designed for high performance and flexibility. It allows developers to test the performance of web servers and APIs with customizable scenarios and detailed metrics.

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/haiphamcoder/warpbench/blob/main/LICENSE)
[![GitHub Repo](https://img.shields.io/badge/GitHub-Repo-green.svg)](https://github.com/haiphamcoder/warpbench)

## Features

- **High Performance**: Built with Rust and Tokio for asynchronous, high-concurrency benchmarking.
- **Multi-threaded**: Efficient load generation with configurable thread and connection pools.
- **HTTP/HTTPS Support**: Full support for both HTTP and HTTPS with TLS.
- **Detailed Metrics**: Collect and analyze metrics like latency percentiles, throughput, and error rates.
- **Rate Limiting**: Control request rate to avoid overwhelming target servers.
- **Flexible Configuration**: Customizable headers, methods, request bodies, and timeouts.
- **wrk-compatible**: Familiar command-line interface and output format.
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
warpbench http://example.com
```

### Command Line Options

- `-t, --threads <THREADS>`: Number of threads to use (default: number of CPU cores)
- `-c, --connections <CONNECTIONS>`: Number of connections to keep open (default: 10)
- `-d, --duration <DURATION>`: Duration of the test, e.g. "10s", "2m", "1h" (default: 10s)
- `--timeout <TIMEOUT>`: Request timeout (default: 2s)
- `-H, --header <HEADER>`: Add header to request (can be used multiple times)
- `-X, --method <METHOD>`: HTTP method to use (default: GET)
- `--body <BODY>`: Request body for POST/PUT requests
- `--rate-limit <RATE_LIMIT>`: Rate limit in requests per second
- `-s, --script <SCRIPT>`: Rhai script file (currently disabled)
- `--latency`: Print detailed latency statistics
- `-h, --help`: Print help information
- `-V, --version`: Print version information

### Examples

Basic GET request:

```bash
warpbench https://httpbin.org/get -t 4 -c 50 -d 30s
```

POST request with JSON body:

```bash
warpbench https://httpbin.org/post \
  -X POST \
  -H "Content-Type: application/json" \
  --body '{"message": "Hello, World!"}' \
  -t 2 -c 10 -d 10s
```

High-load test with rate limiting:

```bash
warpbench http://example.com \
  --threads 8 \
  --connections 200 \
  --duration 60s \
  --rate-limit 1000 \
  --latency
```

Custom headers and timeout:

```bash
warpbench https://api.example.com/endpoint \
  -H "Authorization: Bearer token123" \
  -H "User-Agent: WarpBench/0.1.0" \
  --timeout 10s \
  --latency
```

## Output Format

WarpBench provides detailed benchmark results in a format similar to wrk:

```text
============================================================
Benchmark Results
============================================================
Running 30.00s test @ https://httpbin.org/get
  4 threads and 50 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   245.67ms  45.23ms  1.20s    68.00%
    Req/Sec   52.30k    8.45k    65.00k   86.54%
  1500000 requests in 30.00s, 1.23GB read
Requests/sec: 50000.25
Transfer/sec: 41.95MB

Latency Distribution (with --latency flag)
   50%  234.56ms
   75%  267.89ms
   90%  312.45ms
   95%  378.92ms
   99%  567.34ms
  99.9% 891.23ms
============================================================
```

## Scripting Support (Future)

WarpBench is designed to support Rhai scripting for advanced benchmarking scenarios. This feature is currently disabled due to thread safety considerations but will be available in future releases.

Planned scripting capabilities:

- Custom request generation
- Dynamic headers and bodies  
- Response processing and validation
- Custom metrics collection

Example future script:

```rust
fn request() {
    let req = http_get("/api/endpoint");
    set_header(req, "User-Agent", "WarpBench");
    req
}

fn response(res) {
    if get_status(res) == 200 {
        log_metric("success_count", 1.0);
    }
}
```

## Performance Tips

- **Threads**: Start with the number of CPU cores, adjust based on your workload
- **Connections**: Balance between throughput and resource usage (typically 10-200)
- **Rate Limiting**: Use `--rate-limit` to avoid overwhelming the target server
- **Timeout**: Set reasonable timeouts based on expected response times
- **Duration**: Longer tests (30s+) provide more stable results

## Troubleshooting

### Common Issues

**High timeout rates:**

- Increase `--timeout` value
- Reduce number of connections with `-c`
- Check network connectivity

**Low throughput:**

- Increase number of threads with `-t`
- Increase connections with `-c`  
- Check if rate limiting is too restrictive

**Connection errors:**

- Verify the target URL is accessible
- Check firewall settings
- Ensure the server can handle the connection load

### Getting Help

If you encounter issues:

1. Run with `RUST_LOG=debug` for detailed logging
2. Try with minimal settings first: `warpbench <url> -t 1 -c 1 -d 5s`
3. Check the [Issues](https://github.com/haiphamcoder/warpbench/issues) page
4. Create a new issue with your command and error output

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
