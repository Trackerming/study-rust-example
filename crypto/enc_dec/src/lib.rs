extern crate core;

pub mod base58;
pub mod bech32;
mod der;
mod rlp;
mod ssz;

pub fn hex_string_to_bytes(hex_str: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut hex_chars = hex_str.chars().peekable();
    while let Some(high) = hex_chars.next() {
        // 分别取出高位和低位的字符的数据
        let high_digit = high.to_digit(16).expect("high Invalid hex character.") as u8;
        let low = hex_chars.next().expect("Odd number of hex characters.");
        let low_digit = low.to_digit(16).expect("low Invalid hex character.") as u8;
        // 位操作合成一个完整字节
        bytes.push((high_digit << 4) | low_digit);
    }
    bytes
}
