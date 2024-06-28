use sha2::{Digest, Sha256};

#[derive(Debug, PartialEq)]
pub struct MerkleTrieNode {
    left: Option<Box<MerkleTrieNode>>,
    right: Option<Box<MerkleTrieNode>>,
    hash: Vec<u8>,
}

impl MerkleTrieNode {
    pub fn new(hash: Vec<u8>) -> Self {
        MerkleTrieNode {
            left: None,
            right: None,
            hash,
        }
    }

    pub fn compute_hash(left_hash: &[u8], right_hash: &[u8], is_btc: bool) -> Vec<u8> {
        let mut sha256 = Sha256::new();
        sha256.update(left_hash);
        sha256.update(right_hash);
        let out = sha256.finalize().to_vec();
        if is_btc {
            let mut sha256 = Sha256::new();
            sha256.update(&out);
            let result = sha256.finalize().to_vec();
            result
        } else {
            out
        }
    }

    // 递归构建树结构
    pub fn build_tree(data: Vec<Vec<u8>>, is_btc: bool) -> Option<Box<MerkleTrieNode>> {
        if data.is_empty() {
            return None;
        }
        if data.len() == 1 {
            return Some(Box::new(MerkleTrieNode::new(data[0].clone())));
        }
        let mid = data.len() / 2;
        let left_data = data[..mid].to_vec();
        let right_data = data[mid..].to_vec();
        println!(
            "mid: {:?}, left: {:?} right: {:?}",
            mid, left_data, right_data
        );

        let left_node = MerkleTrieNode::build_tree(left_data, is_btc);
        let right_node = MerkleTrieNode::build_tree(right_data, is_btc);
        println!(
            "mid: {:?}, left: {:?} right: {:?}",
            mid, left_node, right_node
        );
        let hash = match (&left_node, &right_node) {
            (Some(left), Some(right)) => {
                MerkleTrieNode::compute_hash(&left.hash, &right.hash, is_btc)
            }
            (Some(node), None) | (None, Some(node)) => node.hash.clone(),
            _ => Vec::new(),
        };
        Some(Box::new(MerkleTrieNode {
            left: left_node,
            right: right_node,
            hash,
        }))
    }
}

#[cfg(test)]
mod merkle_tree_test {
    use super::*;

    #[test]
    fn test_compute_tree_hash() {
        let data = vec![b"data1".to_vec(), b"merkle".to_vec(), b"tx_hash".to_vec()];
        let expected_tree = Box::new(MerkleTrieNode {
            left: Some(Box::new(MerkleTrieNode {
                left: None,
                right: None,
                hash: vec![100, 97, 116, 97, 49],
            })),
            right: Some(Box::new(MerkleTrieNode {
                left: Some(Box::new(MerkleTrieNode {
                    left: None,
                    right: None,
                    hash: vec![109, 101, 114, 107, 108, 101],
                })),
                right: Some(Box::new(MerkleTrieNode {
                    left: None,
                    right: None,
                    hash: vec![116, 120, 95, 104, 97, 115, 104],
                })),
                hash: vec![
                    72, 201, 49, 75, 184, 35, 7, 31, 16, 4, 116, 179, 208, 182, 191, 214, 236, 122,
                    235, 217, 76, 211, 143, 109, 157, 74, 66, 69, 207, 131, 65, 48,
                ],
            })),
            hash: vec![
                18, 132, 214, 109, 189, 213, 244, 223, 247, 188, 36, 86, 141, 117, 32, 155, 44,
                212, 125, 34, 29, 147, 84, 180, 118, 6, 161, 131, 148, 187, 199, 235,
            ],
        });
        if let Some(node) = MerkleTrieNode::build_tree(data, false) {
            println!("tree: {:?}", node);
            assert_eq!(node, expected_tree);
        }
    }

    /*    #[test]
    fn test_btc_block_120008() {
        let blocks = vec![
            // 1
            enc_dec::hex_string_to_bytes("c338a4bc83d859feec4f9fab9bea5d5cdfa6d80804d026649f9e08e28a1126f5"),
            // 2
            enc_dec::hex_string_to_bytes("31009eb60976f1daa55da93c2d48ab3bb3f183721b376bb4bf6966e6768ddadf"),
            // 3
            enc_dec::hex_string_to_bytes("50ce49e1f542dc74046b14757c5b03b127802d7e01d8662612e417d6d7edc5ae"),
            // 4
            enc_dec::hex_string_to_bytes("2973971ee8dfd69a44954ac35150a65900a871f9402ab60a63708d2774d125cc"),
            // 5
            enc_dec::hex_string_to_bytes("08edefb2eec45dfc9e6228b34988c36d25e2dc5f384a4c3dea2bf5fdb26aefd4"),
            // 6
            enc_dec::hex_string_to_bytes("da5423baae6eeefa3535a8fb05304214b556b9ceed699316a9dd84b00f685326"),
        ];
        if let Some(node) = MerkleTrieNode::build_tree(blocks, true) {
            println!("tree: {:?}", node);
        }
    }*/
}
