use std::env::current_dir;

pub fn pwd() {
    println!("{:?}", current_dir().unwrap().display());
}
