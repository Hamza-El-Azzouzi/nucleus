use crate::prelude::*;

pub struct LsProcessor;

impl LsProcessor {
    pub fn process_files(
        files: &[PathBuf],
        flags: &Flag,
        max_len: &mut usize,
        file_result: &mut Vec<Vec<String>>,
    ) -> Result<(), String> {
        let mut files = files.to_vec();
        files.sort_by(|a, b| {
            let a_name = clean_string(a.to_string_lossy().to_uppercase());
            let b_name = clean_string(b.to_string_lossy().to_uppercase());
            a_name.cmp(&b_name)
        });

        for file in files {
            if flags.l {
                let mut file_name = file.to_string_lossy().to_string();
                quote_if_needed(&mut file_name);
                let info = get_detailed_file_info(file, &mut file_name, None, max_len, flags)?;
                file_result.push(info);
            } else {
                let mut name = file
                    .to_str()
                    .ok_or_else(|| format!("ls: Invalid UTF-8 path: {}", file.display()))?
                    .to_string();
                quote_if_needed(&mut name);
                format_path(file, &mut name, flags)?;
                file_result.push(vec![name]);
            }
        }
        Ok(())
    }

    pub fn process_directories(
        directories: &[PathBuf],
        flags: &Flag,
        dir_results: &mut Vec<Directory>,
    ) -> Result<(), String> {
        for dir in directories {
            let entries = read_dir(dir)
                .map_err(|e| format!("ls: cannot open directory '{}': {}", dir.display(), e))?;

            let mut dir_entry_result: Vec<Vec<String>> = Vec::new();
            let mut total_blocks: u64 = 0;
            let mut max_len = 0;

            if flags.a {
                add_dot_entries(
                    dir.to_path_buf(),
                    &mut dir_entry_result,
                    &mut total_blocks,
                    &mut max_len,
                    flags,
                )
                .map_err(|e| format!("ls: Failed to add dot entries: {e}"))?;
            }

            Self::process_directory_entries(
                entries,
                flags,
                &mut dir_entry_result,
                &mut total_blocks,
                &mut max_len,
            )?;

            dir_results.push(Directory {
                path: dir.clone(),
                entries: dir_entry_result,
                total_blocks,
                max_len,
            });
        }

        // sort directories
        dir_results.sort_by(|a, b| {
            let a_name = clean_string(a.path.to_string_lossy().to_uppercase());
            let b_name = clean_string(b.path.to_string_lossy().to_uppercase());
            a_name.cmp(&b_name)
        });

        Ok(())
    }

    fn process_directory_entries(
        entries: std::fs::ReadDir,
        flags: &Flag,
        dir_entry_result: &mut Vec<Vec<String>>,
        total_blocks: &mut u64,
        max_len: &mut usize,
    ) -> Result<(), String> {
        let mut paths: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                if !flags.a {
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
            if flags.l {
                let mut file_name = path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
                    .or_else(|| Some(path.to_string_lossy().to_string()))
                    .ok_or_else(|| format!("Unable to get file name for path: {path:?}"))?;
                quote_if_needed(&mut file_name);

                match get_detailed_file_info(
                    path,
                    &mut file_name,
                    Some(total_blocks),
                    max_len,
                    flags,
                ) {
                    Ok(info) => dir_entry_result.push(info),
                    Err(e) => {
                        eprintln!("{e}");
                        continue;
                    }
                }
            } else {
                let mut name = path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .ok_or_else(|| format!("ls: Invalid UTF-8 file name: {}", path.display()))?
                    .to_string();

                quote_if_needed(&mut name);
                format_path(path, &mut name, flags)?;

                dir_entry_result.push(vec![name]);
            }
        }
        Ok(())
    }
}
