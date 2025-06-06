use std::{env, path::Path};

pub fn cd(args: Vec<String>) {
    match args.len() {
        0 => change_to_home(),
        _ => match args[0].as_str() {
            "-" => change_to_previous(),
            "~" => change_to_home(),
            path if path.starts_with("~/") => match env::var("HOME") {
                Ok(home_dir) => {
                    let expanded_path = path.replace("~", &home_dir);
                    change_dir(&expanded_path);
                }
                Err(_) => println!("cd: HOME environment variable not set"),
            },
            path => change_dir(path),
        },
    }
}

fn change_to_home() {
    match env::var("HOME") {
        Ok(home_dir) => change_dir(&home_dir),
        Err(_) => println!("cd: HOME environment variable not set"),
    }
}

fn change_to_previous() {
    match env::var("OLDPWD") {
        Ok(old_dir) => {
            let current_dir = match env::current_dir() {
                Ok(dir) => dir,
                Err(e) => {
                    println!("cd: cannot get current directory: {}", e);
                    return;
                }
            };

            let target_path = Path::new(&old_dir);
            if !target_path.exists() {
                println!("cd: {}: No such file or directory", old_dir);
                return;
            }
            if !target_path.is_dir() {
                println!("cd: {}: Not a directory", old_dir);
                return;
            }
            if let Err(err) = env::set_current_dir(target_path) {
                println!("cd: {}", err);
                return;
            }

            unsafe { env::set_var("OLDPWD", current_dir) };

            println!("{}", old_dir);
        }
        Err(_) => println!("cd: OLDPWD not set"),
    }
}

fn change_dir(path: &str) {
    if let Ok(current_dir) = env::current_dir() {
        unsafe { env::set_var("OLDPWD", current_dir) };
    }

    let target_path = Path::new(path);

    if !target_path.exists() {
        println!("cd: {}: No such file or directory", path);
        return;
    }

    if !target_path.is_dir() {
        println!("cd: {}: Not a directory", path);
        return;
    }

    if let Err(err) = env::set_current_dir(target_path) {
        println!("cd: {}", err);
    }
}
