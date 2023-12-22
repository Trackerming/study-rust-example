use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{debug, error, info, warn};

#[derive(Subcommand, Debug)]
pub(crate) enum SubCommands {
    SubCli1 {
        #[arg(short = 's', long, default_value = "address")]
        address: String,
    },
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: SubCommands,
}

pub async fn start(args: Cli) -> Result<()> {
    debug!("test debug info.");
    info!("test log info.");
    warn!("test warn info.");
    error!("test error info.");
    info!("cli args: {:?}", args);
    Ok(())
}
