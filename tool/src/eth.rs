use anyhow::Result;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use std::str::FromStr;
use tracing::info;

use crate::util::u8_array_convert_string;

pub fn get_public_key(private_key: &str) -> PublicKey {
    let secp = Secp256k1::new();
    let sec_key = SecretKey::from_str(private_key).expect("secret key from str failed.");
    PublicKey::from_secret_key(&secp, &sec_key)
}

pub fn pub_key_to_address(public_key: PublicKey) -> String {
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
