#[derive(Debug)]
pub enum Commande {
    Echo(Vec<String>),
    Cd(Option<String>),
    Ls(Vec<char>),
    Pwd,
    Cat(Vec<String>),
    Rm(Vec<String>, bool),
    Mv(Vec<String>),
    Mkdir(Vec<String>),
    Exit,
}

pub fn input_parser(input: String) -> Result<Commande, String> {
    let commande: Vec<String> = input.trim().split_whitespace().map(String::from).collect();

    if commande.is_empty() {
        return Err("No command entered".to_string());
    }

    match commande[0].as_str() {
        "echo" => Ok(Commande::Echo(commande[1..].to_vec())),

        "cd" => {
            let target = commande.get(1).cloned();
            Ok(Commande::Cd(target))
        }

        "ls" => {
            let flags = parse_ls_flags(&commande[1..])?;
            Ok(Commande::Ls(flags))
        }

        "pwd" => Ok(Commande::Pwd),

        "cat" => {
            if commande.len() < 2 {
                Err("cat: missing file operand".to_string())
            } else {
                Ok(Commande::Cat(commande[1..].to_vec()))
            }
        }

        "rm" => {
            let (recursive, files) = parse_rm_flags(&commande[1..])?;
            if files.is_empty() {
                Err("rm: missing operand".to_string())
            } else {
                Ok(Commande::Rm(files, recursive))
            }
        }

        "mv" => {
            if commande.len() < 3 {
                Err("mv: missing file operand".to_string())
            } else {
                Ok(Commande::Mv(commande[1..].to_vec()))
            }
        }

        "mkdir" => {
            if commande.len() < 2 {
                Err("mkdir: missing operand".to_string())
            } else {
                Ok(Commande::Mkdir(commande[1..].to_vec()))
            }
        }

        "exit" => Ok(Commande::Exit),

        _ => Err(format!("Command '{}' not found", commande[0])),
    }
}

fn parse_ls_flags(args: &[String]) -> Result<Vec<char>, String> {
    let valid_flags = ['l', 'a', 'F'];
    let mut flags = Vec::new();

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
        }
    }

    Ok(flags)
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
