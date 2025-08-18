use crate::{Config, Result};
use clap::Parser;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// URL to benchmark
    #[arg(required = true)]
    url: String,

    /// Number of threads to use
    #[arg(short = 't', long, default_value_t = num_cpus::get())]
    threads: usize,

    /// Number of connections to keep open
    #[arg(short = 'c', long, default_value_t = 10)]
    connections: usize,

    /// Duration of the test (e.g. "10s", "2m", "1h")
    #[arg(short = 'd', long, default_value = "10s")]
    duration: String,

    /// Request timeout
    #[arg(long, default_value = "2s")]
    timeout: String,

    /// Add header to request (can be used multiple times)
    #[arg(short = 'H', long)]
    header: Vec<String>,

    /// Rhai script file
    #[arg(short = 's', long)]
    script: Option<String>,

    /// Print latency statistics
    #[arg(long)]
    latency: bool,

    /// HTTP method to use
    #[arg(short = 'X', long, default_value = "GET")]
    method: String,

    /// Request body
    #[arg(long)]
    body: Option<String>,

    /// Rate limit (requests per second)
    #[arg(long)]
    rate_limit: Option<u64>,

    /// Enable verbose logging
    #[arg(short = 'v', long)]
    verbose: bool,

    /// Quiet mode - suppress info logs
    #[arg(short = 'q', long)]
    quiet: bool,
}

impl Cli {
    pub fn parse_args() -> Result<Config> {
        let cli = Self::parse();

        let duration = parse_duration(&cli.duration)?;
        let timeout = parse_duration(&cli.timeout)?;
        let url = url::Url::parse(&cli.url)?;

        let headers = cli
            .header
            .into_iter()
            .filter_map(|h| {
                let parts: Vec<&str> = h.splitn(2, ':').collect();
                if parts.len() == 2 {
                    Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
                } else {
                    None
                }
            })
            .collect();

        Ok(Config {
            url,
            duration,
            connections: cli.connections,
            threads: cli.threads,
            timeout,
            headers,
            script_path: cli.script,
            show_latency: cli.latency,
            rate_limit: cli.rate_limit,
            method: cli.method,
            body: cli.body,
            verbose: cli.verbose,
            quiet: cli.quiet,
        })
    }
}

fn parse_duration(s: &str) -> Result<Duration> {
    humantime::parse_duration(s)
        .map_err(|e| crate::Error::Config(format!("Invalid duration: {}", e)))
}
