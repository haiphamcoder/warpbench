use std::time::Duration;
use url::Url;
use warpbench::{load_generator::LoadGenerator, metrics::MetricsReporter, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Create a basic configuration
    let config = Config {
        url: Url::parse("https://httpbin.org/get")?,
        duration: Duration::from_secs(10),
        connections: 50,
        threads: 4,
        timeout: Duration::from_secs(5),
        headers: vec![
            ("User-Agent".to_string(), "WarpBench/0.1.0".to_string()),
            ("Accept".to_string(), "application/json".to_string()),
        ],
        script_path: None,
        show_latency: true,
        rate_limit: Some(100), // 100 requests per second
        method: "GET".to_string(),
        body: None,
    };

    println!("Starting WarpBench example...");
    println!("Target: {}", config.url);
    println!("Duration: {:?}", config.duration);
    println!("Connections: {}", config.connections);
    println!("Threads: {}", config.threads);

    // Create and run the load generator
    let load_generator = LoadGenerator::new(config)?;
    let result = load_generator.run().await?;

    // Display results
    let reporter = MetricsReporter::new();
    reporter.report(&result);

    println!("\nExample completed successfully!");

    Ok(())
}
