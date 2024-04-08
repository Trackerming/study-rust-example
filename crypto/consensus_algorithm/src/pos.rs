use crate::pow;
use rand::{thread_rng, Rng, SeedableRng};

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    balance: u64,
    // 0~1000，代表节点的诚信因子，实际应该考虑到节点的运行时间、历史的诚信记录等，值越大越值得信任，被选中的概率越大
    honesty_factor: u64,
}

impl Node {
    pub fn new(balance: u64, honesty_factor: u64) -> Self {
        Node {
            balance,
            honesty_factor,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    hash: Option<String>,
    transactions: Vec<String>,
    prev_hash: String,
    validator: Option<Node>,
}

impl Block {
    pub fn new(transactions: Vec<String>, prev_hash: String) -> Block {
        Block {
            transactions,
            prev_hash,
            hash: None,
            validator: None,
        }
    }

    pub fn ser(&self) -> String {
        format!(
            "{:?}{:?}{:?}",
            self.transactions, self.prev_hash, self.validator
        )
    }

    pub fn add_to_chain(&mut self, validator: Node) {
        // validator 验证交易并做出有效签名
        self.transactions
            .insert(0, "coinbase to validator".to_owned());
        self.validator = Some(validator.clone());
        self.hash = Option::from(pow::double_hash(self.ser()));
    }
}

pub struct BlockChain {
    blocks: Vec<Block>,
    nodes: Vec<Node>,
}

impl BlockChain {
    pub fn new(current_blocks: Vec<Block>, nodes: Vec<Node>) -> Self {
        BlockChain {
            blocks: current_blocks,
            nodes,
        }
    }

    pub fn select_validator(&self) -> Node {
        let mut rng = thread_rng();
        let total_weight = self
            .nodes
            .iter()
            .map(|node| node.balance * node.honesty_factor)
            .sum();
        let mut cumulative_weight = 0;
        let mut selected_node = self.nodes[0].clone();
        let random_weight = rng.gen_range(0..=total_weight);
        for node in self.nodes.iter() {
            cumulative_weight += node.balance * node.honesty_factor;
            if cumulative_weight >= random_weight {
                selected_node = node.clone();
                break;
            }
        }
        selected_node
    }

    pub fn add_block(&mut self, transactions: Vec<String>) {
        // 打包block
        let mut new_block = Block::new(
            transactions,
            self.blocks.last().unwrap().clone().hash.unwrap(),
        );
        // 选择validator
        let validator = self.select_validator();
        new_block.add_to_chain(validator);
        println!("added block: {:?}", new_block);
        self.blocks.push(new_block);
        println!("blockchain: {:?}", self.blocks);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_block_chain() {
        let ancestor = Block {
            hash: Some(
                "0000000000000000000000000000000000000000000000000000000000000001".to_owned(),
            ),
            transactions: Vec::new(),
            prev_hash: "0000000000000000000000000000000000000000000000000000000000000000"
                .to_owned(),
            validator: None,
        };
        let node1 = Node::new(100, 50);
        let node2 = Node::new(200, 30);
        let node3 = Node::new(300, 70);
        let node4 = Node::new(800, 10);
        let mut nodes = Vec::new();
        nodes.push(node1);
        nodes.push(node2);
        nodes.push(node3);
        nodes.push(node4);
        let mut blockchain = BlockChain::new(vec![ancestor], nodes);
        blockchain.add_block(vec!["tx1".to_owned(), "tx2".to_owned()]);
    }
}
