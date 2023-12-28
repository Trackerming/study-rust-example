use crate::cli::{
    Cli, EthSubCommands,
    SubCommands::{Decrypt, Encrypt, Eth, Reverse},
};
use anyhow::Result;
use tracing::{debug, error, info, warn};

pub mod cli;
pub mod encrypt_decrypt;
pub mod eth;
pub mod util;

use crate::encrypt_decrypt::{decrypt, encrypt};
use crate::eth::private_key_to_address;

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
        Decrypt {
            cipher,
            password,
            iv,
            tag,
            aad,
        } => decrypt(cipher, password, iv, tag, aad),
        Reverse { text, code } => reverse(text, code),
        Eth(EthSubCommands) => handle_eth_sub_command(EthSubCommands),
    };
    Ok(())
}

pub fn handle_eth_sub_command(eth_sub_commands: EthSubCommands) -> Result<()> {
    match eth_sub_commands {
        EthSubCommands::Sec2Address { private_key } => private_key_to_address(private_key),
    }
}

fn reverse(text: String, code: String) -> Result<()> {
    /*let text_array = text.as_bytes();
    let len = code.len();
    let mut result: Vec<u8> = Vec::new();
    for index in 0..(text_array.len() / len) {
        for current_index in code.as_bytes() {
            result.push(text_array[index * len  + u8::from_str_radix(&code[*current_index], 10).unwrap() as usize]);
        }
    }
    let tail = text_array.len() % len;
    if tail != 0 {
        result.copy_from_slice(&text_array[(text_array.len() - tail)..])
    }
    let result_string = String::from_utf8(result.clone()).unwrap();
    info!("reverse result: {:?}, {:?}", result, result_string );*/
    Ok(())
}
