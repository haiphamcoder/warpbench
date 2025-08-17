# Advanced Configuration for WarpBench

This guide covers advanced configuration options and features of WarpBench for users who want to customize their benchmarking experience beyond the basics.

## Table of Contents

- [Custom Metrics](#custom-metrics)
- [Advanced Scripting](#advanced-scripting)
- [Performance Tuning](#performance-tuning)
- [Integration with CI/CD](#integration-with-cicd)

## Custom Metrics

WarpBench allows you to define custom metrics to track specific aspects of your benchmark. This can be done through Rhai scripts by logging custom data points.

### Example: Custom Metric for Response Size

```rust
fn request() {
    let res = http_get("/api/endpoint");
    let response_size = res.body.len();
    log_metric("response_size", response_size);
    if res.status == 200 {
        print("Success");
    }
}
```

## Advanced Scripting

Beyond basic request scripting, Rhai in WarpBench supports complex workflows such as:

- **Conditional Logic**: Execute different requests based on previous response data.
- **Loops**: Perform repeated actions, like polling an endpoint until a condition is met.
- **State Management**: Maintain state across requests using global variables.

### Example: Polling with Timeout

```rust
let max_attempts = 10;
let mut attempts = 0;
let mut success = false;

while attempts < max_attempts && !success {
    let res = http_get("/status");
    if res.status == 200 && res.body.contains("ready") {
        print("Service is ready!");
        success = true;
    } else {
        print("Waiting for service...");
        attempts += 1;
        sleep(1000); // Wait 1 second
    }
}

if !success {
    print("Timeout: Service not ready after " + max_attempts + " attempts");
}
```

## Performance Tuning

To get the most out of WarpBench on high-performance systems:

- **Thread Configuration**: Experiment with `--threads` to find the optimal number for your hardware. Too many threads can lead to contention.
- **Connection Limits**: Adjust `--connections` based on your network capacity and the server’s ability to handle concurrent connections.
- **Duration**: Longer durations (`--duration`) can help in identifying performance degradation over time.

### Example: High Load Test

```bash
warpbench run --url http://example.com --threads 16 --connections 1000 --duration 300
```

## Integration with CI/CD

WarpBench can be integrated into Continuous Integration/Continuous Deployment pipelines to automatically benchmark performance on code changes.

### GitHub Actions Example

Add the following to your `.github/workflows/benchmark.yml`:

```yaml
name: Benchmark
on: [push, pull_request]
jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build WarpBench
        run: cargo build --release
      - name: Run Benchmark
        run: ./target/release/warpbench run --url http://example.com --duration 60
```

## Troubleshooting Advanced Issues

- **Script Errors**: Ensure your Rhai scripts are syntactically correct. Use `warpbench script --check <file.rhai>` to validate scripts before running.
- **Performance Bottlenecks**: If WarpBench itself becomes a bottleneck, consider running on a more powerful machine or reducing the scope of the benchmark.

For further assistance, open an [issue](https://github.com/haiphamcoder/warpbench/issues) on GitHub.
