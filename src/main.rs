use std::io::{ stdin, stdout, Write };
use shell::parser::*;
use shell::executor::execute;

fn main() {
    loop {
        print!("$");
        stdout().flush().expect("error happend while flushing");
        let mut input = String::new();

        let n = stdin().read_line(&mut input).unwrap();

        if n == 0 {
            println!();
            break;
        }

        match input_parser(input.to_string()) {
            Ok(Command::Exit) => {
                break;
            }
            
            Ok(command) => execute(command),
            Err(error) if error == "No command entered".to_string() => {
                continue;
            }
            Err(err) => println!("{}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Command;

    #[test]
    fn test_echo_command() {
        let result = input_parser("echo Hello world".to_string());
        assert_eq!(
            result,
            Ok(Command::Echo(vec!["Hello".into(), "world".into()]))
        );
    }

    #[test]
    fn test_cd_command() {
        let result = input_parser("cd /tmp".to_string());
        assert_eq!(result, Ok(Command::Cd(Some("/tmp".into()))));
    }

    #[test]
    fn test_pwd_command() {
        let result = input_parser("pwd".to_string());
        assert_eq!(result, Ok(Command::Pwd));
    }

    #[test]
    fn test_invalid_command() {
        let result = input_parser("foobar".to_string());
        assert_eq!(result, Err("Command 'foobar' not found".to_string()));
    }

    #[test]
    fn test_rm_recursive_command() {
        let result = input_parser("rm -r folder".to_string());
        assert_eq!(
            result,
            Ok(Command::Rm(vec!["folder".into()], true))
        );
    }
}    