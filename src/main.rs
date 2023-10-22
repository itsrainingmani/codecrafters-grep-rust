use std::env;
use std::io;
use std::process;

// fn match_here(input_iter: Iter<&str>, pattern: Iter<&str>) -> bool {
//     true
// }
//
#[derive(Debug)]
enum Pattern {
    Digit,
    Alphanumeric,
    PositiveGroup(String),
    NegativeGroup(String),
    Normal(char),
}

fn parse_pattern(pattern: &str) -> Vec<Pattern> {
    let patt_chars = pattern.chars().collect::<Vec<char>>();

    let mut patt_vec: Vec<Pattern> = Vec::new();
    let mut patt_pos = 0;
    while patt_pos < patt_chars.len() {
        // check if we are evaluating a pattern
        let cur_pattern;
        let cur_char = patt_chars.get(patt_pos).unwrap();
        if cur_char.eq(&'\\') {
            match patt_chars.get(patt_pos + 1) {
                Some(c) => {
                    cur_pattern = if let 'd' = c {
                        Pattern::Digit
                    } else {
                        Pattern::Alphanumeric
                    };
                    patt_pos += 2;
                }
                None => panic!("Unhandled Pattern"),
            }
            patt_vec.push(cur_pattern);
        } else if cur_char.eq(&'[') {
            // Look for char groups
            let pos_end_group = patt_chars
                .iter()
                .position(|&x| x == ']')
                .expect("Unhandled Pattern");
            let inside_group: String = patt_chars[patt_pos + 1..pos_end_group].iter().collect();
            cur_pattern = if inside_group.starts_with('^') {
                Pattern::NegativeGroup(String::from(
                    inside_group
                        .strip_prefix('^')
                        .expect("pattern should be starting with a ^"),
                ))
            } else {
                Pattern::PositiveGroup(String::from(inside_group))
            };
            patt_vec.push(cur_pattern);

            patt_pos = pos_end_group + 1;
        } else {
            cur_pattern = Pattern::Normal(*cur_char);
            patt_vec.push(cur_pattern);
            patt_pos += 1;
        };
    }

    patt_vec
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        // Single Char matching
        return input_line.contains(pattern);
    } else {
        let mut input_chars = input_line.trim().chars().enumerate();
        let patterns = parse_pattern(pattern);
        println!("{:?}", &patterns);

        let mut patt_idx = 0;

        while let Some((char_idx, cur_char)) = input_chars.next() {
            if let Some(p) = patterns.get(patt_idx) {
                let is_match = match p {
                    Pattern::Digit => cur_char.is_digit(10),
                    Pattern::Alphanumeric => cur_char.is_alphanumeric(),
                    Pattern::PositiveGroup(grp) => grp.contains(cur_char),
                    Pattern::NegativeGroup(grp) => !grp.contains(cur_char),
                    Pattern::Normal(c) => cur_char.eq(c),
                };

                if is_match {
                    // println!("{:?} - {:?} | {:?} - {:?}", cur_char, p, char_idx, patt_idx);
                    patt_idx += 1;
                } else {
                    patt_idx = 0;
                }
            } else {
                if char_idx < input_line.len() {
                    return true;
                }
                patt_idx = 0;
            }
        }

        // println!("{:?}", patt_idx);

        if patt_idx == patterns.len() {
            return true;
        }

        false
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _literal() {
        let input_line: &str = "apple123";
        let pattern: &str = "a";

        assert_eq!(match_pattern(input_line, pattern), true);
    }

    #[test]
    fn match_digits() {
        let input_line: &str = "apple123";
        let pattern: &str = "\\d";

        assert_eq!(match_pattern(input_line, pattern), true);
    }

    #[test]
    fn match_alphanumerics() {
        let input_line: &str = "alpha-num3ric";
        let pattern: &str = "\\w";

        assert_eq!(match_pattern(input_line, pattern), true);
    }

    #[test]
    fn match_positive_group() {
        let input_line: &str = "apple123";
        let pattern: &str = "[abc]";

        assert_eq!(match_pattern(input_line, pattern), true);
    }

    #[test]
    fn match_negative_group() {
        let input_line: &str = "hello";
        let pattern: &str = "[^abc]";

        assert_eq!(match_pattern(input_line, pattern), true);
    }
}
