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
