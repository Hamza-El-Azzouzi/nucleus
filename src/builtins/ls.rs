use std::path::PathBuf;

pub fn ls(flags: Vec<char>, directories: Vec<PathBuf>, files: Vec<PathBuf>) {
    println!("flags: {:?}\ndirectories: {:?}\nfiles: {:?}", flags, directories,files);
}
