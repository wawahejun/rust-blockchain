mod block;
mod blockchain;
mod transaction;
mod node;
mod merkle_tree;
mod cli;
mod wallet;
mod smart_contract; 
mod privacy; 

use cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    cli.run();
}
