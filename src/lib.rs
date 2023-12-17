use std::{error::Error, fmt::Display};

pub mod hash_operations {
    use sha2::{digest::Digest, Sha256};
    
    pub fn hash_sha256(buffer: &[u8]) -> [u8; 32] {
        let mut hasher: Sha256 = Sha256::new();
        hasher.update(buffer);
        hasher.finalize().into()
    }
}



#[derive(Debug)]
pub enum CrosherError {
    InvalidParameters,
    InvalidFileName,
    InvalidDigest,
}

impl Display for CrosherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error occurred: {}", self)
    }
}

impl Error for CrosherError {
    fn description(&self) -> &str {
        match self {
            CrosherError::InvalidFileName => "Invalid file name",
            CrosherError::InvalidDigest => "Invalid file digest",
            CrosherError::InvalidParameters => "Invalid parameters"
        }
    }
}