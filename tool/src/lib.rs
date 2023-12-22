use crate::cli::{
    Cli,
    SubCommands::{Decrypt, Encrypt},
};
use anyhow::Result;
use tracing::{debug, error, info, warn};

pub mod cli;
pub mod encrypt_decrypt;

use crate::encrypt_decrypt::{decrypt, encrypt};

pub async fn start(args: Cli) -> Result<()> {
    debug!("test debug info.");
    info!("test log info.");
    warn!("test warn info.");
    error!("test error info.");
    info!("cli args: {:?}", args);
    let _ = match args.command {
        Encrypt {
            plaintext,
            password,
        } => encrypt(plaintext, password),
        Decrypt { cipher, password } => decrypt(cipher, password),
    };
    Ok(())
}
