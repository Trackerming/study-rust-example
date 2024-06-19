/// 以太坊的存储结构采用了 Merkle Patricia Trie（或称为 Trie 树）来组织数据，这是一种经过修改的 Merkle 树，旨在提供高效的数据检索和验证。
/// 以太坊使用 Trie 树来存储账户状态、合约代码、存储空间以及其他数据
/// Merkle Patricia Trie 是一种前缀树，其中的每个节点包含了一个键值对，键是一个 256 位的 Keccak256 哈希值，值是任意长度的字节序列。
/// Trie 树的节点分为四种类型：扩展节点、叶子节点、空节点和分支节点。每个节点都通过哈希值来标识
/// 1. 叶子节点（Leaf Node）：叶子节点包含一个键值对，其中键是一个 256 位的 Keccak256 哈希值，值是需要存储的数据。
///
/// 2. 扩展节点（Extension Node）：扩展节点包含一个共同的前缀和一个子节点的哈希值，用于压缩相邻叶子节点中共同的前缀。
///
/// 3. 空节点（Empty Node）：空节点表示不存在的数据，可以节省存储空间。
///
/// 4. 分支节点（Branch Node）：分支节点包含一个长度为 16 的子节点数组和一个可选的值，用于存储哈希后的子节点和部分数据
/// 以太坊的 Trie 树是一棵三层的树，包括根节点、中间节点和叶子节点。
/// 根节点是存储 Trie 树的根哈希值的节点，中间节点包含了根节点和叶子节点之间的路径，叶子节点存储实际的数据
/// 使用 Trie 树的好处是可以实现高效的数据检索和验证。通过哈希值的结构，可以快速确定数据是否存在，并且可以轻松验证数据的完整性和一致性。
/// 这使得以太坊可以高效地存储和管理大量的数据，同时确保数据的安全性和可靠性

#[derive(Clone)]
pub enum TrieNode {
    Leaf(Vec<u8>),
    Extension(Vec<u8>, Box<TrieNode>),
    Branch(Vec<Option<Box<TrieNode>>>, Option<Vec<u8>>),
    Empty,
}

impl TrieNode {
    pub fn new_leaf(data: Vec<u8>) -> Self {
        TrieNode::Leaf(data)
    }

    pub fn new_extension(prefix: Vec<u8>, child: Box<TrieNode>) -> Self {
        TrieNode::Extension(prefix, child)
    }

    pub fn new_branch(children: Vec<Option<Box<TrieNode>>>, value: Option<Vec<u8>>) -> Self {
        TrieNode::Branch(children, value)
    }

    pub fn new_empty() -> Self {
        TrieNode::Empty
    }
}

pub struct MerklePatriciaTrie {
    root: TrieNode,
}

impl MerklePatriciaTrie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new_empty(),
        }
    }

    pub fn insert(&mut self, key: &[u8], value: Vec<u8>) {
        let mut key_iter = key.iter();
        self.root = self.insert_recursive(&mut key_iter, value, &self.root);
    }

    // 递归将键值对插入到trie中
    pub fn insert_recursive(
        &self,
        key_iter: &mut std::slice::Iter<u8>,
        value: Vec<u8>,
        node: &TrieNode,
    ) -> TrieNode {
        match node {
            TrieNode::Leaf(_) => {
                // 当前节点为叶子节点，创建一个新的扩展节点
                let mut new_leaf_key = vec![];
                new_leaf_key.extend(key_iter.clone());
                TrieNode::new_branch(
                    vec![
                        Some(Box::new(TrieNode::new_empty())),
                        Some(Box::new(TrieNode::new_leaf(value))),
                    ],
                    Some(new_leaf_key),
                )
            }
            TrieNode::Branch(children, _) => {
                // 当前节点为分支节点
                let key_byte = key_iter.next().unwrap();
                match &children[*key_byte as usize] {
                    // 子节点存在，继续递归插入
                    Some(_child) => TrieNode::new_branch(
                        children
                            .iter()
                            .enumerate()
                            .map(|(i, &ref c)| {
                                if i == *key_byte as usize {
                                    Some(Box::new(self.insert_recursive(
                                        key_iter,
                                        value.clone(),
                                        c.as_deref().unwrap(),
                                    )))
                                } else {
                                    c.clone()
                                }
                            })
                            .collect(),
                        None,
                    ),
                    None => {
                        // 子节点不存在，创造新的叶子节点
                        let mut new_branch_children = children.clone();
                        new_branch_children[*key_byte as usize] =
                            Some(Box::new(TrieNode::new_leaf(value)));
                        TrieNode::new_branch(new_branch_children.into(), None)
                    }
                }
            }
            TrieNode::Extension(prefix, child) => {
                // 当前节点为扩展节点
                let common_prefix = Self::common_prefix_length(&prefix, key_iter);
                if common_prefix == prefix.len() {
                    // 插入键与扩展节点的前缀相同，继续递归插入
                    TrieNode::new_extension(
                        prefix.clone(),
                        Box::new(self.insert_recursive(key_iter, value, child)),
                    )
                } else {
                    // 分裂当前节点，创建新的分支节点
                    let mut new_branch_children = vec![None; 17];
                    new_branch_children[prefix[0] as usize] = Some(Box::new(
                        TrieNode::new_extension(prefix[1..].to_vec(), child.clone().into()),
                    ));
                    new_branch_children[key_iter.next().unwrap().clone() as usize] =
                        Some(Box::new(TrieNode::new_leaf(value)));
                    TrieNode::new_branch(new_branch_children, None)
                }
            }
            TrieNode::Empty => {
                // 当前节点为空节点，创建新的叶子节点
                TrieNode::new_leaf(value)
            }
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let mut key_iter = key.iter();
        self.get_recursive(&mut key_iter, &self.root)
    }

    pub fn get_recursive(
        &self,
        key_iter: &mut std::slice::Iter<u8>,
        node: &TrieNode,
    ) -> Option<Vec<u8>> {
        match node {
            TrieNode::Leaf(data) => {
                // 当前节点为叶子节点，返回值
                Some(data.clone())
            }
            TrieNode::Extension(prefix, child) => {
                // 当前节点为扩展节点
                let common_prefix = Self::common_prefix_length(&prefix, key_iter);
                if common_prefix == prefix.len() {
                    // 插入键与扩展节点的前缀相同，继续递归检索
                    self.get_recursive(key_iter, child)
                } else {
                    // 键不匹配，返回空值
                    None
                }
            }
            TrieNode::Branch(children, _) => {
                // 当前节点为分支节点
                let key_byte = key_iter.next().unwrap();
                match &children[*key_byte as usize] {
                    Some(child) => {
                        // 子节点存在，继续递归检索
                        self.get_recursive(key_iter, &*child)
                    }
                    None => None,
                }
            }
            TrieNode::Empty => {
                // 当前节点为空节点，返回空值
                None
            }
        }
    }

    fn common_prefix_length(a: &[u8], b: &mut std::slice::Iter<u8>) -> usize {
        a.iter().zip(b).take_while(|&(x, y)| x == y).count()
    }
}
