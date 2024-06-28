use crate::hex_string_to_bytes;
use num_integer::Integer;
use num_traits::cast::ToPrimitive;
use sha2::{Digest, Sha256};

// 编码原理：
// 准备数据：将要编码的二进制数据准备好。
//
// 添加校验和：为了增强错误检测，通常在数据的末尾添加一些校验和，以确保数据的完整性。
//
// 进行Base58编码：将数据转换为Base58表示。Base58编码的字符集包括数字（除了0和1）、大写字母（除了I和O）和小写字母（除了l）。通常使用一个特殊字符（例如Bitcoin中的 1）来表示Base58编码中的0值字节
//
// 解码原理：
// 将Base58表示转换为字节数据：将Base58表示的数据转换回原始的二进制数据。
//
// 移除校验和：如果添加了校验和，需要将校验和部分移除，以便进行后续的校验。
//
// 进行校验：对剩余的数据进行校验，确保其完整性。

const BASE58_ALPHABET: &'static [u8] =
    b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn encode(data: &[u8]) -> String {
    // 将数据转换为2进制数组并转换为大数
    let mut x = num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, data);
    // 分母设置为58，即58进制
    let radix = num_bigint::BigInt::from(58);
    let mut result = Vec::new();
    while x > num_bigint::BigInt::from(0) {
        let div_rem = x.div_rem(&radix);
        // 如果商的值大于 u32 类型的最大值，to_u32() 方法会返回 None，我们在这里使用 unwrap() 方法来获取转换后的 u32 值
        // let quotient = div_rem.0.to_u32().unwrap();
        let remainder = div_rem.1.to_u32().unwrap();
        result.push(BASE58_ALPHABET[remainder as usize]);
        x = div_rem.0;
    }
    // 如果前面为0，则添加0的编码，直至非零的参数
    for &byte in data.iter() {
        if byte != 0 {
            break;
        }
        result.push(BASE58_ALPHABET[0]);
    }
    result.reverse();
    String::from_utf8(result).unwrap()
}

pub fn decode(data: String) -> Vec<u8> {
    let mut x = num_bigint::BigInt::from(0);
    let radix = std::rc::Rc::new(num_bigint::BigInt::from(58));
    // 恢复成原始大数
    for ch in data.chars() {
        let position = BASE58_ALPHABET
            .iter()
            .position(|&c| c as char == ch)
            .unwrap();
        x = x * radix.clone().as_ref() + num_bigint::BigInt::from(position);
    }
    let (_, mut result) = x.to_bytes_be();
    // 去除leding byte
    while result.len() > 0 && result[0] == 0 {
        result.remove(0);
    }
    result
}

fn sha256_double(input: &[u8]) -> Vec<u8> {
    let mut sha256 = Sha256::new();
    sha256.update(&input[..]);
    let out = sha256.finalize().to_vec();
    let mut sha256 = Sha256::new();
    sha256.update(&out);
    let result = sha256.finalize().to_vec();
    result
}

pub fn generate_btc_address(pub_key_hash: &str) -> String {
    let mut address_bytes: Vec<u8> = Vec::new();
    // leading byte 0
    address_bytes.push(0x00);
    address_bytes.append(&mut hex_string_to_bytes(pub_key_hash));
    // 计算校验和
    let mut checksum = sha256_double(address_bytes.as_slice())[..4].to_vec();
    address_bytes.append(&mut checksum);
    assert_eq!(address_bytes.len(), 25);
    // 进行编码
    encode(address_bytes.as_slice())
}

#[cfg(test)]
mod base58_tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let data = b"Hello world";
        let encoded_str = encode(data);
        assert_eq!(encoded_str, "JxF12TrwXzT5jvT".to_string());
        println!("Encoded data: {}", encoded_str);
        let decoded = decode(encoded_str);
        println!("Decoded data: {:?}", String::from_utf8_lossy(&decoded));
        assert_eq!(decoded, data.to_vec());
    }

    #[test]
    fn test_btc_address_encode() {
        let pub_key_hash = "37388911d9c9be6aa8f86d49c118bf34b63285e4";
        let address = generate_btc_address(pub_key_hash);
        println!("address: {}", address);
        assert_eq!(address, "162ythfZpJb4ubNyYzADVPPQs5kSDKSVeU".to_string());
    }
}
