use sha2::{Sha256, Digest};
use std::fmt;

#[derive(Debug)]
pub struct MerkleTree {
    root: Option<MerkleNode>,
}

#[derive(Debug, Clone)]
struct MerkleNode {
    hash: Vec<u8>,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
}

impl MerkleTree {
    // 创建一个新的默克尔树
    pub fn new(transactions: Vec<String>) -> Self {
        if transactions.is_empty() {
            // 如果交易列表为空，返回一个默认的根哈希
            let default_hash = Sha256::digest(b"empty").to_vec();
            return MerkleTree {
                root: Some(MerkleNode {
                    hash: default_hash,
                    left: None,
                    right: None,
                }),
            };
        }

        let mut nodes = transactions
            .into_iter()
            .map(|tx| {
                let hash = Self::hash_leaf(&tx);
                MerkleNode {
                    hash,
                    left: None,
                    right: None,
                }
            })
            .collect::<Vec<_>>();

        while nodes.len() > 1 {
            let mut new_level = Vec::new();
            for chunk in nodes.chunks(2) {
                let left = &chunk[0];
                let right = if chunk.len() > 1 { &chunk[1] } else { &chunk[0] };
                let hash = Self::hash_nodes(&left.hash, &right.hash);
                new_level.push(MerkleNode {
                    hash,
                    left: Some(Box::new(left.clone())),
                    right: Some(Box::new(right.clone())),
                });
            }
            nodes = new_level;
        }

        MerkleTree {
            root: Some(nodes.remove(0)),
        }
    }

    // 获取默克尔树的根哈希
    pub fn root_hash(&self) -> Option<Vec<u8>> {
        self.root.as_ref().map(|node| node.hash.clone())
    }

    // 计算叶子节点的哈希
    fn hash_leaf(data: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hasher.finalize().to_vec()
    }

    // 计算两个节点的哈希
    fn hash_nodes(left: &[u8], right: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(left);
        hasher.update(right);
        hasher.finalize().to_vec()
    }
}

impl fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(root) = &self.root {
            write!(f, "MerkleTree {{ root: {} }}", hex::encode(&root.hash))
        } else {
            write!(f, "MerkleTree {{ root: None }}")
        }
    }
}
