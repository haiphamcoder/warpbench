use std::time::Duration;
use url::Url;
use warpbench::{load_generator::LoadGenerator, metrics::MetricsReporter, Config};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize clean logging for examples
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

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
        verbose: false,
        quiet: false,
    };

    info!("Starting WarpBench basic usage example");
    info!("Target: {}", config.url);
    info!("Duration: {:?}", config.duration);
    info!("Connections: {}", config.connections);
    info!("Threads: {}", config.threads);

    // Create and run the load generator
    let load_generator = LoadGenerator::new(config)?;
    let result = load_generator.run().await?;

    // Display results
    let reporter = MetricsReporter::new();
    reporter.report(&result);

    info!("Basic usage example completed successfully!");

    Ok(())
}
