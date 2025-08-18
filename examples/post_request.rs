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

    // Configuration for POST requests with JSON body
    let config = Config {
        url: Url::parse("https://httpbin.org/post")?,
        duration: Duration::from_secs(5),
        connections: 20,
        threads: 2,
        timeout: Duration::from_secs(10),
        headers: vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("User-Agent".to_string(), "WarpBench/0.1.0".to_string()),
        ],
        script_path: None,
        show_latency: true,
        rate_limit: None,
        method: "POST".to_string(),
        body: Some(r#"{"message": "Hello from WarpBench!", "timestamp": "2024-01-01T00:00:00Z"}"#.to_string()),
        verbose: false,
        quiet: false,
    };

    info!("Starting POST request example to httpbin.org");
    info!("Method: {}", config.method);
    info!("Body: {:?}", config.body);

    let load_generator = LoadGenerator::new(config)?;
    let result = load_generator.run().await?;

    let reporter = MetricsReporter::new();
    reporter.report(&result);

    info!("POST request example completed successfully!");
    Ok(())
}
