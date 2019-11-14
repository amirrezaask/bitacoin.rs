use super::block::Block;
use super::error::BitaCoinError;
use super::hash;
use super::storage::Storage;

#[derive(Debug)]
pub struct BlockChain<T: Storage> {
    blocks: Vec<Block>,
    dificulty: usize,
    storage: T
}

impl<T: Storage> BlockChain<T> {
    pub fn new(dificulty: usize, storage: T) -> Self {
        let genesis_block = Block::new("Genesis".to_string(), "".to_string(), dificulty);
        BlockChain{
            blocks: vec![genesis_block],
            dificulty: dificulty,
            storage: storage
        }
    }
    pub fn add(&mut self, data: String) {
        let this_block = Block::new(data, self.blocks[self.blocks.len()-1].hash.to_string(), self.dificulty);
        self.storage.append(this_block).unwrap(); //TODO: implement error handling 
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
            if !hash::good_enough(&block.hash, self.dificulty) {
                return Some(BitaCoinError::new(format!("hash {} is not good enough", block.hash)))
            }
        }
        return None
    }
}
