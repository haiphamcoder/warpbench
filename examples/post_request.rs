use std::time::Duration;
use url::Url;
use warpbench::{load_generator::LoadGenerator, metrics::MetricsReporter, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

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
    };

    println!("Testing POST requests to httpbin.org");
    println!("Method: {}", config.method);
    println!("Body: {:?}", config.body);

    let load_generator = LoadGenerator::new(config)?;
    let result = load_generator.run().await?;

    let reporter = MetricsReporter::new();
    reporter.report(&result);

    Ok(())
}
