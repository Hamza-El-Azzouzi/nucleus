use std::collections::HashMap;
use std::path::PathBuf;

use std::fs::read_dir;
use std::os::unix::fs::MetadataExt;

use crate::utils::{
    clean_string, get_file_owner_and_group, get_modified_at, get_permission_string,
};

#[derive(Clone)]
struct Directory {
    path: PathBuf,
    entries: Vec<Vec<String>>,
    total_blocks: u64,
}

pub fn ls(flags: Vec<char>, directories: Vec<PathBuf>, files: Vec<PathBuf>) {
    let mut file_result: Vec<Vec<String>> = Vec::new();
    let mut dir_results: Vec<Directory> = Vec::new();

    // extract flags
    let (a_flag, f_flag, l_flag) = (
        flags.contains(&'a'),
        flags.contains(&'F'),
        flags.contains(&'l'),
    );

    // Handle files
    for file in &files {
        if l_flag {
            match get_detailed_file_info(file, None) {
                Ok(info) => file_result.push(info),
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            }
        } else {
            match file.to_str() {
                Some(name) => file_result.push(vec![name.to_string()]),
                None => {
                    eprintln!("ls: Invalid UTF-8 path: {}", file.display());
                    continue;
                }
            }
        }
    }

    // Handle directories
    for dir in directories.iter() {
        let entries = match read_dir(&dir) {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("ls: cannot open directory '{}': {}", dir.display(), e);
                continue;
            }
        };
        let mut dir_entry_result: Vec<Vec<String>> = Vec::new();
        let mut total_blocks: u64 = 0;

        if a_flag {
            if let Err(e) =
                add_dot_entries(&mut dir_entry_result, &mut total_blocks, &f_flag, &l_flag)
            {
                eprintln!("ls: Failed to add dot entries: {}", e);
                continue;
            }
        }

        let mut paths: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                if !a_flag {
                    if let Some(name) = entry.file_name().to_str() {
                        return !name.starts_with('.');
                    }
                }
                true
            })
            .collect();

        paths.sort_by(|a, b| {
            let a_name = clean_string(a.file_name().to_string_lossy().to_uppercase());
            let b_name = clean_string(b.file_name().to_string_lossy().to_uppercase());
            a_name.cmp(&b_name)
        });

        for entry in paths {
            let path = entry.path();
            if l_flag {
                match get_detailed_file_info(&path, Some(&mut total_blocks)) {
                    Ok(info) => dir_entry_result.push(info),
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                }
            } else {
                let name_result = path.file_name().and_then(|s| s.to_str()).map(|s| {
                    let mut name = s.to_string();
                    if f_flag && path.is_dir() {
                        name.push('/');
                    }
                    name
                });

                match name_result {
                    Some(name) => dir_entry_result.push(vec![name]),
                    None => {
                        eprintln!("ls: Invalid UTF-8 file name: {}", path.display());
                        continue;
                    }
                }
            }
        }

        dir_results.push(Directory {
            path: dir.clone(),
            entries: dir_entry_result,
            total_blocks,
        });
    }

    // Print files
    if !file_result.is_empty() {
        print(&mut file_result, &l_flag);
        if !dir_results.is_empty() {
            println!();
        }
    }

    for (i, mut dir) in dir_results.into_iter().enumerate() {
        if directories.len() + files.len() > 1 {
            println!("{}:", dir.path.display());
        }

        if l_flag {
            println!("total {}:", dir.total_blocks);
        }

        print(&mut dir.entries, &l_flag);
        if i < directories.len() - 1 {
            println!();
        }
    }
}

fn get_detailed_file_info(
    path: &PathBuf,
    total_blocks: Option<&mut u64>,
) -> Result<Vec<String>, String> {
    let metadata = path
        .metadata()
        .map_err(|e| format!("cannot access '{}': {}", path.display(), e))?;

    let permission = get_permission_string(&metadata);

    let len = metadata.len().to_string();

    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .or_else(|| Some(path.to_string_lossy().to_string())) // fallback for non-Unicode
        .ok_or_else(|| format!("Unable to get file name for path: {:?}", path))?;

    let (owner_name, group_name) = get_file_owner_and_group(&metadata)
        .map_err(|e| format!("cannot access '{}': {}", path.display(), e))?;

    let n_link = metadata.nlink().to_string();

    let modified_at = get_modified_at(&metadata);

    if let Some(blocks) = total_blocks {
        *blocks += metadata.blocks() / 2;
    }

    Ok(vec![
        permission,
        n_link,
        owner_name,
        group_name,
        len,
        modified_at,
        file_name,
    ])
}

fn add_dot_entries(
    result: &mut Vec<Vec<String>>,
    total_blocks: &mut u64,
    f_flag: &bool,
    l_flag: &bool,
) -> Result<(), String> {
    let mut dot = ".".to_owned();
    let mut dotdot = "..".to_owned();

    if *f_flag {
        dot.push('/');
        dotdot.push('/');
    };

    if *l_flag {
        let dot_path = PathBuf::from(".");
        let dotdot_path = PathBuf::from("..");

        let mut dot_info = get_detailed_file_info(&dot_path, Some(total_blocks))?;
        let mut dotdot_info = get_detailed_file_info(&dotdot_path, Some(total_blocks))?;

        dot_info[6] = dot;
        dotdot_info[6] = dotdot;

        result.insert(0, dotdot_info);
        result.insert(0, dot_info);
    } else {
        result.insert(0, vec![dotdot]);
        result.insert(0, vec![dot]);
    }
    Ok(())
}

fn format_detailed_file_info(max_lens: &HashMap<usize, usize>, path: &Vec<String>) -> String {
    let mut result = String::new();

    for (i, info) in path.iter().enumerate() {
        let max_width = max_lens.get(&i).copied().unwrap_or(0);

        if i == path.len() - 1 {
            result.push_str(info);
        } else if i == 1 || i == 4 {
            result.push_str(&format!("{:>width$} ", info, width = max_width));
        } else {
            result.push_str(&format!("{:<width$} ", info, width = max_width));
        }
    }

    result
}

fn print(result: &mut Vec<Vec<String>>, is_long: &bool) {
    let mut max_lens: HashMap<usize, usize> = HashMap::new();

    if *is_long {
        for path in result.iter() {
            for (i, field) in path.iter().enumerate() {
                let len = field.len();
                let entry = max_lens.entry(i).or_insert(0);
                if len > *entry {
                    *entry = len;
                }
            }
        }
    }

    for (i, path) in result.iter().enumerate() {
        if *is_long {
            println!("{}", format_detailed_file_info(&max_lens, path));
        } else {
            print!("{}", path[0]);
            if i < result.len() - 1 {
                print!("  ");
            }
        }
    }

    if !*is_long {
        println!();
    }
}
