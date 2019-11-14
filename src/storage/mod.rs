pub mod hash_map_store;

use super::block::Block;
use super::error::BitaCoinError;

pub trait Storage {
    fn load(&self, hash: &String) -> Result<&Block, BitaCoinError>;
    fn append(&mut self, b: Block) -> Result<(), BitaCoinError>;
    fn last_hash(&self) -> String;
}