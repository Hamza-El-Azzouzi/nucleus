use std::collections::HashMap;

use regex::Regex;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions, Alignment};
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

        for s in result.iter() {
            let clean_name = Self::strip_ansi_codes(&s[0]);
            let cell = Cell {
                contents: s[0].clone(), 
                width: clean_name.len(),
                alignment: Alignment::Left,
            };
            grid.add(cell);
        }
        let mut res = String::new();
        if let Some(display) = grid.fit_into_width(term_width) {
            res.push_str(&display.to_string());
        } else {
            res.push_str(&Self::format_result_manual(result, term_width));
        }
        res
    }

    fn format_result_manual(result: &Vec<Vec<String>>, term_width: usize) -> String {
        if result.is_empty() {
            return String::new();
        }
    
        let names: Vec<String> = result
            .iter()
            .map(|s| s.first().cloned().unwrap_or_default())
            .collect();
    
        let mut best_cols = 1;
        let mut best_rows = names.len();
        
        for cols in 1..=names.len() {
            let rows: usize = names.len().div_ceil(cols);
            let mut col_widths = vec![0; cols];
            for (idx, name) in names.iter().enumerate() {
                let col = idx / rows;
                if col < cols {
                    let clean_len = Self::strip_ansi_codes(name).len();
                    col_widths[col] = col_widths[col].max(clean_len);
                }
            }
            let total_width: usize = col_widths.iter().sum::<usize>() + (cols - 1) * 2;
            
            if total_width <= term_width && rows <= best_rows {
                best_cols = cols;
                best_rows = rows;
            }
        }
        
        let cols = best_cols;
        let rows = best_rows;
        let mut col_widths = vec![0; cols];
        for (idx, name) in names.iter().enumerate() {
            let col = idx / rows;
            if col < cols {
                let clean_len = Self::strip_ansi_codes(name).len();
                col_widths[col] = col_widths[col].max(clean_len);
            }
        }
        let mut lines = Vec::new();
        for row in 0..rows {
            let mut line = String::new();
            for col in 0..cols {
                let idx = col * rows + row;
                if idx < names.len() {
                    let name = &names[idx];
                    let clean = Self::strip_ansi_codes(name);
                    let pad = col_widths[col] - clean.len();
                    line.push_str(name);
                    if col < cols - 1 && idx < names.len() - 1 {
                        for _ in 0..(pad + 2) {
                            line.push(' ');
                        }
                    }
                }
            }
            lines.push(line);
        }
    
        lines.join("\n") + "\n"
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
