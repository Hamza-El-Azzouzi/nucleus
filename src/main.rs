use shell::prelude::*;

fn main() {
    loop {
        match current_dir() {
            Ok(p) => print_cur_dir(p),
            Err(_) => print!("$ "),
        }

        match stdout().flush() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        }
        
        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    println!();
                    break;
                }
            }
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        }

        match input_parser(input.to_string()) {
            Ok(Command::Exit) => {
                break;
            }

            Ok(command) => execute(command),
            Err(error) if error == "No command entered" => {
                continue;
            }
            Err(err) => println!("{err}"),
        }
    }
}
