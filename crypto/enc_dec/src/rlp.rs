// RLP（Recursive Length Prefix）是一种用于对数据进行编码的格式，最初是为以太坊中的智能合约和交易设计的。
// 它的设计目的是将任意复杂的数据结构编码为字节流，同时保持数据结构的层次性
// 原理如下
// 1. 基本类型编码：
//
//  对于单个字节，如果其值在 0 到 127 之间（包括 0 和 127），则它就是一个基本类型，其编码就是该字节本身。
//  如果字节的值大于等于 128，则它是一个长类型，需要用更多的字节来表示其长度和内容
// 2. 长类型编码：
//  对于长类型，首先需要编码其长度。
//  如果长度为 0 到 55 之间，使用一个单字节表示长度，并将其值加上 128，得到的结果就是长度的编码。
//  如果长度大于 55，则需要用更多的字节来表示长度的长度。这部分长度的编码的规则是：对长度进行分解，得到其二进制表示，然后将表示长度所需的字节数加上 183，得到的结果作为长度的编码。
//  长度编码后，接着编码数据内容。
// 3. 递归编码：
//  如果数据是一个列表，那么需要递归地对列表中的每个元素进行编码。
//  对于每个元素，先编码其内容，然后根据内容的长度编码规则编码其长度。
//  将每个元素的长度编码和内容编码连接起来，就得到了列表的编码。
// 具体关于rlp的定义，详细见：https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/

use crate::read_u8;
use crate::rlp::DecodeResult::List;
use std::io::{Cursor, Read};
use std::ops::DerefMut;

pub struct RLP {}

impl RLP {
    pub fn encode_length(len: usize, offset: usize) -> Vec<u8> {
        if len < 56 {
            vec![(offset + len) as u8]
        } else {
            // 递归存储长度
            let encode_len = Self::encode_length(len / 256, offset + 55);
            let mut result = vec![encode_len.len() as u8 + offset as u8 + 55];
            result.extend_from_slice(&encode_len);
            result
        }
    }

    pub fn encode_item(input: &[u8]) -> Vec<u8> {
        let len = input.len();
        // 0到127之间编码就是自身
        if len == 1 && input[0] < 128 {
            input.to_vec()
        } else {
            // 先编码长度，再添加自身的数据
            let length_encode = Self::encode_length(input.len(), 128);
            let mut result = length_encode.clone();
            result.extend_from_slice(input);
            result
        }
    }

    pub fn encode_list(input: Vec<Vec<u8>>) -> Vec<u8> {
        let mut result = Vec::new();
        for item in input {
            let encode_item = Self::encode_item(item.as_slice());
            result.extend(encode_item);
        }
        let len = result.len();
        let length_encode = Self::encode_length(len, 192);
        length_encode
            .into_iter()
            .chain(result.into_iter())
            .collect()
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
enum DecodeResult {
    Single(Vec<u8>),
    Multi(Vec<Vec<u8>>),
    List(Box<Vec<DecodeResult>>),
}

pub fn decode(mut cursor: &mut Cursor<&[u8]>) -> Result<DecodeResult, &'static str> {
    // 单字节数据
    let first_byte = read_u8(cursor.deref_mut()).unwrap();
    if first_byte <= 0x7f {
        return Ok(DecodeResult::Single(vec![first_byte]));
    }
    // 如果第一个数据在[0x80, 0xb7]范围内，表示是一个短字符串
    if first_byte <= 0xb7 {
        let length = (first_byte - 0x80) as usize;
        // 检查数据是否包含足够的字节
        if cursor.remaining_slice().len() < length {
            return Err("Invalid RLP data.");
        }
        let mut buf = vec![0; length];
        cursor.read_exact(&mut buf).expect("read short str.");
        return Ok(DecodeResult::Single(buf));
    }
    // 如果数据的第一个字节的值在 [0xb8, 0xbf] 范围内，表示它是一个长字符串
    if first_byte <= 0xbf {
        let length = (first_byte - 0xb7) as usize;
        // 检查数据是否包含足够的字节
        if cursor.remaining_slice().len() < length {
            return Err("Invalid RLP data.");
        }
        let mut len_bytes = vec![0; length];
        cursor
            .read_exact(&mut len_bytes)
            .expect("read long str length bytes");
        let len = bytes_to_usize(&len_bytes);
        if cursor.remaining_slice().len() < len {
            return Err("Invalid RLP data.");
        }
        let mut data_bytes = vec![0; len];
        cursor
            .read_exact(&mut data_bytes)
            .expect("read long str data");
        return Ok(DecodeResult::Single(data_bytes));
    }
    if first_byte <= 0xf7 {
        let length = (first_byte - 0xc0) as usize;
        // 检查数据是否包含足够的字节
        if cursor.remaining_slice().len() < length {
            return Err("Invalid RLP data.");
        }
        let mut bytes = vec![0; length];
        cursor.read_exact(&mut bytes).expect("read short list");
        let result = decoder(bytes.as_slice()).unwrap();
        return Ok(List(Box::new(result)));
    }
    if first_byte <= 0xff {
        let list_length_bytes_len = (first_byte - 0xf7) as usize;
        // 检查数据是否包含足够的字节
        if cursor.remaining_slice().len() < list_length_bytes_len {
            return Err("Invalid RLP data.");
        }
        let mut bytes = vec![0; list_length_bytes_len];
        cursor.read_exact(&mut bytes).expect("read list length.");
        let data_len = bytes_to_usize(&bytes);
        if cursor.remaining_slice().len() < data_len {
            return Err("Invalid RLP data.");
        }
        let mut data_bytes = vec![0; data_len];
        cursor
            .read_exact(data_bytes.as_mut_slice())
            .expect("read long list.");
        let result = decoder(data_bytes.as_slice()).unwrap();
        return Ok(List(Box::new(result)));
    }
    return Err("Invalid RLP data");
}

pub fn bytes_to_usize(data: &[u8]) -> usize {
    let mut result = 0usize;
    for &byte in data {
        // 为什么这里左移动8位会overflow？因为左移动拓宽了数据的范围，与rust的操作理念不符合
        result = result * 256 + (byte as usize);
    }
    result
}

pub fn decoder(data: &[u8]) -> Result<Vec<DecodeResult>, &'static str> {
    let mut cursor = Cursor::new(data);
    let mut result = vec![];
    while !cursor.is_empty() {
        result.push(decode(&mut cursor).unwrap());
    }
    Ok(result)
}

#[cfg(test)]
mod test_rlp {
    use super::*;
    use crate::hex_string_to_bytes;
    use crate::rlp::DecodeResult::Single;

    #[test]
    fn test_encode_item() {
        let encode_item_1 = RLP::encode_item(&[48]);
        let encode_item_2 = RLP::encode_item(&[72]);
        println!("{:?}", encode_item_1);
        println!("{:?}", encode_item_2);
        assert_eq!(encode_item_1, vec![48]);
        assert_eq!(encode_item_2, vec![72]);
    }

    #[test]
    fn test_encode_list() {
        let encode_tx = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let enc = RLP::encode_list(encode_tx);
        println!("{:?}", enc);
        assert_eq!(enc, vec![200, 131, 1, 2, 3, 131, 4, 5, 6])
    }

    #[test]
    fn test_decode() {
        // RLP编码的字节数组
        let rlp_data: [u8; 6] = [0x83, b'c', b'a', b't', 0x01, 0x02];
        let data = decode(&mut Cursor::new(&rlp_data)).unwrap();
        println!("decode data: {:?}", data);
        // 因为目前还没有考虑递归循环处理，所以只是解码出第一组cat的u8数组
        assert_eq!(data, Single(vec![99, 97, 116]));
        let result = decoder(&rlp_data).unwrap();
        println!("decoder data: {:?}", result);
        assert_eq!(
            result,
            vec![Single(vec![99, 97, 116]), Single(vec![1]), Single(vec![2])]
        );
    }

    #[test]
    fn test_eth_decode() {
        let mut raw_tx = "0x02f8b1018084951e475b85177a3bb47a8301725e94dac17f958d2ee523a2206206994597c13d831ec780b844a9059cbb0000000000000000000000000b8c245fb6d5afaecc836e11602d41b85cd1eca20000000000000000000000000000000000000000000000000000000040414a9cc001a00942374401459aeb1f0606e90cb2dbbd615b6815cac40ae8c7e311197d251ac6a03ec0270e656e5b7835941b565d7d75c5f1ae6cc505dc900e2244e881fd78ee0d";
        if raw_tx.starts_with("0x") {
            raw_tx = &raw_tx[2..];
        }
        let bytes = hex_string_to_bytes(raw_tx);
        let result = decoder(bytes.as_slice()).unwrap();
        println!("raw tx decode result: {:?}", result);
        // EIP1559交易编码格式：0x02 || rlp([chain_id, nonce, max_priority_fee_per_gas, max_fee_per_gas, gas_limit, destination, amount, data, access_list, signature_y_parity, signature_r, signature_s])
        let expected_result = vec![
            Single(vec![2]),
            List(Box::new(vec![
                Single(vec![1]),                     // chain_id
                Single(vec![]),                      // nonce
                Single(vec![149, 30, 71, 91]),       // max_priority_fee_per_gas
                Single(vec![23, 122, 59, 180, 122]), // max_fee_per_gas
                Single(vec![1, 114, 94]),            // gas_limit
                Single(vec![
                    218, 193, 127, 149, 141, 46, 229, 35, 162, 32, 98, 6, 153, 69, 151, 193, 61,
                    131, 30, 199,
                ]), // destination
                Single(vec![]),                      // amount
                Single(vec![
                    169, 5, 156, 187, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 140, 36, 95, 182,
                    213, 175, 174, 204, 131, 110, 17, 96, 45, 65, 184, 92, 209, 236, 162, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64,
                    65, 74, 156,
                ]), // call_data
                List(Box::new(vec![])),              // access_list
                Single(vec![1]),                     // signature_y_parity
                Single(vec![
                    9, 66, 55, 68, 1, 69, 154, 235, 31, 6, 6, 233, 12, 178, 219, 189, 97, 91, 104,
                    21, 202, 196, 10, 232, 199, 227, 17, 25, 125, 37, 26, 198,
                ]), // signature_r
                Single(vec![
                    62, 192, 39, 14, 101, 110, 91, 120, 53, 148, 27, 86, 93, 125, 117, 197, 241,
                    174, 108, 197, 5, 220, 144, 14, 34, 68, 232, 129, 253, 120, 238, 13,
                ]), // signature_s
            ])),
        ];
        assert_eq!(result, expected_result);
    }
}
