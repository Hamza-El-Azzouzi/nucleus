use std::env::current_dir;

pub fn pwd() {
    match current_dir() {
        Ok(path) => {
            println!("{}", path.display());
        }
        Err(err) => println!("pwd: unexpected error: {}", err),
    }
}
