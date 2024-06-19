// SSZ（Simple Serialize）是一种用于对数据进行序列化和反序列化的格式，最初是为以太坊 2.0 中的数据结构设计的。
// 它的目标是提供一种简单、高效的序列化方法，同时保持数据的紧凑性和可扩展性。SSZ编码主要用于以太坊 2.0 中的状态转换、区块链数据存储和网络通信等场景。
//
// SSZ编码的特点和原理如下：
//
// 1. 基本类型编码：
//      对于基本数据类型（如整数、布尔值、字节序列等），采用固定长度的编码规则，直接将数据转换为字节流。
// 2. 复合类型编码：
//      对于复合数据类型（如列表、结构体等），采用递归的方式对每个成员进行编码。
//      列表类型的编码首先编码列表的长度，然后依次对列表中的每个元素进行编码。
//      结构体类型的编码按照结构体定义的顺序对每个字段进行编码。
// 3. 长度前缀编码：
//      SSZ编码中使用长度前缀来表示复合类型的长度，这种方式使得解码时能够准确地确定数据的边界。
//      对于长度不固定的列表和字节序列，使用长度前缀来表示其长度。
// 4. 变长整数编码：
//      SSZ编码中采用可变长度的整数编码来表示整数类型的数据，以提高编码的效率。
//      对于小整数，采用紧凑的编码方式，占用更少的字节。
//      对于大整数，采用可变长度的编码方式，占用更多的字节。
// 5. 哈希树编码：
//      SSZ编码中常用哈希树来表示数据的树状结构，以提高数据的访问效率和安全性。
//      对于大型数据集合，可以使用哈希树将数据分块存储，并通过哈希值来验证数据的完整性。
//
// 总的来说，SSZ编码是一种简单、高效的数据序列化方法，适用于各种数据结构的序列化和反序列化操作。
// 它在以太坊 2.0 中的应用已经得到了验证，并被广泛应用于以太坊 2.0 的核心协议中

pub enum SszValue {
    Integer(u64),
    Bytes(Vec<u8>),
    List(Vec<SszValue>),
}

pub fn encode_ssz(value: &SszValue) -> Vec<u8> {
    match value {
        SszValue::Integer(n) => {
            let mut bytes = vec![0u8; 8];
            bytes.copy_from_slice(&n.to_le_bytes());
            bytes
        }
        SszValue::Bytes(bytes) => {
            let len_bytes = encode_ssz(&SszValue::Integer(bytes.len() as u64));
            let mut result = Vec::new();
            result.extend_from_slice(&len_bytes);
            result.extend_from_slice(bytes);
            result
        }
        SszValue::List(list) => {
            let len_bytes = encode_ssz(&SszValue::Integer(list.len() as u64));
            let mut result = Vec::new();
            result.extend_from_slice(&len_bytes);
            for item in list {
                let item_bytes = encode_ssz(item);
                result.extend_from_slice(item_bytes.as_slice());
            }
            result
        }
    }
}

#[cfg(test)]
mod ssz_test {
    use super::*;

    #[test]
    fn test_ssz_encode() {
        let list = SszValue::List(vec![
            SszValue::Integer(42),
            SszValue::Bytes(b"hello".to_vec()),
        ]);
        let encode_bytes = encode_ssz(&list);
        println!("Encoded SSZ bytes: {:?}", encode_bytes);
        assert_eq!(
            encode_bytes,
            vec![
                2, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 104, 101,
                108, 108, 111
            ]
        );
    }
}
