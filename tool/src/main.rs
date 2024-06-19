use anyhow::Result;
use clap::Parser;
use tool_lib::{cli::Cli, start};
use tracing::{info, metadata::LevelFilter, Level};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::from_level(Level::INFO).into())
                .from_env()
                .expect("should create tracing subscribe env filter"),
        )
        .init();
    let args = Cli::parse();
    info!("crypto cli start...");
    let result = start(args).await?;
    info!("crypto cli finished {:?}", result);
    Ok(())
}
