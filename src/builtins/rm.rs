use crate::prelude::*;

pub fn rm(args: Vec<String>, recursive: bool) {
    let cur_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            eprintln!("Error getting current directory: {err}");
            return;
        }
    };

    for elem in args {
        if elem.chars().all(|c| c == '.' || c == '/') {
            eprintln!("rm: refusing to remove '.' or '..' directory: skipping '{elem}'",);
            continue;
        }

        let path = cur_dir.join(&elem);
        if !path.exists() {
            eprintln!("rm: cannot remove '{elem}': No such file or directory");
            continue;
        }

        if path.is_file() {
            if let Err(err) = fs::remove_file(&path) {
                eprintln!("rm: Failed to remove file '{elem}': {err}");
                continue;
            }
        } else if path.is_dir() {
            if recursive {
                if let Err(err) = fs::remove_dir_all(&path) {
                    eprintln!("rm: Failed to remove file '{elem}': {err}");
                    continue;
                }
            } else {
                eprintln!(
                    "rm: cannot remove '{elem}': Is a directory. Use -r to remove recursively."
                );
                continue;
            }
        } else {
            eprintln!("rm: cannot remove '{elem}'");
        }
    }
}
