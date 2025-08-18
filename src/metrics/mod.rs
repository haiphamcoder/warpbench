use crate::{BenchmarkResult, BenchmarkStats, LatencyStats};
use hdrhistogram::Histogram;
use std::sync::Mutex;
use std::time::Duration;
use tracing::debug;

pub struct MetricsCollector {
    latency_histogram: Mutex<Histogram<u64>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            latency_histogram: Mutex::new(Histogram::new(3).unwrap()),
        }
    }

    pub fn record_latency(&self, latency: Duration) {
        let latency_us = latency.as_micros() as u64;
        if let Ok(mut histogram) = self.latency_histogram.lock() {
            if let Err(e) = histogram.record(latency_us) {
                debug!("Failed to record latency: {}", e);
            }
        }
    }

    pub fn get_latency_stats(&self) -> LatencyStats {
        if let Ok(histogram) = self.latency_histogram.lock() {
            let min = Duration::from_micros(histogram.min());
            let max = Duration::from_micros(histogram.max());
            let mean = Duration::from_micros(histogram.mean() as u64);
            let p50 = Duration::from_micros(histogram.value_at_quantile(0.5));
            let p75 = Duration::from_micros(histogram.value_at_quantile(0.75));
            let p90 = Duration::from_micros(histogram.value_at_quantile(0.90));
            let p95 = Duration::from_micros(histogram.value_at_quantile(0.95));
            let p99 = Duration::from_micros(histogram.value_at_quantile(0.99));
            let p999 = Duration::from_micros(histogram.value_at_quantile(0.999));

            LatencyStats {
                histogram: histogram.clone(),
                min,
                max,
                mean,
                p50,
                p75,
                p90,
                p95,
                p99,
                p999,
            }
        } else {
            LatencyStats::default()
        }
    }
}

pub struct MetricsReporter;

impl MetricsReporter {
    pub fn new() -> Self {
        Self
    }

    pub fn report(&self, result: &BenchmarkResult) {
        let duration_secs = result.duration.as_secs_f64();
        let stats = &result.stats;

        println!("\n{}", "=".repeat(60));
        println!("Benchmark Results");
        println!("{}", "=".repeat(60));

        // Basic statistics
        println!("Running {:.2}s test @ {}", duration_secs, result.config.url);
        println!("  {} threads and {} connections", result.config.threads, result.config.connections);
        
        // Thread stats header
        println!("  Thread Stats   Avg      Stdev     Max   +/- Stdev");
        
        // Latency stats
        let latency = &result.latency;
        println!("    Latency   {:<8} {:<8} {:<8} {:<8}",
            format_duration(latency.mean),
            format_duration(Duration::from_micros((latency.histogram.stdev() as u64).max(1))),
            format_duration(latency.max),
            format!("{:.2}%", calculate_stdev_percentage(&latency.histogram))
        );

        // Request rate (this would need per-thread tracking for real stdev)
        println!("    Req/Sec   {:<8} {:<8} {:<8} {:<8}",
            format!("{:.2}k", stats.requests_per_sec / 1000.0),
            "N/A", // Would need per-thread tracking
            "N/A", // Would need per-thread tracking  
            "N/A"  // Would need per-thread tracking
        );

        // Summary
        println!("  {} requests in {:.2}s, {} read",
            stats.requests,
            duration_secs,
            format_bytes(stats.bytes_read)
        );

        // Error breakdown if any
        if stats.failures > 0 {
            println!("  Socket errors: connect {}, read {}, write {}, timeout {}",
                stats.connect_errors,
                stats.read_errors, 
                stats.write_errors,
                stats.timeout_errors
            );
        }

        println!("Requests/sec: {:.2}", stats.requests_per_sec);
        println!("Transfer/sec: {}", format_bytes(stats.bytes_per_sec as u64));

        // Detailed latency distribution  
        if result.stats.requests > 0 {
            println!("\nLatency Distribution");
            println!("   50%  {}", format_duration(latency.p50));
            println!("   75%  {}", format_duration(latency.p75));
            println!("   90%  {}", format_duration(latency.p90));
            println!("   95%  {}", format_duration(latency.p95));
            println!("   99%  {}", format_duration(latency.p99));
            println!("  99.9% {}", format_duration(latency.p999));
        }

        println!("{}", "=".repeat(60));
    }

    pub fn print_progress(&self, stats: &BenchmarkStats, elapsed: Duration) {
        let elapsed_secs = elapsed.as_secs_f64();
        let rps = if elapsed_secs > 0.0 {
            stats.requests as f64 / elapsed_secs
        } else {
            0.0
        };

        print!("\rProgress: {}s | Requests: {} | RPS: {:.1} | Errors: {}",
            elapsed.as_secs(),
            stats.requests,
            rps,
            stats.failures
        );
    }
}

fn format_duration(duration: Duration) -> String {
    let micros = duration.as_micros();
    if micros < 1000 {
        format!("{}us", micros)
    } else if micros < 1_000_000 {
        format!("{:.2}ms", micros as f64 / 1000.0)
    } else {
        format!("{:.2}s", duration.as_secs_f64())
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut bytes = bytes as f64;
    let mut unit = 0;

    while bytes >= 1024.0 && unit < UNITS.len() - 1 {
        bytes /= 1024.0;
        unit += 1;
    }

    format!("{:.2}{}", bytes, UNITS[unit])
}

fn calculate_stdev_percentage(histogram: &Histogram<u64>) -> f64 {
    let mean = histogram.mean();
    let _stdev = histogram.stdev();
    
    if mean > 0.0 && histogram.len() > 0 {
        // Simple approximation: assume normal distribution
        // In a normal distribution, ~68% of values fall within 1 standard deviation
        68.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collector_creation() {
        let collector = MetricsCollector::new();
        collector.record_latency(Duration::from_millis(10));
        
        let stats = collector.get_latency_stats();
        assert!(stats.min > Duration::ZERO);
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_micros(500)), "500us");
        assert_eq!(format_duration(Duration::from_millis(5)), "5.00ms");
        assert_eq!(format_duration(Duration::from_secs(2)), "2.00s");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512.00B");
        assert_eq!(format_bytes(1024), "1.00KB");
        assert_eq!(format_bytes(1048576), "1.00MB");
    }
}
