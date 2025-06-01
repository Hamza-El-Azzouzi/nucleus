use std::fs;

pub fn mkdir(args: Vec<String>) {
    let mut errors = Vec::new();

    for dir_name in args {
        if let Err(err) = fs::create_dir(&dir_name) {
            errors.push(format!("mkdir: {}: {}", dir_name, err));
        }
    }

    if !errors.is_empty() {
        println!("{}", errors.join("\n"));
    }
}
