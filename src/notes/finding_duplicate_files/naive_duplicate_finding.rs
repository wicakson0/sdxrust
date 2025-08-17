// the raw/naive way of matching files

use std::{fs, io};

fn open_binary(filepath: &str) -> std::io::Result<Vec<u8>> {
    return fs::read(filepath);
}

fn is_same_binary(left_filepath: &str, right_filepath: &str) -> Result<bool, io::Error> {
    let left_file = open_binary(left_filepath)?;
    let right_file = open_binary(right_filepath)?;

    return Ok(left_file == right_file);
}

fn find_duplicate(filepaths: Vec<&str>) -> Result<Vec<(&str, &str)>, io::Error> {
    let mut matches: Vec<(&str, &str)> = Vec::new();

    for (left_idx, left_file) in filepaths.iter().enumerate() {
       for right_idx in 0..left_idx {
           let right_file = filepaths[right_idx];
           if is_same_binary(left_file, right_file)? {
                matches.push((&left_file, &right_file));
           }
       }
    }

    return Ok(matches);
}

fn main() {
    let files: Vec<&str> = vec![
        "E:/Current_Workdir/sdxrust/Cargo.toml",
        "E:/Current_Workdir/sdxrust/Cargo.lock",
        "E:/Current_Workdir/sdxrust/.gitignore",
        "E:/Current_Workdir/sdxrust/src/main.rs",
        "E:/Current_Workdir/sdxrust/src/main.rs",
    ];

    let matches = find_duplicate(files).unwrap();
    println!("matches: {:?}", matches);
}
