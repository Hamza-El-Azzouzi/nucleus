use std::io;
use std::io::Write;
mod parser;
use parser::*;

fn main() {
    loop {
        print!("$");
        io::stdout().flush().expect("error happend while flushing");
        let mut input = String::new();

        let _ = io::stdin().read_line(&mut input);
        match input_parser(input.to_string()) {
            Ok(Commande::Exit) => {
                break;
            }

             Err(error) if error == "No command entered".to_string() => {
                continue;
            }
            Ok(command) => println!("{:?}", command),
            Err(err) => println!("Error: {}", err),
        }
    }
}
