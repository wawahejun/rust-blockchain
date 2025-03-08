use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;

#[derive(Debug, Clone)]
pub struct Node {
    pub address: SocketAddr,
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub peers: Vec<SocketAddr>,
}

impl Node {
    // 创建一个新节点
    pub fn new(address: SocketAddr, difficulty: usize) -> Self {
        Node {
            address,
            blockchain: Arc::new(Mutex::new(Blockchain::new(difficulty))),
            peers: Vec::new(),
        }
    }

    // 添加一个对等节点
    pub fn add_peer(&mut self, peer: SocketAddr) {
        self.peers.push(peer);
    }

    // 同步区块链
    pub fn sync_blockchain(&self) {
        for peer in &self.peers {
            println!("Syncing blockchain with peer: {}", peer);
        }
    }
}
