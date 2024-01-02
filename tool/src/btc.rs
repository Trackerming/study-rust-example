use anyhow::Result;
use bs58::encode;
use crypto::digest::Digest;
use crypto::{ripemd160, sha2::Sha256};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use std::str::FromStr;
use tracing::info;

use crate::util::u8_array_convert_string;

// base58 for 1
const P2PKH_PREFIX: u8 = 0x00;
const OP_DUP: u8 = 0x76;
const OP_HASH160: u8 = 0xa9;
const OP_EQUALVERIFY: u8 = 0x88;
const OP_CHECKSIG: u8 = 0xac;

// 先实现最基础的P2PKH和P2WPKH，其他的多签和script的类型暂不考虑，后续考虑优化，这里也不过多依赖第三方库
fn pub_key_to_address(public_key: PublicKey) -> String {
    // sha256
    let mut sha256 = Box::new(Sha256::new());
    // 压缩型的地址
    sha256.input(&public_key.serialize()[..]);
    let mut out_1 = vec![0u8; sha256.output_bytes()];
    sha256.result(&mut out_1);
    // ripemd160
    let mut hash = Box::new(ripemd160::Ripemd160::new());
    hash.input(&out_1);
    let mut out = vec![0u8; hash.output_bytes()];
    hash.result(&mut out);
    // P2PKH
    let p2pkh = [
        &[OP_DUP, OP_HASH160, out.len() as u8][..],
        &out[..],
        &[OP_EQUALVERIFY, OP_CHECKSIG],
    ]
    .concat();
    let script_hex = u8_array_convert_string(&p2pkh);
    info!("script hex: {:?}", script_hex);
    // base58 编码
    // 获取checksum
    // sha256
    let leading_byte = 0;
    out.insert(0, leading_byte);
    let mut sha256 = Box::new(Sha256::new());
    sha256.input(&out);
    let mut result = vec![0u8; sha256.output_bytes()];
    sha256.result(&mut result);
    // sha256
    let mut sha256 = Box::new(Sha256::new());
    sha256.input(&result);
    sha256.result(&mut out_1);
    out.extend_from_slice(&out_1[..4]);
    encode(out).into_string()
}

// 后续加network和地址类型参数
pub fn network_pub_key_to_address(public_key: String) -> Result<()> {
    let public_key =
        PublicKey::from_str(&public_key).expect("btc get public key from string failed");
    let _ = pub_key_to_address(public_key);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn to_address() {
        let pub_key =
            "0281e8a3181164227ff6b3b759fdcd5175b30500c7391ef6ee070a3bc2316c64da".to_string();
        let public_key =
            PublicKey::from_str(&pub_key).expect("btc get public key from string failed");
        let address = pub_key_to_address(public_key);
        assert_eq!(address, "1GKSnhP1XmCjZpEyUoupWsm7c1o64seyow".to_string());
    }
}
