use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else {
        // Match on different character classes
        match pattern {
            "\\d" => input_line.chars().filter(|c| c.is_digit(10)).count() > 0,
            "\\w" => {
                input_line
                    .chars()
                    .filter(|c| c.is_ascii_alphanumeric())
                    .count()
                    > 0
            }
            _ => {
                if pattern.starts_with('[') && pattern.ends_with(']') {
                    let inside_group: Vec<_> = pattern
                        .strip_prefix('[')
                        .expect("Pattern should start with [")
                        .strip_suffix(']')
                        .expect("Pattern should end with ]")
                        .chars()
                        .collect();
                    if inside_group.starts_with(&['^']) {
                        let negative_char_group =
                            inside_group.strip_prefix(&['^']).expect("Starts with ^");
                        input_line
                            .chars()
                            .all(|c| !negative_char_group.contains(&c))
                    } else {
                        input_line.chars().any(|c| inside_group.contains(&c))
                    }
                } else {
                    panic!("Unhandled Pattern");
                }
            }
        }
    }
}

// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        // println!("Pattern Found");
        process::exit(0)
    } else {
        // println!("Pattern not found");
        process::exit(1)
    }
}
