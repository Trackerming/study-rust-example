use crypto::{digest::Digest, sha2::Sha256};

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

    pub fn compute_hash(left_hash: &[u8], right_hash: &[u8]) -> Vec<u8> {
        let mut sha256 = Box::new(Sha256::new());
        sha256.input(left_hash);
        sha256.input(right_hash);
        let mut out = vec![0u8; sha256.output_bytes()];
        sha256.result(&mut out);
        out
    }

    // 递归构建树结构
    pub fn build_tree(data: Vec<Vec<u8>>) -> Option<Box<MerkleTrieNode>> {
        if data.is_empty() {
            return None;
        }
        if data.len() == 1 {
            return Some(Box::new(MerkleTrieNode::new(data[0].clone())));
        }
        let mid = data.len() / 2;
        let left_data = data[..mid].to_vec();
        let right_data = data[mid..].to_vec();

        let left_node = MerkleTrieNode::build_tree(left_data);
        let right_node = MerkleTrieNode::build_tree(right_data);
        let hash = match (&left_node, &right_node) {
            (Some(left), Some(right)) => MerkleTrieNode::compute_hash(&left.hash, &right.hash),
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
        if let Some(node) = MerkleTrieNode::build_tree(data) {
            println!("tree: {:?}", node);
            assert_eq!(node, expected_tree);
        }
    }
}
