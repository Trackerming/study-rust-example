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
        length_encode.into_iter().chain(result.into_iter()).collect()
    }
}

#[cfg(test)]
mod test_rlp {
    use super::*;

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
}
