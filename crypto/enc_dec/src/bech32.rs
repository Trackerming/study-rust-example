use crate::hex_string_to_bytes;
use std::char::ParseCharError;
use std::error::Error;

// bech32编码集
//
// Bech32编码是一种用于比特币和其他加密货币地址的编码格式，它使用了一种称为Bech32的算法。
// Bech32编码由两部分组成：人类可读的部分和数据部分。
// Bech32编码的基本原理如下：
// 准备数据：首先，准备要编码的数据。在比特币中，通常是一个公钥哈希（public key hash）或脚本哈希（script hash）。
// 添加版本前缀：将一个版本前缀添加到数据的前面。版本前缀用于指示编码后的数据的类型。
// 计算校验和：对添加了版本前缀的数据进行一系列的数学计算，包括多项式除法和 XOR 运算，以生成校验和。校验和通常是数据的一部分，并且用于检测数据的完整性。
// 拼接数据：将版本前缀、原始数据和校验和拼接在一起，形成一个整体的数据。
// 将数据分割为片段：将整体的数据分割为一系列的片段，每个片段包含特定数量的比特位。
// 进行字符转换：将每个片段转换为对应的字符，使用Bech32字符表中的字符。
// 拼接人类可读的部分和数据部分：将人类可读的部分和数据部分拼接在一起，以形成最终的Bech32编码。
const CHARSET: &'static str = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";
const ENCODING_POLYMOD_CONST: u32 = 1;

fn encode_u5(input: u8) -> char {
    CHARSET.chars().nth(input as usize).unwrap()
}

fn u8_convert_u5(input: &[u8], from: u32, to: u32, pad: bool) -> Vec<u8> {
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

fn bech32_extend_hrp(hrp: &str) -> Vec<u8> {
    let mut result = Vec::new();
    // 取字符高三位
    for ch in hrp.chars() {
        result.push(ch.to_ascii_lowercase() as u8 >> 5);
    }
    result.push(0);
    // 取字符的低5位 00011111 = 16+8+4+2+1 = 31
    for ch in hrp.chars() {
        result.push((ch.to_ascii_lowercase() as u8) & 0x1f);
    }
    result
}

/**
 * 计算校验和，具体来说，bech32_polymod 函数对输入的数据进行一系列的计算，并返回一个32位整数作为校验和。
 * 如果校验和为0，则表示数据有效；否则，表示数据无效
 */
fn bech32_polymod(values: &[u8]) -> u32 {
    let mut chk = 1u32;
    let generator: [u32; 5] = [0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3];
    let mut b;
    for v in values {
        // 取字符的高7位
        b = chk >> 25;
        // 取字符的低25位左移5bit
        // 初始数组进行异或
        chk = (chk & 0x1ffffff) << 5 ^ (*v as u32);
        // 根据chk的高7位向右循环移动5次，每次如果bit位上为1就异或0x3b6a57b2
        for i in 0..5 {
            if (b >> i & 1) == 1 {
                chk ^= generator[i];
            }
        }
    }
    chk
}

fn create_checksum(hrp: &str, data: &[u8]) -> Vec<u8> {
    let mut values = bech32_extend_hrp(hrp);
    values.extend_from_slice(data);
    // 填充6个0
    values.extend_from_slice(&[0u8; 6]);
    // 得到的checksum要与当前的
    let plm = bech32_polymod(values.as_slice()) ^ ENCODING_POLYMOD_CONST;
    let mut checksum = Vec::new();
    println!("polymod: {plm}");
    for p in 0..6 {
        checksum.push(((plm >> 5 * (5 - p)) & 0x1f) as u8);
    }
    checksum
}

fn bech32_verify_checksum(hrp: &str, data: &[u8]) -> bool {
    println!("check data: {:?}", data);
    let mut values = bech32_extend_hrp(hrp);
    values.extend_from_slice(data);
    // 判断最终校验和是否为1
    bech32_polymod(&values) == ENCODING_POLYMOD_CONST
}

/**
 * hrp： human read part
 */
pub fn bech32_encode(hrp: &str, data: &str) -> String {
    let data = hex_string_to_bytes(data);
    // 填充一个版本字节0
    let mut u5_bytes = vec![0];
    u5_bytes.extend_from_slice(u8_convert_u5(data.as_slice(), 8, 5, true).as_slice());
    let checksum = create_checksum(hrp, u5_bytes.as_slice());
    println!("checksum: {:?}", checksum);
    u5_bytes.extend_from_slice(checksum.as_slice());
    // 编码结果
    let mut result = String::new();
    result.push_str(hrp);
    println!("u5_bytes: {:?}", u5_bytes);
    assert_eq!(bech32_verify_checksum(hrp, u5_bytes.as_slice()), true);
    // 可读和数据采用1隔离
    result.push('1');
    for v in u5_bytes {
        result.push(CHARSET.chars().nth(v as usize).expect("invalid u5"));
    }
    result
}

#[cfg(test)]
mod bech32_encode_test {
    use super::*;

    #[test]
    fn test_bech32_encode() {
        let pub_key_str = "fab423b12a2f13ddb207dde536d8b183728859f1";
        let human_read_part = "bc";
        let address = bech32_encode(human_read_part, pub_key_str);
        assert_eq!(
            address,
            "bc1ql26z8vf29ufamvs8mhjndk93sdegsk03clpuh8".to_string()
        );
    }

    #[test]
    fn test_bech32_checksum() {
        let hrp = "bc";
        let data = vec![
            0, 31, 10, 26, 2, 7, 12, 9, 10, 5, 28, 9, 29, 27, 12, 16, 7, 27, 23, 18, 19, 13, 22, 5,
            17, 16, 13, 25, 8, 16, 22, 15, 17, 24, 31, 1, 28, 23, 7,
        ];
        assert_eq!(bech32_verify_checksum(hrp, data.as_slice()), true);
    }
}
