use crate::block::Block;
use crate::transaction::Transaction; 
use chrono::Utc;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Write, Read};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}


impl Blockchain {
    // 创建创世区块
    pub fn create_genesis_block() -> Block {
        Block::new(0, Utc::now().timestamp(), Vec::new(), "0".to_string())
    }

    // 初始化区块链
    pub fn new(difficulty: usize) -> Self {
        let genesis_block = Self::create_genesis_block();
        Blockchain {
            chain: vec![genesis_block],
            difficulty,
        }
    }

    // 获取最新区块
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    // 添加新区块
    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let latest_block = self.get_latest_block();
        let mut new_block = Block::new(
            latest_block.index + 1,
            Utc::now().timestamp(),
            transactions,
            latest_block.hash.clone(),
        );
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }

    // 验证区块链的完整性
    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // 检查当前区块的哈希是否正确
            if current_block.hash != current_block.calculate_hash() {
                println!("Invalid hash for block {}", current_block.index);
                return false;
            }

            // 检查前一个区块的哈希是否匹配
            if current_block.previous_hash != previous_block.hash {
                println!("Invalid previous hash for block {}", current_block.index);
                return false;
            }
        }
        true
    }

    // 将区块链保存到文件
    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let data = serde_json::to_string(self)?;
        let mut file = File::create(filename)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    // 从文件加载区块链
    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let blockchain: Blockchain = serde_json::from_str(&data)?;
        Ok(blockchain)
    }
}
