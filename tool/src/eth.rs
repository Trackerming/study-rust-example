use crate::bip32::{derive_private_by_path, derive_public_by_path, mnemonic_to_x_prv};
use crate::http_request::fetch_url;
use crate::util::u8_array_convert_string;
use anyhow::Result;
use bip32::secp256k1::elliptic_curve::weierstrass::add;
use bip32::{Prefix, PublicKey as Bip32PubKey};
use clap::builder::Str;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use ethers::abi::AbiEncode;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::utils::hex::ToHex;
use ethers::{
    core::types::{Address, TransactionRequest},
    etherscan::Client,
    prelude::*,
    signers::LocalWallet,
};
use num_traits::Num;
use regex::Regex;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde_json::json;
use std::collections::HashMap;
use std::f64;
use std::ops::{Div, Mul, Sub};
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;

pub async fn create_transaction(
    private_key: String,
    rpc_url: String,
    to: String,
    value: u128,
    chain_id: u8,
    is_broadcast: bool,
    contract: Option<String>,
    gas_price: Option<u128>,
    gas_limit: Option<u128>,
) -> Result<()> {
    let wallet = private_key.as_str().parse::<LocalWallet>().unwrap();
    let provider = Provider::<Http>::try_from(rpc_url.as_str()).unwrap();
    let client = Arc::new(SignerMiddleware::new(provider, wallet));
    let mut tx_request: TypedTransaction = if contract.is_none() {
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
    if let Some(gas_price_val) = gas_price {
        tx_request.set_gas_price(gas_price_val);
    }
    if let Some(gas_limit_val) = gas_limit {
        tx_request.set_gas(gas_limit_val);
    }
    client
        .fill_transaction(&mut tx_request, None)
        .await
        .unwrap();
    let sig = client.signer().sign_transaction(&tx_request).await.unwrap();
    let tx = tx_request.rlp_signed(&sig);
    info!("tx: {:?}", tx);
    if is_broadcast {
        let pending_tx = client.provider().send_raw_transaction(tx).await.unwrap();
        info!("txHash: {:?}", pending_tx.tx_hash());
    }
    Ok(())
}

pub async fn calculate_balance(
    rpc_url: String,
    address: String,
    gas_price: String,
    gas_limit: String,
    block_id: Option<u64>,
) -> Result<()> {
    // 获取地址余额
    let provider = Provider::try_from(rpc_url.as_str()).unwrap();
    let block_id = match block_id {
        Some(id) => Some(BlockId::from(id)),
        None => None,
    };
    let balance = provider
        .get_balance(address.as_str().parse::<Address>().unwrap(), block_id)
        .await
        .expect("query address balance error");
    let gas_price_on_chain = provider.get_gas_price().await.unwrap();
    let g_wei: U256 = 1_000_000_000.into();
    let gas_price = U256::from_str_radix(gas_price.as_str(), 10)
        .unwrap()
        .mul(g_wei);
    let fee = gas_price.mul(U256::from_str_radix(gas_limit.as_str(), 10).unwrap());
    if balance.gt(&fee) {
        let transfer_balance = balance.sub(fee);
        println!("address: {:?}, transfer balance: {:?}(gas_price: {gas_price}, gas_limit: {gas_limit}), chain_gas_price = {:?}GWei", address, transfer_balance, gas_price_on_chain.div(g_wei));
    } else {
        println!("address balance: {balance}, fee: {fee}");
    }
    Ok(())
}

#[derive(Debug)]
pub enum Unit {
    Wei(U256),
    GWei(f64),
    Eth(f64),
}

impl Unit {
    pub fn get_all(&self) -> Vec<Unit> {
        let wei: U256 = 1_000_000_000_000_000_000i64.into();
        let g_wei = U256::from(1_000_000_000);
        let wei_f64 = wei.as_u64() as f64;
        let g_wei_f64 = g_wei.as_u64() as f64;
        let mut result = vec![];
        match &self {
            Unit::Eth(val) => {
                result.push(Unit::Wei(U256::from(wei_f64.mul(*val) as u64)));
                result.push(Unit::GWei(g_wei_f64.mul(*val)));
                result.push(Unit::Eth(*val));
            }
            Unit::Wei(val) => {
                result.push(Unit::Wei(*val));
                // 损失了精度位，这里要优化就得借助第三方库像rust_decimal进行优化
                result.push(Unit::GWei((val.as_u128() as f64).div(&g_wei_f64)));
                result.push(Unit::Eth((val.as_u128() as f64).div(&wei_f64)));
            }
            Unit::GWei(val) => {
                result.push(Unit::Wei(U256::from(val.mul(g_wei_f64) as u64)));
                result.push(Unit::GWei(*val));
                result.push(Unit::Eth(val.div(g_wei_f64)));
            }
        }
        result
    }
}

pub trait EthersUnit {
    fn parse_uint_str(&self) -> Option<Unit>;
}

fn split_number_and_uint(input: &str) -> Option<(String, String)> {
    let reg = r"(\d+(\.\d+)?)\s*(\w+)";
    let re = Regex::new(reg).unwrap();
    if let Some(captures) = re.captures(input) {
        Some((
            captures.get(1).unwrap().as_str().to_string(),
            captures.get(3).unwrap().as_str().to_string(),
        ))
    } else {
        None
    }
}

impl EthersUnit for String {
    fn parse_uint_str(&self) -> Option<Unit> {
        if let Some((amount, unit)) = split_number_and_uint(&self) {
            let result = match unit.to_lowercase().as_str() {
                "wei" => Unit::Wei(U256::from_str_radix(&amount, 10).unwrap()),
                "gwei" => Unit::GWei(f64::from_str_radix(&amount, 10).unwrap()),
                "eth" => Unit::Eth(f64::from_str_radix(&amount, 10).unwrap()),
                _ => {
                    panic!("unit not supported.")
                }
            };
            Some(result)
        } else {
            None
        }
    }
}

pub fn eth_convert(amount: String) -> Result<()> {
    if let Some(amount_unit) = amount.parse_uint_str() {
        let result = amount_unit.get_all();
        println!("{amount} equal: {:?}", result);
    }
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

pub fn mnemonic_to_key_pair_by_path(
    mnemonic: String,
    passphrase: String,
    path: String,
) -> Result<()> {
    let key = mnemonic_to_x_prv(mnemonic, passphrase);
    let xkey_str = &*key.to_string(Prefix::XPRV).to_string();
    let mut path_complete = "m/44'/60'/0'".to_string();
    path_complete.push_str(&path[1..]);
    let extend_key = derive_private_by_path(path_complete, xkey_str.to_string());
    let address = pub_key_to_address(
        PublicKey::from_slice(extend_key.public_key().to_bytes().to_vec().as_slice()).unwrap(),
    );
    let private_key = u8_array_convert_string(extend_key.to_bytes().to_vec().as_slice());
    info!("private key: {:?}, address: {:?}", private_key, address);
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

pub async fn query_account_by_etherscan(
    address: String,
    api_key: String,
    chain: u64,
) -> Result<()> {
    let client = Client::new(
        Chain::try_from(chain).expect("chain try from {chain} failed"),
        api_key,
    )
    .expect("client init failed.");
    let address = address.as_str().parse()?;
    // 获取链的原生币种余额
    let native_balance = client.get_ether_balance_single(&address, None).await?;
    // 查询指定token的价格
    let price = client.eth_price().await?;
    info!(
        "address: {:?}, native token: {:?}, price: {:?}",
        native_balance.account, native_balance.balance, price.ethusd
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::runtime::Runtime;

    #[ignore]
    #[test]
    fn test_create_tx() {
        let rt = Runtime::new().unwrap();
        let _ = rt.block_on(create_transaction(
            "1cb90607624a78a065b51ded6fc701c381aa6b0aef37ed278f15774dd5b85758".to_string(),
            "https://ethereum-goerli.publicnode.com".to_string(),
            "0x9BF5a8AF3333e2bF300FB00A0B7B8aDddc90dd43".to_string(),
            100000000000000000,
            0x05,
            false,
            None,
            None,
            None,
        ));
    }

    #[ignore]
    #[test]
    fn test_create_erc20_tx() {
        let rt = Runtime::new().unwrap();
        let _ = rt.block_on(create_transaction(
            "1cb90607624a78a065b51ded6fc701c381aa6b0aef37ed278f15774dd5b85758".to_string(),
            "https://ethereum-goerli.publicnode.com".to_string(),
            "0x9BF5a8AF3333e2bF300FB00A0B7B8aDddc90dd43".to_string(),
            100000,
            0x05,
            false,
            Some("0xBA62BCfcAaFc6622853cca2BE6Ac7d845BC0f2Dc".to_string()),
            None,
            None,
        ));
    }

    #[test]
    fn test_split_number_uint() {
        let str1 = "1326 ETH".to_string();
        let str2 = "1000236326 GWei".to_string();
        let str3 = "100023632602373623 WEI".to_string();
        let str4 = "1000236326GWei".to_string();
        let result1 = split_number_and_uint(&str1);
        println!("result1: {:?}", result1);
        assert_eq!(result1.unwrap(), ("1326".to_string(), "ETH".to_string()));
        let result2 = split_number_and_uint(&str2);
        println!("result2: {:?}", result2);
        assert_eq!(
            result2.unwrap(),
            ("1000236326".to_string(), "GWei".to_string())
        );
        let result3 = split_number_and_uint(&str3);
        println!("result3: {:?}", result3);
        assert_eq!(
            result3.unwrap(),
            ("100023632602373623".to_string(), "WEI".to_string())
        );
        let result4 = split_number_and_uint(&str4);
        println!("result4: {:?}", result4);
        assert_eq!(
            result4.unwrap(),
            ("1000236326".to_string(), "GWei".to_string())
        );
    }

    #[ignore]
    #[test]
    fn test_unit_convert() {
        let str1 = "1.326 ETH".to_string();
        let str2 = "1000236326 GWei".to_string();
        let str3 = "100023632602373623 WEI".to_string();
        let str4 = "1000236326GWei".to_string();
        let _ = eth_convert(str1);
        let _ = eth_convert(str2);
        let _ = eth_convert(str3);
        let _ = eth_convert(str4);
    }
}
