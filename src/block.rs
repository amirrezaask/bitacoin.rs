use super::hash;
use super::error::BitaCoinError;

#[derive(Debug)]
pub struct Block {
    data: String,
    dificulty: usize,
    pub previous_hash: String,
    created_at: std::time::SystemTime,
    pub hash: String
}

impl Block {
    pub fn new(data: String, prev_hash: String, dificulty: usize) -> Self {
        let mut b = Block{
            data: data,
            dificulty: dificulty,
            previous_hash: prev_hash,
            created_at: std::time::SystemTime::now(),
            hash: "".to_string()
        };
        b.hash = b.generate_hash();
        b
    }
    pub fn generate_hash(&self) -> String {
        let data = format!("{}{}{:?}", self.data, self.previous_hash, self.created_at);
        hash::difficult_hash(data, self.dificulty)
    }
    pub fn validate(&self) -> Option<BitaCoinError> {
        let current_hash = self.generate_hash();
        if current_hash != self.hash {
            return Some(BitaCoinError::new(format!("hash should be {} but is {}", self.hash, current_hash)))
        }
        return None
    }
}
