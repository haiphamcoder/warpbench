use crate::{
    client::HttpClient, metrics::MetricsCollector, Config, ErrorType,
    Result, Stats,
};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Barrier;
use tokio::time::{sleep, Duration, Instant as TokioInstant};
use tracing::{debug, trace};

pub struct WorkerThread {
    id: usize,
    connections: usize,
    config: Config,
    stats: Arc<Stats>,
    metrics_collector: Arc<MetricsCollector>,
    script_path: Option<String>,
    start_barrier: Arc<Barrier>,
    stop_barrier: Arc<Barrier>,
}

impl WorkerThread {
    pub fn new(
        id: usize,
        connections: usize,
        config: Config,
        stats: Arc<Stats>,
        metrics_collector: Arc<MetricsCollector>,
        script_path: Option<String>,
        start_barrier: Arc<Barrier>,
        stop_barrier: Arc<Barrier>,
    ) -> Self {
        Self {
            id,
            connections,
            config,
            stats,
            metrics_collector,
            script_path,
            start_barrier,
            stop_barrier,
        }
    }

    pub async fn run(self) -> Result<()> {
        debug!("Worker {} starting with {} connections", self.id, self.connections);

        // Scripting temporarily disabled due to Send/Sync issues with Rhai engine
        if let Some(_script_path) = &self.script_path {
            debug!("Scripting temporarily disabled due to thread safety constraints");
        }

        // Create HTTP clients for this worker
        let mut clients = Vec::with_capacity(self.connections);
        for _ in 0..self.connections {
            clients.push(HttpClient::new(&self.config)?);
        }

        // Wait for synchronized start
        self.start_barrier.wait().await;
        debug!("Worker {} started", self.id);

        let start_time = TokioInstant::now();
        let mut client_index = 0;

        // Rate limiting setup
        let rate_limiter = if let Some(rate_limit) = self.config.rate_limit {
            let interval_ns = 1_000_000_000u64 / (rate_limit / self.config.threads as u64);
            Some(Duration::from_nanos(interval_ns))
        } else {
            None
        };

        let mut last_request_time = TokioInstant::now();

        loop {
            // Check if we should stop
            if start_time.elapsed() >= self.config.duration {
                break;
            }

            // Rate limiting
            if let Some(interval) = rate_limiter {
                let elapsed = last_request_time.elapsed();
                if elapsed < interval {
                    sleep(interval - elapsed).await;
                }
                last_request_time = TokioInstant::now();
            }

            // Select client for this request (round-robin)
            let client = &clients[client_index];
            client_index = (client_index + 1) % clients.len();

            // Execute request
            self.execute_request(client).await;
        }

        // Wait for stop signal
        self.stop_barrier.wait().await;
        debug!("Worker {} stopped", self.id);

        Ok(())
    }

    async fn execute_request(&self, client: &HttpClient) {
        let request_start = Instant::now();
        
        // Record request attempt
        self.stats.record_request();

        // Prepare request (scripting disabled)
        let request = None;

        // Execute HTTP request
        match client.execute_request(request).await {
            Ok(response) => {
                let latency = request_start.elapsed();
                
                // Record success metrics
                self.stats.record_success(response.body_size);
                self.metrics_collector.record_latency(latency);

                // Process response with script if available (disabled)
                // TODO: Implement thread-safe scripting

                trace!("Request completed in {:?}", latency);
            }
            Err(e) => {
                let error_type = match e {
                    crate::Error::Timeout => ErrorType::Timeout,
                    crate::Error::Connection(_) => ErrorType::Connect,
                    crate::Error::Io(_) => ErrorType::Read,
                    _ => ErrorType::Read,
                };

                self.stats.record_failure(error_type);
                debug!("Request failed: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[tokio::test]
    async fn test_worker_creation() {
        let config = Config {
            url: Url::parse("http://httpbin.org/get").unwrap(),
            duration: Duration::from_millis(100),
            connections: 1,
            threads: 1,
            ..Default::default()
        };

        let stats = Arc::new(Stats::new());
        let metrics_collector = Arc::new(MetricsCollector::new());
        let start_barrier = Arc::new(Barrier::new(2));
        let stop_barrier = Arc::new(Barrier::new(2));

        let worker = WorkerThread::new(
            0,
            1,
            config,
            stats,
            metrics_collector,
            None,
            start_barrier,
            stop_barrier,
        );

        // Just test creation, not execution
        assert_eq!(worker.id, 0);
        assert_eq!(worker.connections, 1);
    }
}
