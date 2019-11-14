use core::fmt;
use std::error;

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