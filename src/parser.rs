//echo,cd ,ls (supporting -l, -a, -F),pwd,cat,cp,rm (supporting -r) ,mv,mkdir,exit
enum Commande {
    Echo,
    Cd,
    Ls(Vec<String>),
    Pwd,
    Cat,
    Rm,
    Mv,
    Mkdir,
    Exit,
}

pub fn input_parser(input: String) -> Result<Commande, String> {
    let commande: Vec<String> = input.trim().split_whitespace().map(String::from).collect();
    if commande.is_empty() {
        Err(format!("Command {} not found", input));
    }
    let _ = match commande[0]. {
        "echo" => Ok(Commande::Echo),
        "cd" => Ok(Commande::Cd),
        "ls" => {
            let flags = parse_ls_flags(&commande)?;
            Ok(Commande::Ls(flags))
        }
        "pwd" => Ok(Commande::Pwd),
        "cat" => Ok(Commande::Cat),
        "rm" => Ok(Commande::Rm),
        "mv" => Ok(Commande::Mv),
        "mkdir" => Ok(Commande::Mkdir),
        "exit" => Ok(Commande::Exit),
        _ => Err(format!("Command {} not found", input)),
    };
}

fn parse_ls_flags(args: &[String]) -> Result<Vec<char>, String> {
    let valid_flags = ['l', 'a', 'F'];
    let mut flags = Vec::new();

    for arg in args.iter().skip(1) {
        if arg.starts_with('-') {
            for ch in arg.chars().skip(1) {
                if valid_flags.contains(&ch) {
                    if !flags.contains(&ch) {
                        flags.push(ch);
                    }
                } else {
                    return Err(format!("Invalid ls flag: -{}", ch));
                }
            }
        }
    }

    Ok(flags)
}
