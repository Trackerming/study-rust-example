use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub(crate) enum SubCommands {
    Encrypt {
        #[arg(short = 't', long, default_value = "plaintext")]
        plaintext: String,
        #[arg(short = 'p', long, default_value = "password")]
        password: String,
    },
    Decrypt {
        #[arg(short = 'c', long, default_value = "cipher")]
        cipher: String,
        #[arg(short = 'p', long, default_value = "password")]
        password: String,
    },
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: SubCommands,
}
