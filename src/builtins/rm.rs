use std::{env, fs, path::PathBuf};

pub fn rm(args: Vec<String>, recursive: bool) {
    if args.is_empty() {
        println!("missing operand");
    }

    // get the current dir
    let cur_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("Error getting current directory: {}", err);
            return;
        }
    };

    for elem in args {
        // check if the elements of the args are valid and exist
        let path = cur_dir.join(&elem);
        if !path.exists() {
            eprintln!("rm: cannot remove '{}': No such file or directory", elem);
            continue;
        }

        if path.is_file() {
            if let Err(err) = fs::remove_file(&path) {
                println!("Failed to remove file '{}': {}", elem, err);
                return;
            }
        } else if path.is_dir() {
        }
    }
}
