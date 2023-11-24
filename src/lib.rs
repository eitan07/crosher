pub mod hash_operations {
    use sha2::{Sha256, digest::Digest};

    pub fn hash_sha256(buffer: &[u8]) -> [u8; 32] {
        let mut hasher: Sha256 = Sha256::new();
        hasher.update(buffer);
        hasher.finalize().into()
    }
}