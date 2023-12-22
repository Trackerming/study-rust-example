use tracing::{debug, error, info, metadata::LevelFilter, warn, Level};
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::from_level(Level::DEBUG).into())
                .from_env()
                .expect("should create tracing subscribe env filter"),
        )
        .init();
    debug!("test debug info.");
    info!("test log info.");
    warn!("test warn info.");
    error!("test error info.");
}
