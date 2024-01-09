use crate::bip32::{derive_private_by_path, derive_public_by_path};
use crate::http_request::fetch_url;
use anyhow::Result;
use bip32::PublicKey as Bip32PubKey;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde_json::json;
use std::str::FromStr;
use tracing::info;

use crate::util::u8_array_convert_string;

pub fn get_public_key(private_key: &str) -> PublicKey {
    let secp = Secp256k1::new();
    let sec_key = SecretKey::from_str(private_key).expect("secret key from str failed.");
    PublicKey::from_secret_key(&secp, &sec_key)
}

fn pub_key_to_address(public_key: PublicKey) -> String {
    let mut hash = Box::new(Sha3::keccak256());
    hash.input(&public_key.serialize_uncompressed()[1..]);
    let mut out = vec![0u8; hash.output_bytes()];
    hash.result(&mut out);
    // 取后20个bytes作为地址
    let addr = u8_array_convert_string(&out[12..]);
    let mut address = "0x".to_string();
    address.push_str(&addr);
    address
}

pub fn pub_key_str_to_address(public_key: String) -> Result<()> {
    let public_key = PublicKey::from_str(&public_key).expect("PublicKey from str failed.");
    let address = pub_key_to_address(public_key);
    info!("address: {:?}", address);
    Ok(())
}

pub fn private_key_to_address(private_key: String) -> Result<()> {
    let public_key = get_public_key(&private_key);
    let address = pub_key_to_address(public_key);
    info!(
        "public_key: {:?}, \naddress: {:?}",
        public_key.to_string(),
        address
    );
    Ok(())
}

pub fn bip32_to_address(xkey: String, path: String) -> Result<()> {
    let x_pub_key = if xkey.starts_with("xprv") {
        let x_priv_key = derive_private_by_path(path, xkey);
        x_priv_key.public_key()
    } else {
        derive_public_by_path(path, xkey)
    };
    let address = pub_key_to_address(
        PublicKey::from_slice(x_pub_key.public_key().to_bytes().to_vec().as_slice()).unwrap(),
    );
    info!("address: {:?}", address);
    Ok(())
}

pub async fn query_chain_info_by_address(
    host: String,
    api_key: String,
    address: String,
) -> Result<()> {
    let url = host + &'/'.to_string() + &api_key;
    let url = url.parse().unwrap();
    let mut params = Vec::new();
    params.push(address.as_str());
    params.push("latest");
    let json_data = json!({
        "jsonrpc": "2.0",
        "method": "eth_getBalance",
        "params": params,
        "id": 1
    });
    fetch_url(url, serde_json::to_string(&json_data).unwrap())
        .await
        .unwrap();
    Ok(())
}
