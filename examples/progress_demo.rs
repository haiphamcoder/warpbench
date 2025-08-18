use std::time::Duration;
use url::Url;
use warpbench::{load_generator::LoadGenerator, metrics::MetricsReporter, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This example demonstrates progress indicators with a reasonable load
    
    let config = Config {
        url: Url::parse("https://httpbin.org/get")?,
        duration: Duration::from_secs(8), // 8 seconds to see progress
        connections: 5,                    // Low connections to avoid timeouts
        threads: 1,                       // Single thread for simplicity
        timeout: Duration::from_secs(10), // Generous timeout
        headers: vec![
            ("User-Agent".to_string(), "WarpBench-Progress-Demo/0.1.0".to_string()),
        ],
        script_path: None,
        show_latency: true,
        rate_limit: None,
        method: "GET".to_string(),
        body: None,
        verbose: false,  // Default mode - shows basic progress
        quiet: false,    // Not quiet - shows progress indicators
    };

    // No tracing initialization - let WarpBench handle it
    
    let load_generator = LoadGenerator::new(config)?;
    let result = load_generator.run().await?;

    let reporter = MetricsReporter::new();
    reporter.report(&result);

    Ok(())
}
