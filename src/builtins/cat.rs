use super::pwd;
use std::fs;
use std::path::{Path, PathBuf};
pub fn cat(files: Vec<String>) {
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
