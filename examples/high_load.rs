use std::time::Duration;
use url::Url;
use warpbench::{load_generator::LoadGenerator, metrics::MetricsReporter, Config};
use tracing::{info, warn, Level};
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

    // High load configuration - be careful with this!
    let config = Config {
        url: Url::parse("https://httpbin.org/get")?,
        duration: Duration::from_secs(30),
        connections: 200,
        threads: num_cpus::get(),
        timeout: Duration::from_secs(2),
        headers: vec![
            ("User-Agent".to_string(), "WarpBench-HighLoad/0.1.0".to_string()),
        ],
        script_path: None,
        show_latency: true,
        rate_limit: Some(1000), // 1000 requests per second
        method: "GET".to_string(),
        body: None,
        verbose: false,
        quiet: false,
    };

    warn!("WARNING: This is a high load test!");
    info!("Target: {}", config.url);
    info!("Connections: {}", config.connections);
    info!("Threads: {}", config.threads);
    info!("Rate limit: {:?} RPS", config.rate_limit);
    info!("Duration: {:?}", config.duration);
    
    warn!("Make sure you have permission to test the target server!");
    warn!("Press Ctrl+C to cancel or wait 5 seconds to continue...");
    
    tokio::time::sleep(Duration::from_secs(5)).await;
    
    info!("Starting high load test...");

    let load_generator = LoadGenerator::new(config)?;
    let result = load_generator.run().await?;

    let reporter = MetricsReporter::new();
    reporter.report(&result);

    info!("High load test completed successfully!");

    Ok(())
}
