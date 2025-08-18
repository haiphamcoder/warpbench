use anyhow::Result;
use warpbench::{
    cli::Cli,
    load_generator::LoadGenerator,
    metrics::MetricsReporter,
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments first to get logging preferences
    let config = Cli::parse_args()?;
    
    // Save verbose flag before moving config
    let verbose = config.verbose;
    
    // Initialize tracing based on CLI flags
    let level = if config.verbose {
        Level::DEBUG
    } else if config.quiet {
        Level::WARN
    } else {
        Level::ERROR  // Default: only show errors, not info logs
    };
    
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    if verbose {
        info!("WarpBench - HTTP Benchmarking Tool");
        info!("Version: {}", env!("CARGO_PKG_VERSION"));
    }
    
    if verbose {
        info!("Starting benchmark with configuration:");
        info!("  URL: {}", config.url);
        info!("  Duration: {:?}", config.duration);
        info!("  Threads: {}", config.threads);
        info!("  Connections: {}", config.connections);
        info!("  Timeout: {:?}", config.timeout);
        
        if let Some(script_path) = &config.script_path {
            info!("  Script: {}", script_path);
        }
    }

    // Show startup progress for all modes
    if !config.quiet {
        eprintln!("Initializing WarpBench...");
        eprintln!("Target: {}", config.url);
        eprintln!("Configuration: {} threads, {} connections, {} duration", 
                 config.threads, config.connections, humantime::format_duration(config.duration));
    }

    // Create and run load generator
    if !config.quiet {
        eprintln!("Setting up load generator...");
    }
    let load_generator = LoadGenerator::new(config.clone())?;
    let result = load_generator.run().await?;

    // Report results
    let reporter = MetricsReporter::new();
    reporter.report(&result);

    if verbose {
        info!("Benchmark completed successfully");
    }
    Ok(())
}
