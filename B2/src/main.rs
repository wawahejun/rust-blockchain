mod block;
mod blockchain;
mod transaction;
mod node;
mod merkle_tree;

use std::net::{SocketAddr, IpAddr};
use std::sync::{Arc, Mutex};
use std::thread;
use chrono::Utc;
use node::Node;
use transaction::{Transaction, generate_key_pair};
use blockchain::Blockchain;
use ring::signature::KeyPair;

fn main() {
    // 生成密钥对（用于交易签名）
    let alice_key_pair = generate_key_pair();
    let bob_key_pair = generate_key_pair();

    // 创建两个节点
    let node1 = Arc::new(Mutex::new(Node::new(
        SocketAddr::new(IpAddr::from([127, 0, 0, 1]), 8080),
        4, // 难度
    )));
    let node2 = Arc::new(Mutex::new(Node::new(
        SocketAddr::new(IpAddr::from([127, 0, 0, 1]), 8081),
        4, // 难度
    )));

    // 添加对等节点
    node1.lock().unwrap().add_peer(node2.lock().unwrap().address);
    node2.lock().unwrap().add_peer(node1.lock().unwrap().address);

    // 启动节点
    let node1_clone = Arc::clone(&node1);
    let node2_clone = Arc::clone(&node2);

    let handle1 = thread::spawn(move || {
        let mut node = node1_clone.lock().unwrap();

        // 添加一笔交易
        let transaction = Transaction::new(
            hex::encode(alice_key_pair.public_key().as_ref()), // Alice 的公钥
            hex::encode(bob_key_pair.public_key().as_ref()),   // Bob 的公钥
            100, // 交易金额
            &alice_key_pair, // Alice 的密钥对（用于签名）
        );

        // 将交易添加到新区块
        node.blockchain.lock().unwrap().add_block(vec![transaction]);

        // 同步区块链
        node.sync_blockchain();

        // 打印节点1的区块链
        println!("Blockchain on Node 1:");
        for block in &node.blockchain.lock().unwrap().chain {
            println!("{:#?}", block);
        }
    });

    let handle2 = thread::spawn(move || {
        let mut node = node2_clone.lock().unwrap();

        // 同步区块链
        node.sync_blockchain();

        // 打印节点2的区块链
        println!("Blockchain on Node 2:");
        for block in &node.blockchain.lock().unwrap().chain {
            println!("{:#?}", block);
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
