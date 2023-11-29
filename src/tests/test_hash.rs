#![cfg(test)]

use crosher::hash_operations::hash_sha256;
use std::env::consts;
use std::process::Command;
use std::{fs, io::Write};

#[test]
fn hash_file_and_test() {
    let dir = fs::read_dir(".").expect("Can't read directory \".\"");
    let mut output_file = fs::File::create("files.sha256").expect("Failed to create file.");

    for file in dir {
        let file = file.expect("Can't get directory entry.");
        let file_name = file
            .file_name()
            .into_string()
            .expect("Failed to convert to UTF-8 String");

        if file
            .file_type()
            .expect("Can't retrieve file type.")
            .is_file()
        {
            if !file_name.contains(".sha256") {
                output_file
                    .write(
                        format!(
                            "{}  {}\n",
                            hash_sha256(&fs::read(&file_name).expect("Can't read file."))
                                .iter()
                                .map(|val| format!("{:02x}", val))
                                .collect::<String>(),
                            file_name
                        )
                        .as_bytes(),
                    )
                    .expect("Can't write to file!");
            }
        }
    }

    for s in std::env::args().collect::<Vec<String>>() {
        println!("{}", s);
    }

    if consts::OS == "linux" {
        let status = Command::new("sha256sum")
            .arg("--status")
            .arg("-c")
            .arg("files.sha256")
            .status();

        match status {
            Ok(code) => assert!(
                code.success(),
                "SHA256SUM command {}",
                if code.success() { "PASSED!" } else { "FAILED!" }
            ),
            Err(error) => assert!(false, "{}", error),
        }
    } else {
        assert!(
            false,
            "Can't run this test in non-linux OS! try using WSL if you're using windows."
        )
    }
}
