use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use crate::merkle_tree::MerkleTree;
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub merkle_root: String, // 默克尔树的根哈希
}

impl Block {
    // 创建一个新区块
    pub fn new(index: u64, timestamp: i64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let merkle_tree = MerkleTree::new(transactions.iter().map(|tx| tx.to_string()).collect());
        let merkle_root = hex::encode(merkle_tree.root_hash().unwrap());

        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            merkle_root,
        };
        block.hash = block.calculate_hash();
        block
    }

    // 计算区块的哈希值
    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.merkle_root,
            self.transactions.len(),
            self.previous_hash,
            self.nonce
        );
        let mut hasher = Sha256::new(); 
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }
    

    // 挖矿（PoW）
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}
