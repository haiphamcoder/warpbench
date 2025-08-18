pub mod cli;
pub mod client;
pub mod load_generator;
pub mod metrics;
pub mod scripting;
pub mod worker;

use hdrhistogram::Histogram;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use url::Url;

#[derive(Debug, Clone)]
pub struct Config {
    pub url: Url,
    pub duration: Duration,
    pub connections: usize,
    pub threads: usize,
    pub timeout: Duration,
    pub headers: Vec<(String, String)>,
    pub script_path: Option<String>,
    pub show_latency: bool,
    pub rate_limit: Option<u64>, // requests per second
    pub method: String,
    pub body: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            url: Url::parse("http://localhost").unwrap(),
            duration: Duration::from_secs(10),
            connections: 10,
            threads: num_cpus::get(),
            timeout: Duration::from_secs(2),
            headers: Vec::new(),
            script_path: None,
            show_latency: false,
            rate_limit: None,
            method: "GET".to_string(),
            body: None,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub requests: Arc<AtomicU64>,
    pub success: Arc<AtomicU64>,
    pub failures: Arc<AtomicU64>,
    pub bytes_read: Arc<AtomicU64>,
    pub connect_errors: Arc<AtomicU64>,
    pub read_errors: Arc<AtomicU64>,
    pub write_errors: Arc<AtomicU64>,
    pub timeout_errors: Arc<AtomicU64>,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            requests: Arc::new(AtomicU64::new(0)),
            success: Arc::new(AtomicU64::new(0)),
            failures: Arc::new(AtomicU64::new(0)),
            bytes_read: Arc::new(AtomicU64::new(0)),
            connect_errors: Arc::new(AtomicU64::new(0)),
            read_errors: Arc::new(AtomicU64::new(0)),
            write_errors: Arc::new(AtomicU64::new(0)),
            timeout_errors: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn record_request(&self) {
        self.requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_success(&self, bytes: u64) {
        self.success.fetch_add(1, Ordering::Relaxed);
        self.bytes_read.fetch_add(bytes, Ordering::Relaxed);
    }

    pub fn record_failure(&self, error_type: ErrorType) {
        self.failures.fetch_add(1, Ordering::Relaxed);
        match error_type {
            ErrorType::Connect => self.connect_errors.fetch_add(1, Ordering::Relaxed),
            ErrorType::Read => self.read_errors.fetch_add(1, Ordering::Relaxed),
            ErrorType::Write => self.write_errors.fetch_add(1, Ordering::Relaxed),
            ErrorType::Timeout => self.timeout_errors.fetch_add(1, Ordering::Relaxed),
        };
    }

    pub fn get_requests(&self) -> u64 {
        self.requests.load(Ordering::Relaxed)
    }

    pub fn get_success(&self) -> u64 {
        self.success.load(Ordering::Relaxed)
    }

    pub fn get_failures(&self) -> u64 {
        self.failures.load(Ordering::Relaxed)
    }

    pub fn get_bytes_read(&self) -> u64 {
        self.bytes_read.load(Ordering::Relaxed)
    }
}

#[derive(Debug, Clone)]
pub struct LatencyStats {
    pub histogram: Histogram<u64>,
    pub min: Duration,
    pub max: Duration,
    pub mean: Duration,
    pub p50: Duration,
    pub p75: Duration,
    pub p90: Duration,
    pub p95: Duration,
    pub p99: Duration,
    pub p999: Duration,
}

impl Default for LatencyStats {
    fn default() -> Self {
        Self {
            histogram: Histogram::new(3).unwrap(),
            min: Duration::ZERO,
            max: Duration::ZERO,
            mean: Duration::ZERO,
            p50: Duration::ZERO,
            p75: Duration::ZERO,
            p90: Duration::ZERO,
            p95: Duration::ZERO,
            p99: Duration::ZERO,
            p999: Duration::ZERO,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub duration: Duration,
    pub stats: BenchmarkStats,
    pub latency: LatencyStats,
    pub config: BenchmarkConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkStats {
    pub requests: u64,
    pub success: u64,
    pub failures: u64,
    pub bytes_read: u64,
    pub connect_errors: u64,
    pub read_errors: u64,
    pub write_errors: u64,
    pub timeout_errors: u64,
    pub requests_per_sec: f64,
    pub bytes_per_sec: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub url: String,
    pub duration: Duration,
    pub connections: usize,
    pub threads: usize,
    pub timeout: Duration,
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    Connect,
    Read,
    Write,
    Timeout,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Hyper error: {0}")]
    Hyper(#[from] hyper::Error),

    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Script error: {0}")]
    Script(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("HTTP body error: {0}")]
    Body(String),

    #[error("Timeout error")]
    Timeout,

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("TLS error: {0}")]
    Tls(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
