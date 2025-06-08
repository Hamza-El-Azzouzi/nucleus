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
            Err(err) => println!("{}", err),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

   #[test]
fn test_basic_words() {
    assert_eq!(split("echo hello world"), vec!["echo", "hello", "world"]);
}

#[test]
fn test_escaped_newline() {
    assert_eq!(split("echo a\\\na"), vec!["echo", "ana"]);
}

#[test]
fn test_multiple_backslashes_before_newline() {
    assert_eq!(split("echo a\\\\\\na"), vec!["echo", "a\\a"]);
    assert_eq!(split("echo a\\\\\\\\na"), vec!["echo", "a\\\\na"]);
}

#[test]
fn test_double_quoted_string() {
    assert_eq!(split("echo \"aaaa\""), vec!["echo", "aaaa"]);
    assert_eq!(split("echo \"aa\\n aa\""), vec!["echo", "aa\\n aa"]);
    assert_eq!(split("echo \"aa\\\\n aa\""), vec!["echo", "aa\\\\n aa"]);
    assert_eq!(split("echo \"aa\\\" aa\""), vec!["echo", "aa\" aa"]);
}

#[test]
fn test_single_quoted_string() {
    assert_eq!(split("echo 'aaaa'"), vec!["echo", "aaaa"]);
    assert_eq!(split("echo 'aa\\n aa'"), vec!["echo", "aa\\n aa"]);
    assert_eq!(split("echo 'aa\\\\n aa'"), vec!["echo", "aa\\\\n aa"]);
    assert_eq!(split("echo 'aa\\' aa'"), vec!["echo", "aa\\' aa"]);
}

#[test]
fn test_mixed_arguments() {
    assert_eq!(split("echo \"hello\"world"), vec!["echo", "helloworld"]);
    assert_eq!(split("echo 'hello'world"), vec!["echo", "helloworld"]);
}

}
