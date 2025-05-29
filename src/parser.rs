use std::io::{ stdin, stdout, Write };
#[derive(Debug, PartialEq)]
pub enum Command {
    Echo(Vec<String>),
    Cd(Option<String>),
    Ls(Vec<char>, Option<String>),
    Pwd,
    Cp(Vec<String>),
    Cat(Vec<String>),
    Rm(Vec<String>, bool),
    Mv(Vec<String>),
    Mkdir(Vec<String>),
    Exit,
}

pub fn input_parser(input: String) -> Result<Command, String> {
    // let command: Vec<String> = input.trim().split_whitespace().map(String::from).collect();
    let command: Vec<String> = split(input.trim_end().to_string());

    if command.is_empty() {
        return Err("No command entered".to_string());
    }

    match command[0].as_str() {
        "echo" => Ok(Command::Echo(command[1..].to_vec())),

        "cd" => {
            let target = command.get(1).cloned();
            Ok(Command::Cd(target))
        }

        "ls" => {
            let (flags, path) = parse_ls_flags(&command[1..])?;
            Ok(Command::Ls(flags, path))
        }

        "pwd" => Ok(Command::Pwd),

        "cat" => {
            if command.len() < 2 {
                Err("cat: missing file operand".to_string())
            } else {
                Ok(Command::Cat(command[1..].to_vec()))
            }
        }

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
                Err("mv: missing file operand".to_string())
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
                Err("cp: missing file operand".to_string())
            } else {
                Ok(Command::Cp(command[1..].to_vec()))
            }
        }

        "exit" => Ok(Command::Exit),

        _ => Err(format!("Command '{}' not found", command[0])),
    }
}

fn parse_ls_flags(args: &[String]) -> Result<(Vec<char>, Option<String>), String> {
    let valid_flags = ['l', 'a', 'F'];
    let mut flags = Vec::new();
    let mut path = None;

    for arg in args {
        if arg.starts_with('-') {
            for ch in arg.chars().skip(1) {
                if valid_flags.contains(&ch) {
                    if !flags.contains(&ch) {
                        flags.push(ch);
                    }
                } else {
                    return Err(format!("ls: invalid option -- '{}'", ch));
                }
            }
        } else {
            if path.is_some() {
                return Err("ls: too many arguments".to_string());
            }
            path = Some(arg.clone());
        }
    }

    Ok((flags, path))
}

fn parse_rm_flags(args: &[String]) -> Result<(bool, Vec<String>), String> {
    let mut recursive = false;
    let mut files = Vec::new();

    for arg in args {
        if arg == "-r" {
            recursive = true;
        } else if arg.starts_with('-') {
            return Err(format!("rm: invalid option '{}'", arg));
        } else {
            files.push(arg.clone());
        }
    }

    Ok((recursive, files))
}

pub fn split(mut command: String) -> Vec<String> {
    let mut result = Vec::new();
    let mut word = String::new();
    let mut quote_char: Option<char> = None;
    let mut chars: Vec<char> = command.trim_end().chars().collect();
    let mut i = 0;
    let mut first_line = true;

    while i < chars.len() {
        let c = chars[i];

        match quote_char {
            Some(q) => {
                if c == q {
                    quote_char = None;
                    result.push(word.clone());
                    word.clear();
                } else {
                    word.push(c);
                }
            }
            None => {
                match c {
                    '\'' | '"' => {
                        quote_char = Some(c);
                    }
                    ' ' => {
                        if !word.is_empty() {
                            result.push(word.clone());
                            word.clear();
                        }
                    }
                    _ => {
                        word.push(c);
                    }
                }
            }
        }

        i += 1;

        // End of current buffer and still inside quote
        if i == chars.len() && quote_char.is_some() {
            print!("> ");
            stdout().flush().unwrap();

            let mut next_line = String::new();
            if stdin().read_line(&mut next_line).unwrap() == 0 {
                break;
            }
            if first_line {
                word.push('\n');
                first_line = false;
            }

            chars = next_line.chars().collect();
            i = 0;
        }
    }

    if !word.is_empty() {
        result.push(word);
    }
    if result[result.len()-1] == "\n" {
        return result[0..result.len()-1].to_vec();
    }
    result
}
