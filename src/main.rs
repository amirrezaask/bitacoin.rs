mod block;
mod blockchain;
mod hash;
mod error;

use blockchain::BlockChain;

fn main() {
    let mut bc = BlockChain::new(4);
    bc.add("block 1".to_string());
    bc.add("block 2".to_string());
    println!("{:?}", bc);
    match bc.validate() {
        Some(err) => println!("ERR {}", err),
        None => println!("OK")
    }
}
