#[derive(Debug)]
pub enum Command {
    Echo(Vec<String>),
    Cd(Option<String>),
    Ls(Vec<char>, Option<String>),
    Pwd,
    Cat(Vec<String>),
    Rm(Vec<String>, bool),
    Mv(Vec<String>),
    Mkdir(Vec<String>),
    Exit,
}

pub fn input_parser(input: String) -> Result<Command, String> {
    let command: Vec<String> = split(input);

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
fn split(command: String) -> Vec<String> {
    let mut res = Vec::new();
    let mut word = String::new();
    let mut in_quotes = false;
    let mut chars = command.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                // Don't include the quote character itself
            }
            ' ' if !in_quotes => {
                if !word.is_empty() {
                    res.push(word.clone());
                    word.clear();
                }
            }
            _ => {
                word.push(c);
            }
        }
    }

    if !word.is_empty() {
        res.push(word);
    }

    res
}
