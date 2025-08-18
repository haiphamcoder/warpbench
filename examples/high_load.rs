use std::time::Duration;
use url::Url;
use warpbench::{load_generator::LoadGenerator, metrics::MetricsReporter, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

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
    };

    println!("WARNING: This is a high load test!");
    println!("Target: {}", config.url);
    println!("Connections: {}", config.connections);
    println!("Threads: {}", config.threads);
    println!("Rate limit: {:?} RPS", config.rate_limit);
    println!("Duration: {:?}", config.duration);
    
    println!("\nMake sure you have permission to test the target server!");
    println!("Press Ctrl+C to cancel or wait 5 seconds to continue...");
    
    tokio::time::sleep(Duration::from_secs(5)).await;
    
    println!("Starting high load test...");

    let load_generator = LoadGenerator::new(config)?;
    let result = load_generator.run().await?;

    let reporter = MetricsReporter::new();
    reporter.report(&result);

    println!("\nHigh load test completed!");

    Ok(())
}
