use std::path::PathBuf;

pub use crate::builtins::ls::parser::Flag;
use crate::builtins::ls::{output::LsOutput, processor::LsProcessor};

mod file_info;
mod file_permissions;
mod formatter;
mod output;
mod parser;
mod processor;

#[derive(Clone, Debug)]
pub struct Directory {
    pub path: PathBuf,
    pub entries: Vec<Vec<String>>,
    pub max_len: usize,
    pub total_blocks: u64,
}

pub fn ls(flags: Vec<char>, directories: Vec<PathBuf>, files: Vec<PathBuf>) {
    let mut directories: Vec<PathBuf> = directories.clone();
    let files: Vec<PathBuf> = files.clone();
    let mut file_result: Vec<Vec<String>> = Vec::new();
    let mut dir_results: Vec<Directory> = Vec::new();

    // let flags = Flag::parse(&args, &mut directories, &mut files)?;

    let flags = Flag {
        l: flags.contains(&'l'),
        a: flags.contains(&'a'),
        f: flags.contains(&'F'),
    };

    if directories.is_empty() && files.is_empty() {
        directories.push(PathBuf::from("."));
    }
    let mut max_files_len = 0;
    match LsProcessor::process_files(&files, &flags, &mut max_files_len, &mut file_result) {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
    };
    match LsProcessor::process_directories(&directories, &flags, &mut dir_results) {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
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
