use crate::bip32::{derive_private_by_path, derive_public_by_path, mnemonic_to_x_prv};
use crate::http_request::fetch_url;
use crate::util::u8_array_convert_string;
use anyhow::Result;
use bip32::{Prefix, PublicKey as Bip32PubKey};
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use ethers::abi::AbiEncode;
use ethers::utils::hex::ToHex;
use ethers::{
    core::types::{Address, TransactionRequest},
    prelude::*,
    signers::LocalWallet,
};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde_json::json;
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;

pub async fn create_transaction(
    private_key: String,
    rpc_url: String,
    to: String,
    value: u128,
    chain_id: u8,
    contract: Option<String>,
) -> Result<()> {
    let wallet = private_key.as_str().parse::<LocalWallet>().unwrap();
    let provider = Provider::<Http>::try_from(rpc_url.as_str()).unwrap();
    let client = Arc::new(SignerMiddleware::new(provider, wallet));
    let mut tx_request = if contract.is_none() {
        TransactionRequest::new()
            .to(to.as_str())
            .value(value)
            .chain_id(chain_id)
            .into()
    } else {
        abigen!(
            ERC20Contract,
            r#"[
            function balanceOf(address account) external view returns (uint256)
            function decimals() external view returns (uint8)
            function symbol() external view returns (string memory)
            function transfer(address to, uint256 amount) external returns (bool)
            event Transfer(address indexed from, address indexed to, uint256 value)
        ]"#,
        );
        let address = contract.unwrap().as_str().parse::<Address>().unwrap();
        let contract = ERC20Contract::new(address, client.clone());
        (*contract
            .transfer(to.as_str().parse::<Address>().unwrap(), value.into())
            .tx
            .set_chain_id(chain_id)
            .set_value(0))
        .clone()
        .into()
    };
    client
        .fill_transaction(&mut tx_request, None)
        .await
        .unwrap();
    let sig = client.signer().sign_transaction(&tx_request).await.unwrap();
    let tx = tx_request.rlp_signed(&sig);
    info!("tx: {:?}", tx);
    let pending_tx = client.provider().send_raw_transaction(tx).await.unwrap();
    info!("txHash: {:?}", pending_tx.tx_hash());
    Ok(())
}

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

pub fn bip39_to_key(mnemonic: String, passphrase: String) -> Result<()> {
    let key = mnemonic_to_x_prv(mnemonic, passphrase);
    let xkey_str = &*key.to_string(Prefix::XPRV).to_string();
    let extend_key = derive_private_by_path("m/44'/60'/0'".to_string(), xkey_str.to_string());
    info!("xPrivKey: {:?}", xkey_str);
    info!(
        "extend key for m/44'/60'/0': {:?}\n xPubKey: {:?}",
        extend_key.to_string(Prefix::XPRV),
        extend_key.public_key().to_string(Prefix::XPUB)
    );
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

#[cfg(test)]
mod test {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_create_tx() {
        let rt = Runtime::new().unwrap();
        let _ = rt.block_on(create_transaction(
            "1cb90607624a78a065b51ded6fc701c381aa6b0aef37ed278f15774dd5b85758".to_string(),
            "https://ethereum-goerli.publicnode.com".to_string(),
            "0x9BF5a8AF3333e2bF300FB00A0B7B8aDddc90dd43".to_string(),
            100000000000000000,
            0x05,
            None,
        ));
    }

    #[test]
    fn test_create_erc20_tx() {
        let rt = Runtime::new().unwrap();
        let _ = rt.block_on(create_transaction(
            "1cb90607624a78a065b51ded6fc701c381aa6b0aef37ed278f15774dd5b85758".to_string(),
            "https://ethereum-goerli.publicnode.com".to_string(),
            "0x9BF5a8AF3333e2bF300FB00A0B7B8aDddc90dd43".to_string(),
            100000,
            0x05,
            Some("0xBA62BCfcAaFc6622853cca2BE6Ac7d845BC0f2Dc".to_string()),
        ));
    }
}
