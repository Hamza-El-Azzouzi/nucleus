use crate::prelude::*;

pub fn mv(args: Vec<String>) {
    let (sources, dest) = args.split_at(args.len() - 1);
    let dest_path = Path::new(&dest[0]);
    let source_path = Path::new(&sources[0]);

    if sources.len() > 1 {
        move_multiple_sources(sources, dest_path);
    } else {
        move_single_source(source_path, dest_path);
    }
}

fn move_single_source(source: &Path, dest: &Path) {
    if !source.exists() {
        println!(
            "mv: cannot stat '{}': No such file or directory",
            source.display()
        );
        return;
    }

    if dest.is_dir() {
        if source == dest {
            println!(
                "mv: cannot move '{}' to a subdirectory of itself, '{}'",
                source.display(),
                dest.display()
            );
        } else {
            match source.file_name() {
                Some(file_name) => {
                    let dest = dest.join(file_name);
                    if let Err(e) = fs::rename(source, dest) {
                        println!("{e}")
                    }
                }
                _ => {
                    if let Err(e) = fs::rename(source, dest) {
                        println!("{e}")
                    }
                }
            }
        }
    } else {
        if source == dest {
            println!(
                "mv: '{}' and '{}' are the same file",
                source.display(),
                dest.display()
            );
            return;
        }

        if let Err(e) = fs::rename(source, dest) {
            println!("{e}")
        }
    }
}

fn move_multiple_sources(sources: &[String], dest: &Path) {
    if !dest.is_dir() {
        println!("mv: target '{}' is not a directory", dest.display());
        return;
    }

    let mut errors = Vec::new();

    for source in sources {
        let source_path = Path::new(source);

        if !source_path.exists() {
            errors.push(format!(
                "mv: cannot stat '{}': No such file or directory",
                source_path.display()
            ));
        }

        match source_path.file_name() {
            Some(file_name) => {
                let dest_path = dest.join(file_name);
                if let Err(e) = fs::rename(source_path, dest_path) {
                    println!("{e}")
                }
            }
            _ => {
                if let Err(e) = fs::rename(source_path, dest) {
                    println!("{e}")
                }
            }
        }
    }

    if !errors.is_empty() {
        println!("{}", errors.join("\n"));
    }
}
