pub fn echo(args: Vec<String>) {
    if args.is_empty() {
        println!();
        return;
    }
    let mut parsed_args = args
        .iter()
        .map(|arg| process_escape(arg))
        .collect::<Vec<String>>()
        .join(" ");
    if parsed_args.len() < 2 || !parsed_args.ends_with("\n\n") {
        parsed_args.push('\n');
        print!("{parsed_args}");
        return;
    }
    print!("{parsed_args}");
}

fn process_escape(arg: &str) -> String {
    let mut chars = arg.chars();
    let mut result = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            '\\' => {
                if let Some(next_char) = chars.next() {
                    match next_char {
                        'n' => result.push_str("\n\n"),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        '\\' => result.push('\\'),
                        _ => {
                            result.push('\\');
                            result.push(next_char);
                        }
                    }
                }
            }
            _ => {
                result.push(ch);
            }
        }
    }

    result
}
