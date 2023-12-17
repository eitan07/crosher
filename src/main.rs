use crosher::{hash_operations, CrosherError};
use std::{env, fs, path::Path, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err(Box::new(CrosherError::InvalidParameters))
    }

    match args.get(1).unwrap().as_str() {
        "hashfile" | "hash" if args.len() == 3 => {
            let file_path = args.get(2).unwrap();
            let buffer: Vec<u8> = fs::read(file_path).expect("Can't read file.");
            let hash: [u8; 32] = hash_operations::hash_sha256(&buffer);
            let mut hash_str = String::new();
            for b in hash {
                hash_str += format!("{:02x}", b).as_str();
            }
            println!(
                "{}  {}",
                hash_str,
                Path::new(file_path).file_name().unwrap().to_str().unwrap()
            );
        }

        "help" => display_help(),

        _ => return Err(Box::new(CrosherError::InvalidParameters)),
    };
    Ok(())
}

fn display_help() {
    let help_page = include_str!("../resources/help_page.txt");
    println!("{}", help_page)
}
