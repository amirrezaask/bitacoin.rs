extern crate sha2;

use sha2::Digest;
use std::ops::Add;
use core::fmt;
use std::error;

/*
    Block:
        Data string/[]byte
        previous_hash string/[]byte
        created_at time
        hash string/[]byte

    BlockChain:
        [Block]
*/

#[derive(Debug, Clone)]
pub struct BitaCoinError(String);


impl BitaCoinError {
    pub fn new(msg: String) -> Self {
        BitaCoinError(msg)
    }
}

impl fmt::Display for BitaCoinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl error::Error for BitaCoinError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}




#[derive(Debug)]
pub struct Block {
    data: String,
    previous_hash: String,
    created_at: std::time::SystemTime,
    hash: String
}

impl Block {
    pub fn new(data: String, prev_hash: String) -> Self {
        let mut b = Block{
            data: data,
            previous_hash: prev_hash,
            created_at: std::time::SystemTime::now(),
            hash: "".to_string()
        };
        b.hash = b.generate_hash();
        b
    }
    pub fn generate_hash(&self) -> String {
        let data = format!("{}{}{:?}", self.data, self.previous_hash, self.created_at);
        difficult_hash(data)
    }
    pub fn validate(&self) -> Option<BitaCoinError> {
        let current_hash = self.generate_hash();
        if current_hash != self.hash {
            return Some(BitaCoinError::new(format!("hash should be {} but is {}", self.hash, current_hash)))
        }
        return None
    }
}

pub struct BlockChain {
    blocks: Vec<Block>
}

impl BlockChain {
    pub fn new() -> Self {
        let genesis_block = Block::new("Genesis".to_string(), "".to_string());
        BlockChain{
            blocks: vec![genesis_block]
        }
    }
    pub fn add(&mut self, data: String) {
        let this_block = Block::new(data, self.blocks[self.blocks.len()-1].hash.to_string());
        self.blocks.push(this_block);
    }
    pub fn validate(&self) -> Option<BitaCoinError> {
        for (idx, block) in self.blocks.iter().enumerate() {
            if block.validate().is_some() {
                return Some(BitaCoinError::new(format!("block chain validation failed due to {}", block.validate().unwrap())))
            }
            if idx == 0{
                continue
            }
            if block.previous_hash != self.blocks[idx-1].hash {
                return Some(BitaCoinError::new(format!("prev hash should be {} but is {}", self.blocks[idx-1].hash, block.previous_hash)))
            }
            if !good_enough(&block.hash) {
                return Some(BitaCoinError::new(format!("hash {} is not good enough", block.hash)))
            }
        }
        return None
    }
}


pub fn simple_hash(data: String) -> String {
    format!("{:x}", sha2::Sha256::digest(data.as_bytes()))
}

pub fn good_enough(hash: &String) -> bool {
    if hash.chars().nth(0).unwrap() == '0' {
        return true
    }
    return false
}


pub fn difficult_hash(data: String) -> String {
    let mut i = 0;
    loop {
        let data = format!("{}{}", data, i);
        let h = simple_hash(data);
        if good_enough(&h) {
            return h
        }
        i = i + 1;
    }
}

fn main() {
    let mut bc = BlockChain::new();
    bc.add("block 1".to_string());
    bc.add("block 2".to_string());
    for b in &bc.blocks {
        println!("{:?}", b)
    }
    match bc.validate() {
        Some(err) => println!("ERR {}", err),
        None => println!("OK")
    }
}
