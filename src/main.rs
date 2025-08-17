use anyhow::Result;
use warpbench::cli;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse CLI arguments
    let config = cli::Cli::parse_args()?;

    println!("Config: {:?}", config);

    // Print the results
    Ok(())
}
