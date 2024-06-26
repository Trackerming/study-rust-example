use sha2::{Digest, Sha256};
use std::hash::Hash;
// use sha2::digest::{DynDigest, Update};

#[derive(Debug, PartialEq)]
pub struct Block {
    data: String,
    prev_hash: String,
    nonce: u64,
}

pub fn double_hash(data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let out = hasher.finalize().to_vec();
    let mut hasher = Sha256::new();
    hasher.update(&out);
    let result = hasher.finalize().to_vec();
    result
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

impl Block {
    pub fn new(data: String, prev_hash: String) -> Self {
        Block {
            data,
            prev_hash,
            nonce: 0,
        }
    }

    pub fn calculate_hash(&self) -> String {
        // 序列化之后计算hash
        let input = format!("{}{}{}", self.data, self.prev_hash, self.nonce);
        double_hash(input)
    }

    pub fn mine_block(&mut self, difficulty: u32) -> Block {
        let target = vec![0u8; (difficulty >> 1) as usize];
        let mut target_prefix = target
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        if difficulty % 2 != 0 {
            target_prefix.push('0');
        }
        loop {
            self.nonce += 1;
            let hash = self.calculate_hash();
            if hash.starts_with(&target_prefix) {
                println!("Block mined: {}", hash);
                break;
            }
        }
        Block {
            data: self.data.clone(),
            prev_hash: self.prev_hash.clone(),
            nonce: self.nonce,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mine_block() {
        let mut block_new = Block::new(
            "hello blockchain...".to_owned(),
            "0000000000000000000000000000000000000000000000000000000000000000".to_owned(),
        );
        let mined_block = block_new.mine_block(4);
        println!("mine block for difficulty 4: {:?}", mined_block);
        let mut expected_block = Block::new(block_new.data.clone(), block_new.prev_hash.clone());
        expected_block.nonce = 29066;
        assert_eq!(mined_block, expected_block);
        // 重新从0计算
        block_new.nonce = 0;
        let mined_block = block_new.mine_block(3);
        expected_block.nonce = 90;
        println!("mine block for difficulty 3: {:?}", mined_block);
        assert_eq!(mined_block, expected_block);
    }
}
