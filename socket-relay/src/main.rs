use anyhow::Result;
use clap::Parser;
use tracing::metadata::LevelFilter;
use tracing_subscriber::EnvFilter;

use socket_relay::{listen_and_serve, Cli};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env()
                .expect("should create tracing subscribe env filter"),
        )
        .init();
    listen_and_serve(&args).await
}
