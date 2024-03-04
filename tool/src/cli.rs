use clap::{Args, Parser, Subcommand};
use ethers::types::U128;

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

    ChainInfo {
        #[arg(short = 's', long, default_value = "address")]
        address: String,
        #[arg(short = 'k', long, default_value = "api_key")]
        api_key: String,
        #[arg(short = 'i', long)]
        chain_id: u64,
    },
    Bip32 {
        #[arg(short = 's', long, default_value = "x_private_key")]
        x_private_key: Option<String>,
        #[arg(short = 'u', long, default_value = "x_public_key")]
        x_public_key: Option<String>,
        #[arg(short = 'p', long, default_value = "path")]
        path: String,
    },
    Bip39 {
        #[arg(short = 'm', long, default_value = "mnemonic")]
        mnemonic: String,
        #[arg(short = 'p', long, default_value = "passphrase")]
        passphrase: String,
    },
    ExportPrivateKey {
        #[arg(short = 'm', long, default_value = "mnemonic")]
        mnemonic: String,
        #[arg(short = 'p', long, default_value = "passphrase")]
        passphrase: String,
        #[arg(short = 't', long, default_value = "path")]
        path: String,
    },
    Transfer {
        #[arg(short = 's', long, default_value = "private_key")]
        private_key: String,
        #[arg(short = 'r', long, default_value = "rpc host")]
        rpc_url: String,
        #[arg(short = 't', long, default_value = "destination")]
        to: String,
        #[arg(short = 'v', long)]
        value: u128,
        #[arg(short = 'i', long)]
        chain_id: u8,
        #[arg(short = 'b', long)]
        is_broadcast: bool,
        #[arg(short = 'c', long)]
        contract_address: Option<String>,
        #[arg(short = 'p', long)]
        gas_price: Option<u128>,
        #[arg(short = 'l', long)]
        gas_limit: Option<u128>,
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
    Bip32 {
        #[arg(short = 's', long, default_value = "x_private_key")]
        x_private_key: Option<String>,
        #[arg(short = 'u', long, default_value = "x_public_key")]
        x_public_key: Option<String>,
        #[arg(short = 'p', long, default_value = "path")]
        path: String,
    },
    Bip39 {
        #[arg(short = 'm', long, default_value = "mnemonic")]
        mnemonic: String,
        #[arg(short = 'p', long, default_value = "passphrase")]
        passphrase: String,
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
    Random {
        #[arg(short = 'i', long)]
        min: usize,
        #[arg(short = 'a', long)]
        max: usize,
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
