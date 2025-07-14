use std::env;
use std::io::{ Write, stdin, stdout };

use std::string::String;
#[derive(Debug, PartialEq)]
pub enum Command {
    Echo(Vec<String>),
    Cd(Vec<String>),
    Ls(Vec<String>),
    Pwd,
    Cp(Vec<String>),
    Cat(Vec<String>),
    Rm(Vec<String>, bool),
    Mv(Vec<String>),
    Mkdir(Vec<String>),
    Exit,
}

pub fn input_parser(input: String) -> Result<Command, String> {
    let command: Vec<String> = split(&input);
    if command.is_empty() {
        return Err("No command entered".to_string());
    }

    match command[0].as_str() {
        "echo" => Ok(Command::Echo(command[1..].to_vec())),

        "cd" => Ok(Command::Cd(command[1..].to_vec())),

        "ls" => Ok(Command::Ls(command[1..].to_vec())),

        "pwd" => Ok(Command::Pwd),

        "cat" => Ok(Command::Cat(command[1..].to_vec())),

        "rm" => {
            let (recursive, files) = parse_rm_flags(&command[1..])?;
            if files.is_empty() {
                Err("rm: missing operand".to_string())
            } else {
                Ok(Command::Rm(files, recursive))
            }
        }

        "mv" => {
            if command.len() < 3 {
                let mut err = "mv: missing file operand".to_string();
                if command.len() == 2 {
                    err = format!("mv: missing destination file operand after '{}'", command[1]);
                }
                Err(err)
            } else {
                Ok(Command::Mv(command[1..].to_vec()))
            }
        }

        "mkdir" => {
            if command.len() < 2 {
                Err("mkdir: missing operand".to_string())
            } else {
                Ok(Command::Mkdir(command[1..].to_vec()))
            }
        }
        "cp" => {
            if command.len() < 3 {
                let mut err = "cp: missing file operand".to_string();
                if command.len() == 2 {
                    err = format!("cp: missing destination file operand after '{}'", command[1]);
                }
                Err(err)
            } else {
                Ok(Command::Cp(command[1..].to_vec()))
            }
        }

        "exit" => {
            if command.len() > 1 && command[1].parse::<isize>().is_err() {
                let err = format!("exit: Illegal number: {}", command[1]);
                Err(err)
            } else {
                Ok(Command::Exit)
            }
        }

        _ => Err(format!("Command '{}' not found", command[0])),
    }
}

fn parse_rm_flags(args: &[String]) -> Result<(bool, Vec<String>), String> {
    let mut recursive = false;
    let mut files = Vec::new();

    for arg in args {
        if arg == "-r" {
            recursive = true;
        } else if arg.starts_with('-') {
            return Err(format!("rm: invalid option '{arg}'"));
        } else {
            files.push(arg.clone());
        }
    }

    Ok((recursive, files))
}

pub fn split(command: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut word = String::new();
    let mut quote_char: Option<char> = None;
    let mut chars: Vec<char> = command.chars().collect();
    let mut i = 0;
    let mut escape_next = false;
    let mut quoted_words = Vec::new();
    let mut word_was_quoted = false;

    loop {
        while i < chars.len() {
            let c = chars[i];

            if escape_next {
                if c != '\n' {
                    word.push(c);
                }
                escape_next = false;
            } else {
                match quote_char {
                    Some(q) => {
                        match c {
                            '\\' => {
                                if q == '"' {
                                    // In double quotes, handle escape sequences
                                    if i + 1 < chars.len() {
                                        let next = chars[i + 1];
                                        match next {
                                            '\\' | '"' | '\n' => {
                                                i += 1;
                                                if next != '\n' {
                                                    word.push(next);
                                                }
                                            }
                                            _ => word.push('\\'),
                                        }
                                    } else {
                                        word.push('\\');
                                    }
                                } else {
                                    // In single quotes, backslash is literal
                                    word.push('\\');
                                }
                            }
                            c if c == q => {
                                quote_char = None;
                                word_was_quoted = true;
                            }
                            '\n' => {
                                // Newlines are preserved in quoted strings
                                word.push('\n');
                                word_was_quoted = true;
                            }
                            _ => {
                                word.push(c);
                                word_was_quoted = true;
                            }
                        }
                    }
                    None => {
                        match c {
                            '\\' => {
                                if i + 1 < chars.len() && chars[i + 1] == '\n' {
                                    // Line continuation - skip both \ and \n
                                    i += 2;
                                    continue;
                                } else if i + 1 >= chars.len() {
                                    // Backslash at end of input - line continuation needed
                                    break;
                                } else {
                                    // Regular escape
                                    escape_next = true;
                                }
                            }
                            '\'' | '"' => {
                                quote_char = Some(c);
                            }
                            c if c.is_whitespace() => {
                                if !word.is_empty() {
                                    result.push(word.clone());
                                    quoted_words.push(word_was_quoted);
                                    word.clear();
                                    word_was_quoted = false;
                                }
                            }
                            _ => {
                                word.push(c);
                            }
                        }
                    }
                }
            }

            i += 1;
        }

        // Check if we need continuation using your original logic
        let mut rev_chars = chars
            .iter()
            .rev()
            .filter(|&&c| c != '\n');
        let last = rev_chars.next();
        let second_last = rev_chars.next();

        let has_backslash_continuation = last == Some(&'\\') && second_last != Some(&'\\');
        let has_unclosed_quote = quote_char.is_some();
        let needs_continuation = has_backslash_continuation || has_unclosed_quote;
        
        if needs_continuation {
            print!("> ");
            if let Err(e) =stdout().flush() {
                eprintln!("Failed to flush stdout: {e}");
                break;
            }
            let mut next_line = String::new();
            match stdin().read_line(&mut next_line) {
                Ok(0) => {
                    break;
                }
                Ok(_) => {
                    if has_backslash_continuation {
                        if next_line == "\n" {
                            chars.pop();
                            break;
                        }
                        if next_line.trim_end() == "\\" {
                            continue;
                        }
                        
                        chars.pop(); // Remove the trailing backslash
                        let start_pos = chars.len();
                        chars.extend(next_line.chars());
                        // Continue from where we added the new content
                        i = start_pos;
                    } else {
                        word.push('\n');
                        chars.extend(next_line.chars());
                    }
                    continue;
                }
                Err(e) => {
                    eprintln!("Failed to read line: {e}");
                    break;
                }
            }
        } else {
            break;
        }
    }

    if !word.is_empty() {
        result.push(word);
        quoted_words.push(word_was_quoted);
    }
    
    result
        .into_iter()
        .zip(quoted_words)
        .map(|(cmd, quoted)| {
            if !quoted &&
                cmd.starts_with('~') &&
                (cmd.len() == 1 || cmd.chars().nth(1) == Some('/'))
            {
                if let Ok(home) = env::var("HOME") { 
                    format!("{}{}", home, &cmd[1..]) 
                } else { 
                    cmd 
                }
            } else {
                cmd
            }
        })
        .collect()
}