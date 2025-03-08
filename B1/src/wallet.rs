use ring::signature::{Ed25519KeyPair, KeyPair};
use ring::rand::SystemRandom;
use hex;

#[derive(Debug)]
pub struct Wallet {
    pub key_pair: Ed25519KeyPair,
}

impl Wallet {
    // 创建一个新钱包
    pub fn new() -> Self {
        let rng = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();
        Wallet { key_pair }
    }

    // 获取钱包地址（公钥的十六进制表示）
    pub fn address(&self) -> String {
        hex::encode(self.key_pair.public_key().as_ref())
    }

    // 签名数据
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        self.key_pair.sign(data).as_ref().to_vec()
    }

    // 验证签名
    pub fn verify(&self, data: &[u8], signature: &[u8]) -> bool {
        let public_key = self.key_pair.public_key();
        let public_key_bytes = public_key.as_ref();
        let public_key = ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, public_key_bytes);
        public_key.verify(data, signature).is_ok()
    }
}
