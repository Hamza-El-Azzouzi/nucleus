use super::pwd;
use std::fs;
use std::io::{ self, BufRead };
use std::path::{ Path, PathBuf };
pub fn cat(files: Vec<String>) {
    if files.is_empty() {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(content) => println!("{content}"),
                Err(err) => {
                    println!("cat: failed to read from stdin: {err}");
                    break;
                }
            }
        }
        return;
    }

    'files_loop:for file in files {
        if file == "-" {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                match line {
                    Ok(content) => println!("{content}"),
                    Err(err) => {
                        println!("cat: failed to read from stdin: {err}");
                        
                    }
                }
            }
            continue 'files_loop;
        }
        let is_dir = Path::new::<String>(&file).is_dir();
        if is_dir {
            println!("cat: {file}: Is a directory");
            continue;
        }
        let file_path = if Path::new(&file).is_relative() {
            PathBuf::from(pwd::pwd()).join(&file)
        } else {
            PathBuf::from(&file)
        };

        let file_result = fs::read_to_string(file_path);
        match file_result {
            Ok(file_content) => print!("{file_content}"),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => println!("cat: {file}: No such file or directory"),
                _ => println!("cat: {file}: {}",e.kind()),
            },
        }
    }
}
