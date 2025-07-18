use crate::prelude::*;

pub fn cp(args: Vec<String>) {
    let src = args[0..args.len() - 1].to_vec();
    let target = args
        .iter()
        .last()
        .expect("args should not be empty")
        .clone();
    let is_target_dir = Path::new::<String>(&target).is_dir();
    let one_file = src.len() == 1 && Path::new::<String>(&src[0]).is_file();
    let is_source_exist = Path::new::<String>(&src[0]).exists();

    if !is_source_exist && !is_target_dir {
        println!("cp: cannot stat '{}': No such file", &src[0]);
        return;
    }

    if !is_target_dir && one_file {
        let src_path = if Path::new(&src[0]).is_relative() {
            PathBuf::from(pwd::pwd()).join(&src[0])
        } else {
            PathBuf::from(&src[0])
        };

        let target_path = if Path::new(&target).is_relative() {
            PathBuf::from(pwd::pwd()).join(&target)
        } else {
            PathBuf::from(&target)
        };

        if src_path == target_path {
            println!("cp: '{}' and '{}' are the same file", &src[0], &target);
            return;
        }
        if let Err(e) = copy(&src[0], &target) {
            println!("cp: cannot copy '{}' to '{}': {}", &src[0], &target, e);
        }
        return;
    }

    if !is_target_dir && src.len() > 1 {
        println!("cp: target '{target}' is not a directory");
        return;
    }

    if is_target_dir {
        for s in src {
            let path = Path::new(&s);
            if !path.exists() {
                println!("cp: cannot stat '{s}': No such file or directory");
                continue;
            }

            if path.is_dir() {
                println!("cp: -r not specified; omitting directory '{s}'");
                continue;
            }

            let mut dist = target.clone();
            dist.push('/');

            match path.file_name() {
                Some(filename) => {
                    dist.push_str(&filename.to_string_lossy());
                }
                None => {
                    println!("cp: cannot determine filename for '{s}'");
                    continue;
                }
            }

            if let Err(e) = copy(path, &dist) {
                println!("cp: cannot copy '{s}' to '{dist}': {e}");
            }
        }
    }
}
