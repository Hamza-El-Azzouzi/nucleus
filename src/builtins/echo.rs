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
    let has_escape_sequences = args.iter().any(|arg| arg.contains('\\'));
    
    if !has_escape_sequences && parsed_args.chars().any(|c| !c.is_ascii_graphic() && !c.is_ascii_whitespace()) {
        println!();
        return;
    }
    if (parsed_args.len() < 2 || !parsed_args.ends_with("\n\n")) && !parsed_args.is_empty() {
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
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        '\\' => result.push('\\'),
                        'a' => result.push('\u{07}'), // Bell character
                        'b' => result.push('\u{08}'), // Backspace
                        'f' => result.push('\u{0C}'), // Form feed
                        'v' => result.push('\u{0B}'), // Vertical tab
                        '0' => result.push('\0'),     // Null character
                        _ => {
                            // For unknown escapes, just keep the backslash and character
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