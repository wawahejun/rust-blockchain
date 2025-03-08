use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write, Read};
use serde_json;
use chrono::Utc;
use crate::block::Block;
use crate::transaction::Transaction;
use crate::smart_contract::SmartContract;
use crate::privacy::PrivacyTransaction;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub contracts: HashMap<String, SmartContract>, // 存储智能合约
    pub privacy_transactions: Vec<PrivacyTransaction>, // 存储隐私交易
    pub balances: HashMap<String, u64>, // 存储地址余额
}

impl Blockchain {
    // 创建一个新的区块链
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty,
            contracts: HashMap::new(),
            privacy_transactions: Vec::new(),
            balances: HashMap::new(),
        };
        // 创建创世区块
        let genesis_block = Block::new(0, Utc::now().timestamp(), Vec::new(), "0".to_string());
        blockchain.chain.push(genesis_block);
        blockchain
    }

    // 获取地址的余额
    pub fn get_balance(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }

    // 设置地址的余额
    pub fn set_balance(&mut self, address: &str, balance: u64) {
        self.balances.insert(address.to_string(), balance);
    }

    // 获取最新区块
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    // 添加新区块
    pub fn add_block(&mut self, transactions: Vec<Transaction>, miner: &str) {
        // 检查交易的有效性
        for tx in &transactions {
            if !self.is_transaction_valid(&tx) {
                panic!("Invalid transaction: {:?}", tx);
            }
        }

        // 更新余额
        for tx in &transactions {
            self.update_balances(&tx);
        }

        // 发放挖矿奖励
        let mining_reward = 50; // 挖矿奖励金额
        let reward_transaction = Transaction {
            sender: "0".to_string(), // 系统地址
            receiver: miner.to_string(),
            amount: mining_reward,
            signature: "mining_reward".to_string(), // 挖矿奖励不需要签名
        };
        self.update_balances(&reward_transaction);

        // 创建新区块
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

    // 检查交易的有效性
    fn is_transaction_valid(&self, tx: &Transaction) -> bool {
        let sender_balance = self.get_balance(&tx.sender);
        sender_balance >= tx.amount
    }

    // 更新余额
    fn update_balances(&mut self, tx: &Transaction) {
        let sender_balance = self.get_balance(&tx.sender);
        let receiver_balance = self.get_balance(&tx.receiver);
        self.set_balance(&tx.sender, sender_balance - tx.amount);
        self.set_balance(&tx.receiver, receiver_balance + tx.amount);
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

    // 部署智能合约
    pub fn deploy_contract(&mut self, contract_id: String, code: String) {
        let contract = SmartContract::new(code);
        self.contracts.insert(contract_id, contract);
    }

    // 执行智能合约
    pub fn execute_contract(&mut self, contract_id: &str, method: &str, args: Vec<String>) -> Result<String, String> {
        if let Some(contract) = self.contracts.get_mut(contract_id) {
            contract.execute(method, args)
        } else {
            Err("Contract not found".to_string())
        }
    }

    // 添加隐私交易
    pub fn add_privacy_transaction(&mut self, transaction: PrivacyTransaction) {
        if transaction.verify() {
            self.privacy_transactions.push(transaction);
        } else {
            println!("Invalid privacy transaction");
        }
    }
}
