use crate::prelude::*;

pub mod file_info;
pub mod file_permissions;
pub mod formatter;
pub mod output;
pub mod parser;
pub mod processor;

#[derive(Clone, Debug)]
pub struct Directory {
    pub path: PathBuf,
    pub entries: Vec<Vec<String>>,
    pub max_len: usize,
    pub total_blocks: u64,
}

pub fn ls(args: Vec<String>) {
    let mut directories: Vec<PathBuf> = Vec::new();
    let mut files: Vec<PathBuf> = Vec::new();
    let mut file_result: Vec<Vec<String>> = Vec::new();
    let mut dir_results: Vec<Directory> = Vec::new();

    let flags = match Flag::parse(&args, &mut directories, &mut files) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("ls: {e}");
            return;
        }
    };

    if directories.is_empty() && files.is_empty() {
        directories.push(PathBuf::from("."));
    }
    let mut max_files_len = 0;
    match LsProcessor::process_files(&files, &flags, &mut max_files_len, &mut file_result) {
        Ok(_) => (),
        Err(e) => eprintln!("ls: {e}"),
    };
    match LsProcessor::process_directories(&directories, &flags, &mut dir_results) {
        Ok(_) => (),
        Err(e) => eprintln!("ls: {e}"),
    };

    LsOutput::print_results(
        &file_result,
        &dir_results,
        &directories.len(),
        &files.len(),
        &max_files_len,
        &flags,
    );
}
