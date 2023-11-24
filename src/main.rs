use std::{ env, fs };
use winhash::hash_operations;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        display_help();
    }

    match args.get(1).unwrap().as_str() {
        "hashfile" | "hash" if args.len() == 3 => {
            let file_path = args.get(2).unwrap();
            let buffer: Vec<u8> = fs::read(file_path).unwrap();
            let hash: [u8; 32] = hash_operations::hash_sha256(&buffer);
            let mut hash_str = String::new();
            for b in hash {
                hash_str += format!("{:02x}", b).as_str();
            }
            println!("{}  {}", hash_str, file_path);
        }

        _ => display_help(),
    }
}

fn display_help() {}
