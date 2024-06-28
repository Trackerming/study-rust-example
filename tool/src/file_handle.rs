use anyhow::Result as AnyResult;
use csv::Writer;
use regex::Regex;
use serde::Deserialize;
use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Account {
    address: String,
    native_token: u128,
    erc20_token: u128,
}

impl Account {
    pub fn new(address: &str, native_token: u128, erc20_token: u128) -> Self {
        Account {
            address: address.to_string(),
            native_token,
            erc20_token,
        }
    }
}

pub fn read_file_line(file_path: &str, ele: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut match_lines = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.contains(ele) {
            match_lines.push(line);
        }
    }
    Ok(match_lines)
}

pub fn format_line(line: String, reg: &str) -> Option<Account> {
    let re = Regex::new(reg).unwrap();
    if let Some(captures) = re.captures(&line) {
        let address = captures.get(1).unwrap().as_str();
        let bnb = u128::from_str_radix(captures.get(2).unwrap().as_str(), 10).unwrap();
        let bsc_usdt = u128::from_str_radix(captures.get(3).unwrap().as_str(), 10).unwrap();
        let bsc_eth = u128::from_str_radix(captures.get(4).unwrap().as_str(), 10).unwrap();
        Some(Account::new(address, bnb, bsc_usdt))
    } else {
        None
    }
}

fn save_csv(accounts: Vec<Account>, path_str: &str) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(path_str);
    let mut writer = Writer::from_writer(File::create(file_path)?);

    writer.write_record(&["address", "native_token", "erc20_token"])?;
    for a in accounts {
        writer.write_record(&[
            a.address,
            a.native_token.to_string(),
            a.erc20_token.to_string(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

pub fn to_csv_file(path_str: &str, json_data: &str) -> Result<(), Box<dyn Error>> {
    let accounts: Vec<Account> = serde_json::from_str(json_data)?;
    save_csv(accounts, path_str)?;
    Ok(())
}

pub fn log2_csv_file(
    input_file: String,
    output_file: String,
    key_word: String,
    reg: String,
) -> AnyResult<()> {
    let lines = read_file_line(input_file.as_str(), key_word.as_str()).unwrap();
    let mut accounts = vec![];
    for line in lines {
        if let Some(account) = format_line(line, reg.as_str()) {
            accounts.push(account);
        }
    }
    save_csv(accounts, output_file.as_str()).expect("save csv file failed");
    Ok(())
}

#[cfg(test)]
mod test_file_hanle {
    use super::*;
    use std::fs;

    #[test]
    fn test_read_line() {
        let lines = read_file_line("./bin/file_handle.rs", "to_csv_file").unwrap();
        assert_eq!(lines.len(), 5);
    }

    #[test]
    fn test_to_csv_file() {
        // JSON数据
        let json_data = r#"[
        {"address": "0x8f684eab67f7105e43b4ef705df5cbad92bccb0a", "native_token": 0, "erc20_token": 399200000000000000000},
        {"address": "0x68424a917c5c6dbd9088b5b32d194b2822e1dcb3", "native_token": 1859555000000000, "erc20_token": 43021000000000000000},
        {"address": "0x8d9b6706e966ab1294fe85a6684b2c4f8ab4e58d", "native_token": 6307108873877526, "erc20_token": 254345212960000000000}
    ]"#;
        let file_path = "./bin/test_to_csv_file.csv";
        let to_result = to_csv_file(file_path, json_data);
        println!("to_result: {:?}", to_result);
        assert_eq!(to_result.is_err(), false);
        let csv_content = read_file_line(file_path, "address").unwrap();
        println!("csv_content:{:?}", csv_content);
        fs::remove_file(file_path).unwrap();
        assert_eq!(csv_content.len(), 1);
    }

    #[test]
    fn test_format_line() {
        let line = "address: 0xa80bc9e64ab4579f66757870565de0aa3249ab32, BNB = 32452739216, contractBalance :  BSC-USD,34200000000000000000,BSC-ETH,7092020000000000".to_string();
        let reg = r"address: (.*?), BNB = (.*?), contractBalance :  BSC-USD,(.*?),BSC-ETH,(.*)";
        let account = format_line(line, reg).unwrap();
        assert_eq!(
            account.address,
            "0xa80bc9e64ab4579f66757870565de0aa3249ab32"
        );
        assert_eq!(account.native_token, 32452739216);
        assert_eq!(account.erc20_token, 34200000000000000000);
    }
}
