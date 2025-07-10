use shell::executor::execute;
use shell::parser::*;
use std::io::{Write, stdin, stdout};

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
            Err(error) if error == "No command entered" => {
                continue;
            }
            Err(err) => println!("{err}", ),
        }
    }
}
