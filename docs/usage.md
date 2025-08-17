# Usage Guide for WarpBench

This guide provides instructions on how to use WarpBench to benchmark HTTP endpoints effectively.

## Table of Contents

- [Basic Usage](#basic-usage)
- [Command Line Options](#command-line-options)
- [Scripting with Rhai](#scripting-with-rhai)
- [Examples](#examples)

## Basic Usage

To run a simple benchmark against a target URL with default settings, use the following command:

```bash
warpbench run --url http://example.com
```

This will initiate a benchmark with default parameters for threads, duration, and connections.

## Command Line Options

WarpBench provides several options to customize your benchmarking:

- `--threads`: Specifies the number of concurrent threads to use. By default, this is set to the number of CPU cores on your system.
- `--duration`: Defines the duration of the benchmark in seconds. The default is 30 seconds.
- `--connections`: Sets the total number of connections to open during the benchmark. The default is 100.
- `--script`: Allows you to specify a path to a Rhai script for custom request scenarios.

Example with custom options:

```bash
warpbench run --url http://example.com --threads 8 --duration 60 --connections 200
```

## Scripting with Rhai

WarpBench supports advanced benchmarking scenarios through Rhai scripting. This allows you to define complex request patterns and logic.

To use a script, create a file (e.g., `scenario.rhai`) with your Rhai code and pass it to WarpBench using the `--script` option:

```bash
warpbench run --url http://example.com --script scenario.rhai
```

### Example Rhai Script

Below is an example of a Rhai script that performs a GET request and checks the response status:

```rust
fn request() {
    let res = http_get("/api/endpoint");
    if res.status == 200 {
        print("Success");
    } else {
        print("Failed");
    }
}
```

## Examples

### Benchmarking a Single Endpoint

```bash
warpbench run --url https://api.example.com/endpoint --threads 4 --duration 30
```

### Using a Custom Script for Complex Scenarios

```bash
warpbench run --url https://api.example.com --script complex_scenario.rhai
```

### Adjusting Connection Load

```bash
warpbench run --url http://example.com --connections 500 --duration 120
```

## Analyzing Results

After the benchmark completes, WarpBench outputs detailed metrics including:

- **Total Requests**: The number of requests made during the benchmark.
- **Average Latency**: The average time taken for each request.
- **Throughput**: The rate of requests per second.
- **Error Rate**: The percentage of requests that resulted in errors.

These metrics help in understanding the performance characteristics of the tested endpoint.

## Next Steps

For more advanced configurations or if you encounter issues, refer to the [Advanced Configuration](./advanced.md) guide or open an [issue](https://github.com/haiphamcoder/warpbench/issues) on GitHub.
