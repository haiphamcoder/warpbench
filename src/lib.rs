pub mod cli;
pub mod client;
pub mod metrics;
pub mod scripting;

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
        }
    }
}

#[derive(Debug, Default)]
pub struct Stats {
    pub requests: u64,
    pub success: u64,
    pub failures: u64,
    pub bytes_read: u64,
    pub latency_us: Vec<u64>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Script error: {0}")]
    Script(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("HTTP body error: {0}")]
    Body(String),
}

pub type Result<T> = std::result::Result<T, Error>;
