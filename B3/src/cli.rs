use clap::{Parser, Subcommand};
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::wallet::Wallet;
use crate::privacy::PrivacyTransaction;

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

    /// 部署智能合约
    DeployContract {
        contract_id: String,
        code: String,
    },

    /// 执行智能合约
    ExecuteContract {
        contract_id: String,
        method: String,
        args: Vec<String>,
    },

    /// 创建隐私交易
    CreatePrivacyTransaction {
        amount: u64,
    },

    /// 查询地址余额
    GetBalance {
        address: String,
    },

    /// 设置地址余额
    SetBalance {
        address: String,
        balance: u64,
    },
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
                blockchain.add_block(transactions, miner);
                println!("New block mined by miner: {}", miner);
                println!("Latest block: {:#?}", blockchain.get_latest_block());
                blockchain.save_to_file("blockchain.json").unwrap();
            }
            Commands::ValidateChain => {
                let blockchain = Blockchain::load_from_file("blockchain.json").unwrap();
                let is_valid = blockchain.is_chain_valid();
                println!("Blockchain validity: {}", is_valid);
            }
            Commands::DeployContract { contract_id, code } => {
                let mut blockchain = Blockchain::load_from_file("blockchain.json").unwrap_or_else(|_| Blockchain::new(4));
                blockchain.deploy_contract(contract_id.clone(), code.clone());
                println!("Contract deployed: {}", contract_id);
                blockchain.save_to_file("blockchain.json").unwrap();
            }
            Commands::ExecuteContract { contract_id, method, args } => {
                let mut blockchain = Blockchain::load_from_file("blockchain.json").unwrap();
                match blockchain.execute_contract(&contract_id, &method, args.clone()) {
                    Ok(result) => println!("Contract execution result: {}", result),
                    Err(err) => println!("Contract execution failed: {}", err),
                }
                blockchain.save_to_file("blockchain.json").unwrap();
            }
            Commands::CreatePrivacyTransaction { amount } => {
                let transaction = PrivacyTransaction::new(*amount);
                let mut blockchain = Blockchain::load_from_file("blockchain.json").unwrap();
                blockchain.add_privacy_transaction(transaction);
                println!("Privacy transaction created and added to blockchain");
                blockchain.save_to_file("blockchain.json").unwrap();
            }
            Commands::GetBalance { address } => {
                let blockchain = Blockchain::load_from_file("blockchain.json").unwrap();
                let balance = blockchain.get_balance(address);
                println!("Balance of address {}: {}", address, balance);
            }
            Commands::SetBalance { address, balance } => {
                let mut blockchain = Blockchain::load_from_file("blockchain.json").unwrap();
                blockchain.set_balance(address, *balance);
                println!("Balance of address {} set to {}", address, balance);
                blockchain.save_to_file("blockchain.json").unwrap();
            }
        }
    }
}
