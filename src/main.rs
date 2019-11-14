mod block;
mod blockchain;
mod hash;
mod error;
mod storage;

use blockchain::BlockChain;
use storage::hash_map_store::HashMapStore;

fn main() {
    let mut bc = BlockChain::new(4, HashMapStore::new());
    bc.add("block 1".to_string());
    bc.add("block 2".to_string());
    println!("{:?}", bc);
    match bc.validate() {
        Some(err) => println!("ERR {}", err),
        None => println!("OK")
    }
}
