// the raw/naive way of matching files

use std::fs;
use std::io::Error;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

fn open_binary(filepath: &str) -> std::io::Result<Vec<u8>> {
    return fs::read(filepath);
}

fn generate_sha256_hash(binary_file: Vec<u8>) -> Result<String, Error> {
    let mut hasher = Sha256::new();
    hasher.update(binary_file);
    let hash = hasher.finalize();
    return Ok(format!("{:x}", hash));
}

fn generate_hash_dictionary(filepaths: Vec<String>) -> Result<HashMap<String, Vec<String>>, Error> {
    let mut hash_map = HashMap::<String, Vec<String>>::new();

    for file in filepaths {
        let binary_file = open_binary(&file)?;
        let binary_hash = generate_sha256_hash(binary_file)?.to_string();

        // Insert file into the vector for this hash
        hash_map
            .entry(binary_hash)
            .or_insert_with(Vec::new)
            .push(file);
    }

    return Ok(hash_map)
}

fn find_duplicate_hash(file_hash: HashMap<String, Vec<String>>) -> Result<Vec<Vec<String>>, Error> {
    let mut duplicates: Vec<Vec<String>> = Vec::new();

    for(_hash, files) in file_hash {
        if files.len() > 1 {
            duplicates.push(files);
        }
    }

    return Ok(duplicates);
}

fn main() {
    let files: Vec<String> = vec![
        "E:/Current_Workdir/sdxrust/Cargo.toml".to_string(),
        "E:/Current_Workdir/sdxrust/Cargo.lock".to_string(),
        "E:/Current_Workdir/sdxrust/.gitignore".to_string(),
        "E:/Current_Workdir/sdxrust/src/main.rs".to_string(),
        "E:/Current_Workdir/sdxrust/src/main.rs".to_string(),
    ];

    let hash_dict = generate_hash_dictionary(files);

    let duplicates = find_duplicate_hash(hash_dict.unwrap());
    println!("matches: {:?}", duplicates);
}
