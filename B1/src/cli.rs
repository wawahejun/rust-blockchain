use clap::{Parser, Subcommand};
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

#[derive(Parser)]
#[clap(name = "blockchain-cli", version = "1.0", author = "Your Name")]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 创建一个新钱包
    CreateWallet,

    /// 发起一笔交易
    AddTransaction {
        sender: String,
        receiver: String,
        amount: u64,
    },

    /// 挖矿新区块
    MineBlock {
        miner: String,
    },

    /// 验证区块链
    ValidateChain,
}

impl Cli {
    pub fn run(&self) {
        match &self.command {
            Commands::CreateWallet => {
                let wallet = Wallet::new();
                println!("New wallet created!");
                println!("Wallet address: {}", wallet.address());
            }
            Commands::AddTransaction { sender, receiver, amount } => {
                let wallet = Wallet::new(); // 这里需要根据地址加载钱包
                let transaction = Transaction::new(
                    sender.clone(),
                    receiver.clone(),
                    *amount,
                    &wallet.key_pair,
                );
                println!("Transaction created: {:?}", transaction);
            }
            Commands::MineBlock { miner } => {
                let mut blockchain = Blockchain::load_from_file("blockchain.json").unwrap_or_else(|_| Blockchain::new(4));
                let transactions = vec![/* 未确认的交易 */];
                blockchain.add_block(transactions);
                println!("New block mined by miner: {}", miner);
                println!("Latest block: {:#?}", blockchain.get_latest_block());
                blockchain.save_to_file("blockchain.json").unwrap();
            }
            Commands::ValidateChain => {
                let blockchain = Blockchain::load_from_file("blockchain.json").unwrap();
                let is_valid = blockchain.is_chain_valid();
                println!("Blockchain validity: {}", is_valid);
            }
        }
    }
}
