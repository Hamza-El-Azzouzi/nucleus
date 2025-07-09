use std::env::current_dir;

pub fn pwd() -> String {
    match current_dir() {
        Ok(path) => path.to_str().expect("msg").to_string(),
        Err(err) => format!("pwd: unexpected error: {err}"),
    }
}
