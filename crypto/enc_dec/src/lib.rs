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

pub fn bit_convert(input: &[u8], from: u32, to: u32, pad: bool) -> Vec<u8> {
    if from > 8 || to > 8 {
        panic!("convert `from` and `to` params greater than 8");
    }
    let mut acc: u32 = 0;
    let mut bits: u32 = 0;
    let mut ret: Vec<u8> = Vec::new();
    // 转换之后的最大值
    let max_v: u32 = (1 << to) - 1;
    for &value in input {
        let v = value as u32;
        /*if v >> from != 0 {
            return Err();
        }*/
        // 将当前字节读取到acc中并缓存之前的3个字节
        acc = (acc << from) | v;
        // 统计进入的bit位的个数
        bits += from;
        // bit位移动统计，当剩余的未转换的bit位依然比转换的目的位要大的时候进入循环并转换
        while bits >= to {
            bits -= to;
            ret.push(((acc >> bits) & max_v) as u8);
        }
    }
    if pad {
        if bits > 0 {
            ret.push(((acc << (to - bits)) & max_v) as u8);
        }
    } /*else if bits >= from || ((acc << (to - bits)) & max_v) != 0 {
          return Err("invalid padding.");
      }*/
    ret
}
