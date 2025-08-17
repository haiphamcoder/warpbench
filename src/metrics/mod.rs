use crate::Stats;
use std::time::Duration;

pub struct MetricsReporter;

impl MetricsReporter {
    pub fn new() -> Self {
        Self
    }

    pub fn report(&self, stats: &Stats, duration: Duration) {
        let duration_secs = duration.as_secs_f64();
        let requests_per_sec = stats.requests as f64 / duration_secs;
        let bytes_per_sec = stats.bytes_read as f64 / duration_secs;

        // Calculate latency statistics
        let mut latencies = stats.latency_us.clone();
        latencies.sort_unstable();

        let avg_latency = latencies.iter().sum::<u64>() as f64 / latencies.len() as f64;

        let p50 = percentile(&latencies, 50);
        let p75 = percentile(&latencies, 75);
        let p90 = percentile(&latencies, 90);
        let p99 = percentile(&latencies, 99);

        // Print report
        println!("\nTest completed in {:.2} seconds", duration_secs);
        println!("  Threads:\t\t{}", num_cpus::get());
        println!("  Requests:\t\t{}", stats.requests);
        println!("  Success:\t\t{}", stats.success);
        println!("  Failures:\t\t{}", stats.failures);
        println!("  Requests/sec:\t\t{:.2}", requests_per_sec);
        println!("  Transfer/sec:\t\t{}", format_bytes(bytes_per_sec as u64));
        println!("\nLatency:");
        println!("  Average:\t\t{:.2} ms", avg_latency / 1000.0);
        println!("  P50:\t\t\t{:.2} ms", p50 as f64 / 1000.0);
        println!("  P75:\t\t\t{:.2} ms", p75 as f64 / 1000.0);
        println!("  P90:\t\t\t{:.2} ms", p90 as f64 / 1000.0);
        println!("  P99:\t\t\t{:.2} ms", p99 as f64 / 1000.0);
    }
}

fn percentile(data: &[u64], p: u64) -> u64 {
    if data.is_empty() {
        return 0;
    }
    let k = (data.len() as f64 * (p as f64 / 100.0)) as usize;
    data[k.min(data.len() - 1)]
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut bytes = bytes as f64;
    let mut unit = 0;

    while bytes >= 1024.0 && unit < UNITS.len() - 1 {
        bytes /= 1024.0;
        unit += 1;
    }

    format!("{:.2} {}", bytes, UNITS[unit])
}
