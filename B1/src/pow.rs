use crate::block::Block;

pub fn proof_of_work(block: &mut Block, difficulty: usize) {
    let target = "0".repeat(difficulty);
    while &block.hash[..difficulty] != target {
        block.nonce += 1;
        block.hash = block.calculate_hash();
    }
    println!("Block mined: {}", block.hash);
}
