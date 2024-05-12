use crate::bip32::{derive_private_by_path, derive_public_by_path, mnemonic_to_x_prv};
use crate::http_request::fetch_url;
use crate::util::{hex_string_2_array, u8_array_convert_string};
use anyhow::Result;
use bip32::secp256k1::elliptic_curve::weierstrass::add;
use bip32::{Prefix, PublicKey as Bip32PubKey};
use bytes::Buf;
use clap::builder::Str;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use ethers::abi::{parse_abi_str, AbiEncode};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::utils::hex::ToHex;
use ethers::{
    core::types::{Address, TransactionRequest},
    etherscan::Client,
    prelude::*,
    signers::LocalWallet,
};
use num_bigint::BigInt;
use num_bigint::Sign::{NoSign, Plus};
use num_traits::Num;
use regex::Regex;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde_json::json;
use std::collections::HashMap;
use std::f64;
use std::io::{Cursor, Read};
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
    nonce: Option<u128>,
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
    if let Some(nonce) = nonce {
        tx_request.set_nonce(nonce);
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

fn parse_param(data: &[u8; 32]) -> String {
    let ignore_prefix_0: Vec<_> = data.iter().cloned().skip_while(|&x| x == 0).collect();
    if ignore_prefix_0.len() == 20 {
        return format!("{}{}", "0x", u8_array_convert_string(&ignore_prefix_0));
    } else if ignore_prefix_0.len() == 0 {
        return "false".to_string();
    } else if ignore_prefix_0.len() == 1 {
        return ignore_prefix_0[0].to_string();
    } else {
        // 先默认为value
        let value = BigInt::from_bytes_be(Plus, ignore_prefix_0.as_slice());
        let value_str = value.to_string();
        if let Some(u) = (value_str.clone() + "Wei").parse_uint_str() {
            let result = u.get_all();
            return format!("{:?}", result);
        } else {
            value_str
        }
    }
}

pub fn decode_call_data(
    data_field: String,
    abi_str: Option<String>,
    func_name: Option<String>,
) -> Result<()> {
    let data_field_str = data_field.as_str().trim_start_matches("0x");
    match abi_str {
        Some(abi_str) => {
            let parse_abi = parse_abi_str(&abi_str).expect("parse abi failed.");
            let func = parse_abi
                .function(func_name.expect("func name should exist").as_str())
                .unwrap();
            // 去除函数选择器数据字段否则解析不正确
            let decoded = func
                .decode_input(&hex_string_2_array(&data_field_str)[4..])
                .expect("parse data failed.");
            println!("decode result: {:?}", decoded);
        }
        None => {
            let data_bytes = hex_string_2_array(data_field_str);
            let function_selector = &data_bytes[..4];
            let mut params = vec![];
            let mut cursor = Cursor::new(&data_bytes[4..]);
            let mut read_32_bytes = [0; 32];
            while cursor.has_remaining() {
                cursor
                    .read_exact(&mut read_32_bytes)
                    .expect("read param must be 32 bytes.");
                params.push(parse_param(&read_32_bytes));
            }
            println!(
                "function_selector: {:?}\n, params: {:?}",
                u8_array_convert_string(function_selector),
                params
            );
        }
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

    #[ignore]
    #[test]
    fn test_decode_call_data() {
        let data = "0x1a1da0750000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000001569400000000000000000000000000000000000000000000000000000000000000180000000000000000000000004b65e3e1f90806be0ba79826c4a2146c628f72a90000000000000000000000000000000000000000000000000000cea46231e5360000000000000000000000000794238bf47c8fbf13bdcc15408d896c414061be000000000000000000000000000000000000000000000000000119b4f6ac38000000000000000000000000007be467b6c5e3f5f7dc46121b15bfb0b021ecf39e00000000000000000000000000000000000000000000000000058f346048ac00000000000000000000000000e6ccdd85cb7778c27df752c29a4c02c76c2753f4000000000000000000000000000000000000000000000000000ae243a968a40000000000000000000000000075d455cb1fb8eab214f8ab010adaa7749d684d55000000000000000000000000000000000000000000000000000eb29907875800000000000000000000000000538b5d406017afbe4467be4d3d7c1978ff28bb3d000000000000000000000000000000000000000000000000002114576fb08000000000000000000000000000c13f69c9e6e88a492ad93348249116888c84692d00000000000000000000000000000000000000000000000000354a6ba7a18000000000000000000000000000a7756f22f08d7bdaf93a0e51c33e005a1dfa6c7a00000000000000000000000000000000000000000000000000370478a92d7000000000000000000000000000cb1fd701815d5ce67b138e8cfe317067fb6fbb8b000000000000000000000000000000000000000000000000003f8ae06ab008000000000000000000000000007a21342536fb8f68eaae8af943a9349842301f5400000000000000000000000000000000000000000000000000430ae92ab868000000000000000000000000000fdc3e16dcbca3cb68478ff2d765659d24e264950000000000000000000000000000000000000000000000000050596a03ae1c000000000000000000000000002f167469082c4650f6407c502425f461468a789d00000000000000000000000000000000000000000000000000680280d9241c00000000000000000000000000fdfe0847cd314d7c3855a6f19d83e92355cd4e8a000000000000000000000000000000000000000000000000006cfcea218a2800000000000000000000000000cf01266b75c9b664f078ba5456a624a6ae712d3000000000000000000000000000000000000000000000000000ae485cf313b400000000000000000000000000739bb9a82ec9d37e0d3063f27e1df87bdb93b09600000000000000000000000000000000000000000000000000d8672d7abe9000000000000000000000000000bdf37e422dfeab5968a16e8d3babf0766f9c3f5000000000000000000000000000000000000000000000000000d8c36fd1e91000000000000000000000000000da7d38efc2d33fe0d6326471698ba72b86cbf35300000000000000000000000000000000000000000000000000d9d960dd5a9000000000000000000000000000a5732ba013402da91999c9ed4b776b65884222b500000000000000000000000000000000000000000000000000f0aee8f664bc00000000000000000000000000bde41a4a559be6fa2d14c8d395266496caf965c100000000000000000000000000000000000000000000000001461de93d421400000000000000000000000000298591a13ab8582985dc06b53544f12b4badd3a00000000000000000000000000000000000000000000000000220bc65293a4400000000000000000000000000794977e9bc1b717ccb9653a1b4ba2e933dfa05920000000000000000000000000000000000000000000000000220c547ea9b680000000000000000000000000012edc1d51bf2dd34c3703b7521f871e7e9a37c67000000000000000000000000000000000000000000000000029c20b5e2441400000000000000000000000000414d2b9d4a856856de57ce76cbc01119eeddb8920000000000000000000000000000000000000000000000000890cc61f25f8c00000000000000000000000000ad8c8106443ab8bef6b911c9fbe0fa17cc3c9f6a000000000000000000000000000000000000000000000000d71b0fe0a28e0000".to_string();
        let _ = decode_call_data(data, None, None);
        let data = "0xa9059cbb0000000000000000000000000ca0e077a7d81c8ba0aeb710d2cfe2aa5dd3d9550000000000000000000000000000000000000000000000000000000218711a00";
        let abi = r#"[
            function balanceOf(address account) external view returns (uint256)
            function decimals() external view returns (uint8)
            function symbol() external view returns (string memory)
            function transfer(address to, uint256 amount) external returns (bool)
            event Transfer(address indexed from, address indexed to, uint256 value)
        ]"#;
        let _ = decode_call_data(
            data.to_string(),
            Some(abi.to_string()),
            Some("transfer".to_string()),
        );
    }
}
