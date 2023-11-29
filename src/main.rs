use crosher::hash_operations;
use std::{env, ffi::OsStr, fs, path::Path, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        display_help(Some(1));
    }

    match args
        .get(1)
        .unwrap_or_else(|| display_help(Some(1)))
        .as_str()
    {
        "hashfile" | "hash" if args.len() == 3 => {
            let file_path = args.get(2).unwrap_or_else(|| display_help(Some(1)));
            let buffer: Vec<u8> = fs::read(file_path).expect("Can't read file.");
            let hash: [u8; 32] = hash_operations::hash_sha256(&buffer);
            let mut hash_str = String::new();
            for b in hash {
                hash_str += format!("{:02x}", b).as_str();
            }
            println!(
                "{}  {}",
                hash_str,
                Path::new(file_path)
                    .file_name()
                    .unwrap_or(&OsStr::new(file_path))
                    .to_str()
                    .unwrap_or(file_path)
            );
        }

        "help" => display_help(Some(0)),

        _ => display_help(Some(1)),
    }
}

fn display_help(reason: Option<u8>) -> ! {
    let help_page = include_str!("../resources/help_page.txt");

    let exit_code = match reason {
        Some(1) => {
            println!("Invalid subcommand or file!\n");
            127
        }
        _ => 0,
    };

    println!("{}", help_page);
    process::exit(exit_code)
}
