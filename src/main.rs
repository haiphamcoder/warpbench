use anyhow::Result;
use warpbench::{
    cli::Cli,
    load_generator::LoadGenerator,
    metrics::MetricsReporter,
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("WarpBench - HTTP Benchmarking Tool");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Parse CLI arguments
    let config = Cli::parse_args()?;
    
    info!("Starting benchmark with configuration:");
    info!("  URL: {}", config.url);
    info!("  Duration: {:?}", config.duration);
    info!("  Threads: {}", config.threads);
    info!("  Connections: {}", config.connections);
    info!("  Timeout: {:?}", config.timeout);
    
    if let Some(script_path) = &config.script_path {
        info!("  Script: {}", script_path);
    }

    // Create and run load generator
    let load_generator = LoadGenerator::new(config)?;
    let result = load_generator.run().await?;

    // Report results
    let reporter = MetricsReporter::new();
    reporter.report(&result);

    info!("Benchmark completed successfully");
    Ok(())
}
