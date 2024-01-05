use clap::{Args, Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum EthSubCommands {
    Sec2Address {
        #[arg(short = 's', long, default_value = "private_key")]
        private_key: String,
    },

    Pub2Address {
        #[arg(short = 'p', long, default_value = "public_key")]
        public_key: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum BtcSubCommands {
    PrivateKeyConvert {
        #[arg(short = 's', long, default_value = "private_key")]
        private_key: String,
        #[arg(short = 'f', long, default_value = "hex")]
        format: String,
    },
    Sec2Address {
        #[arg(short = 's', long, default_value = "private_key")]
        private_key: String,
    },
    Pub2Address {
        #[arg(short = 's', long, default_value = "public_key")]
        public_key: String,
    },
    Address2Script {
        #[arg(short = 'a', long, default_value = "address")]
        address: String,
    },
    RawTx2TxHash {
        #[arg(short = 'a', long, default_value = "raw_tx")]
        raw_tx: String,
    },
}

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
        #[arg(short = 'i', long, default_value = "iv")]
        iv: String,
        #[arg(short = 't', long, default_value = "tag")]
        tag: String,
        #[arg(short = 'a', long, default_value = "aad")]
        aad: String,
    },
    Reverse {
        #[arg(short = 't', long, default_value = "text")]
        text: String,
        #[arg(short = 'c', long, default_value = "code")]
        code: String,
    },
    #[command(subcommand)]
    Eth(EthSubCommands),
    #[command(subcommand)]
    Btc(BtcSubCommands),
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: SubCommands,
}
