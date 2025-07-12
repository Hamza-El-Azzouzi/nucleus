use std::collections::HashMap;

use regex::Regex;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::{Width, terminal_size};

use super::{Directory, formatter::format_detailed_file_info, parser::Flag};

pub struct LsOutput;

impl LsOutput {
    pub fn print_results(
        file_result: &Vec<Vec<String>>,
        dir_results: &Vec<Directory>,
        directories_length: &usize,
        files_length: &usize,
        max_files_len: &usize,
        flags: &Flag,
    ) {
        // Print files
        if !file_result.is_empty() {
            let mut file_result_clone = file_result.clone();
            Self::print(&mut file_result_clone, max_files_len, flags);
            if !dir_results.is_empty() {
                println!();
            }
        }

        // Print directories
        for (i, dir) in dir_results.iter().enumerate() {
            if directories_length + files_length > 1 {
                println!("{}:", dir.path.display());
            }

            if flags.l {
                println!("total {}:", dir.total_blocks);
            }

            let mut entries_clone = dir.entries.clone();
            Self::print(&mut entries_clone, &dir.max_len, flags);
            if i < directories_length - 1 {
                println!();
            }
        }
    }

    fn strip_ansi_codes(s: &str) -> String {
        let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
        re.replace_all(s, "").to_string()
    }

    fn format_result(result: &Vec<Vec<String>>, term_width: usize) -> String {
        let mut grid = Grid::new(GridOptions {
            filling: Filling::Spaces(2),
            direction: Direction::TopToBottom,
        });

        let max_width = result
            .iter()
            .filter_map(|entry| entry.first())
            .map(|name| Self::strip_ansi_codes(name).len())
            .max()
            .unwrap_or(0);

        for s in result.iter() {
            let clean_name = Self::strip_ansi_codes(&s[0]);
            grid.add(Cell::from(clean_name.as_str()));
        }
        let col_width = max_width + 2; // max width + 2 spaces
        let max_cols = if col_width == 0 {
            1
        } else {
            (term_width / col_width).max(1)
        };
        println!("{max_cols}");
        let mut res = String::new();
        res.push_str(&grid.fit_into_columns(max_cols).to_string());
        // if grid.fit_into_width(term_width).is_some() {
        //     res.push_str(&grid.fit_into_columns(14).to_string());
        // } else {
        //     for (i, path) in result.iter().enumerate() {
        //         res.push_str(&path[0]);
        //         if i < result.len() - 1 {
        //             res.push_str("  ");
        //         }
        //     }
        // }
        res.push('\n');
        res
    }

    fn print(result: &mut Vec<Vec<String>>, max_size_len: &usize, flags: &Flag) {
        let mut max_lens: HashMap<usize, usize> = HashMap::new();

        if flags.l {
            for path in result.iter() {
                for (i, field) in path.iter().enumerate() {
                    let len = field.len();
                    let entry = max_lens.entry(i).or_insert(0);
                    if len > *entry {
                        *entry = len;
                    }
                }
            }
            for path in result.iter() {
                println!(
                    "{}",
                    format_detailed_file_info(&max_lens, path, max_size_len)
                );
            }
        } else {
            let term_width = if let Some((Width(w), _)) = terminal_size() {
                w as usize
            } else {
                100
            };

            let res = Self::format_result(result, term_width);

            print!("{res}");
        }
    }
}
