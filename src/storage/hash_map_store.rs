use std::collections::hash_map::HashMap;
use super::super::error::BitaCoinError;
use super::super::block::Block;
use super::Storage;

#[derive(Debug)]
pub struct HashMapStore {
    blocks: HashMap<String, Block>,
    last: String
}
impl HashMapStore {
    pub fn new() -> Self {
        HashMapStore{
            blocks: HashMap::new(),
            last: String::new(),
        }
    }
}

impl Storage for HashMapStore {
    fn load(&self, hash: &String) -> Result<&Block, BitaCoinError> {
        match self.blocks.get(hash) {
            Some(block) => Ok(block),
            None => Err(BitaCoinError(String::from("block does not exists")))
        }
    }
    fn append(&mut self, b: Block) -> Result<(), BitaCoinError> {
        let hash = b.hash.to_string();
        match self.blocks.insert(hash.to_string(), b) {
            None => {
                self.last = hash;
                Ok(())
            },
            Some(_) => Err(BitaCoinError(String::from("block already exists")))
        }
    }
    fn last_hash(&self) -> String {
        self.last.to_string()
    }
}