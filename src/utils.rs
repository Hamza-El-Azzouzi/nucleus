use crate::prelude::*;

pub fn clean_string(s: String) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_uppercase()
}

pub fn strip_ansi_codes(s: &str) -> String {
    let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(s, "").to_string()
}

pub fn print_cur_dir(path: PathBuf) {
    let current_path = path.to_string_lossy();
    let home_dir = env::var("HOME").unwrap_or_else(|_| "/".to_string());

    let display_path = if current_path.starts_with(&home_dir) {
        current_path.replacen(&home_dir, "~", 1)
    } else {
        current_path.to_string()
    };

    let prompt = format!("{display_path} $ ");

    print!("{prompt}");
}
