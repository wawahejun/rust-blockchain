use serde::{Serialize, Deserialize};
use ring::signature::{self, Ed25519KeyPair};
use ring::rand::SystemRandom;
use hex;
use crate::wallet::Wallet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signature: String,
}

impl Transaction {
    // 创建一笔新交易
    pub fn new(sender: String, receiver: String, amount: u64, key_pair: &Ed25519KeyPair) -> Self {
        let mut transaction = Transaction {
            sender,
            receiver,
            amount,
            signature: String::new(),
        };
        transaction.sign(key_pair);
        transaction
    }

    // 对交易进行签名
    pub fn sign(&mut self, key_pair: &Ed25519KeyPair) {
        let message = self.to_message();
        let signature_bytes = key_pair.sign(&message).as_ref().to_vec();
        self.signature = hex::encode(signature_bytes);
    }

    // 验证交易的签名
    pub fn verify(&self) -> bool {
        let message = self.to_message();
        let public_key_bytes = hex::decode(&self.sender).unwrap();
        let signature_bytes = hex::decode(&self.signature).unwrap();
        let public_key = ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, &public_key_bytes);
        public_key.verify(&message, &signature_bytes).is_ok()
    }

    // 将交易数据转换为消息（用于签名和验证）
    fn to_message(&self) -> Vec<u8> {
        let data = format!("{}{}{}", self.sender, self.receiver, self.amount);
        data.into_bytes()
    }
}

// 实现交易的字符串表示
impl ToString for Transaction {
    fn to_string(&self) -> String {
        format!(
            "sender: {}, receiver: {}, amount: {}, signature: {}",
            self.sender, self.receiver, self.amount, self.signature
        )
    }
}

// 生成一个新的密钥对
pub fn generate_key_pair() -> Ed25519KeyPair {
    let rng = SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap()
}
