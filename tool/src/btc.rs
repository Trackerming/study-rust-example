use crate::bip32::{derive_private_by_path, derive_public_by_path, mnemonic_to_x_prv};
use crate::eth::get_public_key;
use anyhow::Result;
use bip32::{Prefix, PublicKey as Bip32PubKey};
use bitcoin_hashes::{ripemd160, Hash};
use bs58::{decode, encode};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};
use std::str::FromStr;
use tracing::info;

use crate::util::{hex_string_2_array, u8_array_convert_string};

// base58 for 1
const P2PKH_PREFIX: u8 = 0x00;
const OP_DUP: u8 = 0x76;
const OP_HASH160: u8 = 0xa9;
const OP_EQUALVERIFY: u8 = 0x88;
const OP_CHECKSIG: u8 = 0xac;

// 先实现最基础的P2PKH和P2WPKH，其他的多签和script的类型暂不考虑，后续考虑优化，这里也不过多依赖第三方库
fn pub_key_to_address(public_key: PublicKey) -> String {
    // sha256
    let mut sha256 = Sha256::new();
    // 压缩型的地址
    sha256.update(&public_key.serialize()[..]);
    let out_1 = sha256.finalize().to_vec();
    // ripemd160
    let mut hash = ripemd160::Hash::hash(&out_1);
    let out_bytes: &[u8] = hash.as_ref();
    let mut out: Vec<u8> = out_bytes.to_vec();
    // P2PKH
    let p2pkh = [
        &[OP_DUP, OP_HASH160, 20][..],
        &out,
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
    let checksum = double_sha256(&out);
    out.extend_from_slice(&checksum[..4]);
    encode(out).into_string()
}

pub fn secret_to_address(secret_key: String) -> Result<()> {
    let public_key = get_public_key(&secret_key);
    let address = pub_key_to_address(public_key);
    info!("address: {:?}", address);
    Ok(())
}

fn address_to_p2pkh(address: String) -> String {
    let address_bytes = decode(address).into_vec().unwrap();
    // check
    let sep = address_bytes.len() - 4;
    let checksum = double_sha256(&address_bytes[..sep]);
    assert_eq!(&checksum[..4], &address_bytes[sep..]);
    let pub_key_bytes = &address_bytes[1..sep];
    let p2pkh = [
        &[OP_DUP, OP_HASH160, pub_key_bytes.len() as u8][..],
        &pub_key_bytes[..],
        &[OP_EQUALVERIFY, OP_CHECKSIG],
    ]
    .concat();
    let script_hex = u8_array_convert_string(&p2pkh);
    script_hex
}

pub fn get_tx_hash(raw_tx: String) -> Result<()> {
    let tx_bytes = hex_string_2_array(&raw_tx);
    let mut hash_bytes = double_sha256(&tx_bytes);
    hash_bytes.reverse();
    let tx_hash = u8_array_convert_string(&hash_bytes);
    println!("tx hash: {:?}", tx_hash);
    Ok(())
}

fn double_sha256(input: &[u8]) -> Vec<u8> {
    let mut sha256 = Sha256::new();
    sha256.update(&input[..]);
    let out = sha256.finalize().to_vec();
    let mut sha256 = Sha256::new();
    sha256.update(&out);
    let result = sha256.finalize().to_vec();
    result
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

pub fn bip39_to_key(mnemonic: String, passphrase: String) -> Result<()> {
    let key = mnemonic_to_x_prv(mnemonic, passphrase);
    let xkey_str = &*key.to_string(Prefix::XPRV).to_string();
    let extend_key = derive_private_by_path("m/44'/0'/0'".to_string(), xkey_str.to_string());
    info!("xPrivKey: {:?}", xkey_str);
    info!(
        "extend key for m/44'/0'/0': {:?}\n xPubKey: {:?}",
        extend_key.to_string(Prefix::XPRV),
        extend_key.public_key().to_string(Prefix::XPUB)
    );
    Ok(())
}

pub fn address_to_script(address: String) -> Result<()> {
    let p2pkh = address_to_p2pkh(address);
    info!("script_hex P2PKH: {:?}", p2pkh);
    Ok(())
}

// 后续加network和地址类型参数
pub fn network_pub_key_to_address(public_key: String) -> Result<()> {
    let public_key =
        PublicKey::from_str(&public_key).expect("btc get public key from string failed");
    let address = pub_key_to_address(public_key);
    info!("address: {:?}", address);
    Ok(())
}

pub fn wif_2_private_key(wif: String, is_compressed: bool) -> String {
    let key_bytes = bs58::decode(wif).into_vec().unwrap();
    let mut sep = key_bytes.len() - 4;
    let check_sum = double_sha256(&key_bytes[..sep]);
    assert_eq!(&key_bytes[sep..], &check_sum[..4]);
    // 压缩公钥 末尾多加一个0x01 字节的数据
    if is_compressed {
        sep = sep - 1;
        assert_eq!(key_bytes[sep], 1);
    }
    u8_array_convert_string(&key_bytes[1..sep])
}

pub fn private_2_wif_key(private_key: String, is_compressed: bool) -> String {
    let mut key_bytes = hex_string_2_array(&private_key);
    key_bytes.insert(0, 0x80u8);
    if is_compressed {
        key_bytes.push(0x01u8);
    };
    let checksum = double_sha256(&key_bytes);
    key_bytes.extend_from_slice(&checksum[..4]);
    bs58::encode(key_bytes).into_string()
}

pub fn private_key_convert(private_key: String, format: String) -> Result<()> {
    if format.eq(&"hex".to_string()) {
        let key = private_2_wif_key(private_key, true);
        info!("hex => wif: {:?}", key);
    } else {
        let key = wif_2_private_key(private_key, true);
        info!("wif => hex: {:?}", key);
    }
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

    #[test]
    pub fn to_script() {
        let address = "1GKSnhP1XmCjZpEyUoupWsm7c1o64seyow".to_string();
        let p2pkh = address_to_p2pkh(address);
        assert_eq!(
            p2pkh,
            "76a914a806e693f0de6638d99b90bb3c32bf0ece28abf388ac".to_string()
        );
    }

    #[test]
    pub fn test_wif_key_convert() {
        let wif = "5HueCGU8rMjxEXxiPuD5BDku4MkFqeZyd4dZ1jvhTVqvbTLvyTJ".to_string();
        let key = wif_2_private_key(wif, false);
        assert_eq!(
            key,
            "0c28fca386c7a227600b2fe50b7cae11ec86d3bf1fbe471be89827e19d72aa1d".to_string()
        );
        let wif_compute = private_2_wif_key(
            "0c28fca386c7a227600b2fe50b7cae11ec86d3bf1fbe471be89827e19d72aa1d".to_string(),
            false,
        );
        assert_eq!(
            wif_compute,
            "5HueCGU8rMjxEXxiPuD5BDku4MkFqeZyd4dZ1jvhTVqvbTLvyTJ".to_string()
        );
    }

    #[test]
    pub fn test_derive_key() {
        let xpub_key = "xpub6CA8TcTW7TPFnroJuxUqUJmFyhmNxmLJiLToAnwqZwSQzhBhVHTN3C19oAfcDfzHLdt3ZrVzgcWjsZ7ZGcGay6AjhtE8FcKGRfUGkfcayaL".to_string();
        let path = "m/0/1".to_string();
        bip32_to_address(xpub_key, path);
    }
}
