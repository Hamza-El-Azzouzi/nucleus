use super::pwd;
use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
pub fn cat(files: Vec<String>) {
    if files.is_empty() {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(content) => println!("{}", content),
                Err(err) => {
                    println!("cat: failed to read from stdin: {}", err);
                    break;
                }
            }
        }
        return;
    }
    for file in files {
        let is_dir = Path::new::<String>(&file).is_dir();
        if is_dir {
            println!("cat: {}: Is a directory", file);
            continue;
        }
        let file_path = if Path::new(&file).is_relative() {
            PathBuf::from(pwd::pwd()).join(&file)
        } else {
            PathBuf::from(&file)
        };

        let file_result = fs::read_to_string(file_path);
        match file_result {
            Ok(file_content) => println!("{file_content}"),
            Err(_) => println!("cat: {}: No such file or directory", file),
        }
    }
}
