use crate::prelude::*;

pub fn add_dot_entries(
    dir: PathBuf,
    result: &mut Vec<Vec<String>>,
    total_blocks: &mut u64,
    max_len: &mut usize,
    flags: &Flag,
) -> Result<(), String> {
    let mut dot: String = ".".to_owned();
    let mut dotdot = "..".to_owned();

    if flags.l {
        let dot_path = dir.join(PathBuf::from("."));
        let dotdot_path = dir.join(PathBuf::from(".."));

        let mut dot_info =
            get_detailed_file_info(dot_path, &mut dot, Some(total_blocks), max_len, flags)?;
        let mut dotdot_info =
            get_detailed_file_info(dotdot_path, &mut dotdot, Some(total_blocks), max_len, flags)?;

        dot_info[6] = dot;
        dotdot_info[6] = dotdot;

        result.insert(0, dotdot_info);
        result.insert(0, dot_info); // tartib
    } else {
        result.insert(0, vec![dotdot]);
        result.insert(0, vec![dot]);
    }
    Ok(())
}

pub fn format_detailed_file_info(
    max_lens: &HashMap<usize, usize>,
    path: &[String],
    max_size_len: &usize,
    quote_exist: &bool,
) -> String {
    let mut result = String::new();

    for (i, info) in path.iter().enumerate() {
        let max_width = max_lens.get(&i).copied().unwrap_or(0);

        if i == path.len() - 1 {
            let name = strip_ansi_codes(info);
            if *quote_exist && !name.starts_with("'") && !name.starts_with("\"") {
                result.push(' ');
            }

            result.push_str(info);
        } else if i == 1 {
            result.push_str(&format!("{info:>max_width$} "));
        } else if i == 4 {
            if info.contains(",") {
                let parts: Vec<&str> = info.split(',').collect();
                let spaces_to_add = max_size_len - info.len();
                let spaces: String = " ".repeat(spaces_to_add);
                let formatted = format!("{}, {}{}", parts[0].trim(), spaces, parts[1].trim());
                result.push_str(&format!("{formatted:>max_width$} "));
            } else {
                result.push_str(&format!("{info:>max_width$} "));
            }
        } else {
            result.push_str(&format!("{info:<max_width$} "));
        }
    }

    result
}

pub fn format_path(path: PathBuf, file_name: &mut String, flags: &Flag) -> Result<(), String> {
    let metadata = path
        .symlink_metadata()
        .map_err(|e| format!("cannot access '{}': {}", path.display(), e))?;
    let mode = metadata.permissions().mode();

    let file_type = metadata.file_type();
    if file_type.is_fifo() {
        colorize_pipe(file_name, flags);
    } else if file_type.is_socket() {
        colorize_socket(file_name, flags);
    } else if path.is_symlink() {
        return format_symlink(&path, file_name, flags);
    } else if path.is_dir() {
        colorize_dir(file_name, flags);
        return Ok(());
    } else if metadata.file_type().is_block_device() || metadata.file_type().is_char_device() {
        colorize_device(file_name, flags);
    }

    if is_executable(&mode) {
        colorize_executable(file_name, flags);
    }

    Ok(())
}

fn format_symlink(path: &Path, file_name: &mut String, flags: &Flag) -> Result<(), String> {
    let is_broken = fs::metadata(path).is_err();
    colorize_symlink(file_name, is_broken, flags);

    if flags.l {
        if let Ok(target) = fs::read_link(path) {
            let full_target_path = if target.is_absolute() {
                target.clone()
            } else {
                path.parent().unwrap_or_else(|| Path::new("")).join(&target)
            };

            let mut target_str = target.to_string_lossy().to_string();

            if fs::metadata(&full_target_path).is_err() {
                colorize_symlink(&mut target_str, true, flags);
            } else if target.is_symlink() {
                let metadata = target
                    .metadata()
                    .map_err(|e| format!("cannot access '{}': {}", path.display(), e))?;
                let mode = metadata.permissions().mode();
                if metadata.is_dir() {
                    colorize_dir(file_name, flags);
                } else if metadata.file_type().is_char_device()
                    || metadata.file_type().is_block_device()
                {
                    colorize_device(&mut target_str, flags);
                } else if is_executable(&mode) {
                    colorize_executable(&mut target_str, flags);
                }
            } else {
                let _ = format_path(full_target_path, &mut target_str, flags);
            }

            file_name.push_str(" -> ");
            file_name.push_str(&target_str);
        } else {
            file_name.push_str(" -> ");
            file_name.push_str(&colorize("invalid symlink", Color::Red, true));
        }
    }

    Ok(())
}

pub fn quote_if_needed(name: &mut String) {
    let unsafe_chars = [
        ' ', '\t', '\n', '\'', '"', '$', '`', '\\', '!', '*', '?', '&', ';', '|', '<', '>', '(',
        ')', '[', ']', '{', '}', '~', '#',
    ];

    if name.chars().any(|c| unsafe_chars.contains(&c)) {
        let quote = if name.contains("'") { '"' } else { '\'' };
        name.push(quote);
        let mut temp_str: String = String::from(quote);
        temp_str.push_str(name.as_str());
        *name = temp_str;
    }
}
