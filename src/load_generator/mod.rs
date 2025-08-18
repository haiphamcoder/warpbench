use crate::{
    metrics::MetricsCollector, scripting::ScriptEngine, worker::WorkerThread,
    BenchmarkResult, BenchmarkStats, Config, Result, Stats,
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Barrier;
use tracing::{info, warn};

pub struct LoadGenerator {
    config: Config,
    stats: Arc<Stats>,
    metrics_collector: Arc<MetricsCollector>,
    script_path: Option<String>,
}

impl LoadGenerator {
    pub fn new(config: Config) -> Result<Self> {
        let stats = Arc::new(Stats::new());
        let metrics_collector = Arc::new(MetricsCollector::new());

        // Validate script if provided
        if let Some(script_path) = &config.script_path {
            let mut engine = ScriptEngine::new();
            engine.load_script(script_path)?;
            engine.validate_script()?;
        }

        Ok(Self {
            script_path: config.script_path.clone(),
            config,
            stats,
            metrics_collector,
        })
    }

    pub async fn run(&self) -> Result<BenchmarkResult> {
        info!("Starting benchmark with {} threads and {} connections", 
              self.config.threads, self.config.connections);
        
        let start_time = Instant::now();
        
        // Create barrier for synchronized start
        let start_barrier = Arc::new(Barrier::new(self.config.threads + 1));
        let stop_barrier = Arc::new(Barrier::new(self.config.threads + 1));
        
        // Calculate connections per thread
        let connections_per_thread = self.config.connections / self.config.threads;
        let remaining_connections = self.config.connections % self.config.threads;
        
        // Spawn worker threads
        let mut handles = Vec::new();
        
        for thread_id in 0..self.config.threads {
            let connections = if thread_id < remaining_connections {
                connections_per_thread + 1
            } else {
                connections_per_thread
            };
            
            let worker = WorkerThread::new(
                thread_id,
                connections,
                self.config.clone(),
                Arc::clone(&self.stats),
                Arc::clone(&self.metrics_collector),
                self.script_path.clone(),
                Arc::clone(&start_barrier),
                Arc::clone(&stop_barrier),
            );
            
            let handle = tokio::spawn(async move {
                worker.run().await
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads to be ready
        start_barrier.wait().await;
        info!("All worker threads ready, starting benchmark");
        
        // Wait for benchmark duration
        tokio::time::sleep(self.config.duration).await;
        
        // Signal stop to all threads
        stop_barrier.wait().await;
        info!("Benchmark duration completed, stopping workers");
        
        // Wait for all threads to complete
        for handle in handles {
            if let Err(e) = handle.await {
                warn!("Worker thread panicked: {}", e);
            }
        }
        
        let actual_duration = start_time.elapsed();
        
        // Collect final metrics
        let benchmark_stats = self.collect_final_stats(actual_duration);
        let latency_stats = self.metrics_collector.get_latency_stats();
        
        info!("Benchmark completed in {:.2}s", actual_duration.as_secs_f64());
        info!("Total requests: {}, Success: {}, Failures: {}", 
              benchmark_stats.requests, benchmark_stats.success, benchmark_stats.failures);
        
        Ok(BenchmarkResult {
            duration: actual_duration,
            stats: benchmark_stats,
            latency: latency_stats,
            config: crate::BenchmarkConfig {
                url: self.config.url.to_string(),
                duration: self.config.duration,
                connections: self.config.connections,
                threads: self.config.threads,
                timeout: self.config.timeout,
            },
        })
    }
    
    fn collect_final_stats(&self, duration: Duration) -> BenchmarkStats {
        let requests = self.stats.get_requests();
        let success = self.stats.get_success();
        let failures = self.stats.get_failures();
        let bytes_read = self.stats.get_bytes_read();
        
        let duration_secs = duration.as_secs_f64();
        let requests_per_sec = requests as f64 / duration_secs;
        let bytes_per_sec = bytes_read as f64 / duration_secs;
        
        BenchmarkStats {
            requests,
            success,
            failures,
            bytes_read,
            connect_errors: self.stats.connect_errors.load(std::sync::atomic::Ordering::Relaxed),
            read_errors: self.stats.read_errors.load(std::sync::atomic::Ordering::Relaxed),
            write_errors: self.stats.write_errors.load(std::sync::atomic::Ordering::Relaxed),
            timeout_errors: self.stats.timeout_errors.load(std::sync::atomic::Ordering::Relaxed),
            requests_per_sec,
            bytes_per_sec,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[tokio::test]
    async fn test_load_generator_creation() {
        let config = Config {
            url: Url::parse("http://httpbin.org/get").unwrap(),
            duration: Duration::from_secs(1),
            connections: 10,
            threads: 2,
            ..Default::default()
        };

        let load_generator = LoadGenerator::new(config);
        assert!(load_generator.is_ok());
    }
}
