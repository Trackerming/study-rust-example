use clap::Parser;
use tool_lib::{start, Cli};
use tracing::{info, metadata::LevelFilter, Level};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::from_level(Level::DEBUG).into())
                .from_env()
                .expect("should create tracing subscribe env filter"),
        )
        .init();
    let args = Cli::parse();
    info!("start...");
    let _ = start(args).await;
}
