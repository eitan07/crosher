mod filesystem_operations {
    use std::fs::File;
    use std::io::{ BufReader, Read };

    pub fn read_file(file_name: &String) -> Vec<u8> {
        let file = File::open(file_name).unwrap();
        let mut buffer_reader = BufReader::new(file);
        let mut buffer: Vec<u8> = Vec::new(); 
        buffer_reader.read_to_end(&mut buffer).unwrap();
        buffer
    }
}

pub mod hash_operations {
    use sha2::{ Sha256, Digest };
    use super::filesystem_operations::*;

    pub fn get_file_hash_sha256(file_name: &String) -> [u8; 32] {
        let buffer = read_file(file_name);
        let mut hasher: Sha256 = Sha256::new();
        hasher.update(&buffer);
        hasher.finalize().into()
    }
}