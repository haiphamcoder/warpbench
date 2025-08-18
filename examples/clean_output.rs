use std::time::Duration;
use url::Url;
use warpbench::{load_generator::LoadGenerator, metrics::MetricsReporter, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example demonstrates clean output (no logs)
    // Similar to the default CLI behavior
    
    let config = Config {
        url: Url::parse("https://httpbin.org/get")?,
        duration: Duration::from_secs(5),
        connections: 10,
        threads: 2,
        timeout: Duration::from_secs(5),
        headers: vec![
            ("User-Agent".to_string(), "WarpBench-Clean/0.1.0".to_string()),
        ],
        script_path: None,
        show_latency: true,
        rate_limit: None,
        method: "GET".to_string(),
        body: None,
        verbose: false,
        quiet: true, // This ensures clean output
    };

    // No tracing initialization - clean output like CLI
    
    let load_generator = LoadGenerator::new(config)?;
    let result = load_generator.run().await?;

    let reporter = MetricsReporter::new();
    reporter.report(&result);

    Ok(())
}
