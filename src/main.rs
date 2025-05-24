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
        input_parser(input);
    }
}
