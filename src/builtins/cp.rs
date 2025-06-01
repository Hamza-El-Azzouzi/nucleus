use std::fs::copy;
use std::path::Path;

pub fn cp(args: Vec<String>) {
    let src = args[0..args.len() - 1].to_vec();
    let target = args.iter().last().expect("args should not be empty").clone();
    let is_target_dir = Path::new::<String>(&target).is_dir();
    let one_file = src.len() == 1 && Path::new::<String>(&src[0]).is_file();
    let is_source_exist = Path::new::<String>(&src[0]).exists();

    if !is_source_exist {
        println!("cp: cannot stat '{}': No such file", &src[0]);
        return;
    }

    if !is_target_dir && one_file {
        if let Err(e) = copy(&src[0], &target) {
            println!("cp: cannot copy '{}' to '{}': {}", &src[0], &target, e);
        }
        return;
    }

    if !is_target_dir && src.len() > 1 {
        println!("cp: target '{}' is not a directory", target);
        return;
    }

    if is_target_dir {
        for s in src {
            let path = Path::new::<String>(&s);
            if !path.exists() {
                println!("cp: cannot stat '{}': No such file or directory", s);
                continue;
            }
            
            if path.is_dir() {
                println!("cp: -r not specified; omitting directory '{}'", s);
                continue;
            }
            
            let mut dist = target.clone();
            dist.push('/');
        
            match path.file_name() {
                Some(filename) => {
                    dist.push_str(&filename.to_string_lossy());
                }
                None => {
                    println!("cp: cannot determine filename for '{}'", s);
                    continue;
                }
            }

            if let Err(e) = copy(path, &dist) {
                println!("cp: cannot copy '{}' to '{}': {}", s, dist, e);
            }
        }
    }
}