use std::env;
use std::io;
use std::process;

#[derive(Debug)]
enum Pattern {
    Digit,
    Alphanumeric,
    PositiveGroup(String),
    NegativeGroup(String),
    OneOrMore(char),
    Symbol(char),
    StartAnchor,
    EndAnchor,
}

fn parse_pattern(pattern: &str) -> Vec<Pattern> {
    let patt_chars = pattern.chars().collect::<Vec<char>>();

    let mut patt_vec: Vec<Pattern> = Vec::new();
    let mut patt_pos = 0;
    while let Some(cur_char) = patt_chars.get(patt_pos) {
        let cur_pattern;
        match cur_char {
            '\\' => {
                if let Some(c) = patt_chars.get(patt_pos + 1) {
                    patt_pos += 2;
                    cur_pattern = if *c == 'd' {
                        Pattern::Digit
                    } else {
                        Pattern::Alphanumeric
                    }
                } else {
                    panic!("Unhandled Pattern");
                }
            }
            '[' => {
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

                patt_pos = pos_end_group + 1;
            }
            '^' => {
                // Start of String or Line anchor
                cur_pattern = Pattern::StartAnchor;
                patt_pos += 1;
            }
            '$' => {
                // End of String or Line anchor
                cur_pattern = Pattern::EndAnchor;
                patt_pos += 1;
            }
            _ => {
                if let Some(c) = patt_chars.get(patt_pos + 1) {
                    if c.eq(&'+') {
                        cur_pattern = Pattern::OneOrMore(*cur_char);
                        patt_pos += 1;
                    // } else if c.eq(&'?') {
                    //     cur_pattern = Pattern::ZeroOrMore(*cur_char);
                    //     patt_pos += 1;
                    } else {
                        cur_pattern = Pattern::Symbol(*cur_char);
                    }
                } else {
                    cur_pattern = Pattern::Symbol(*cur_char);
                }
                patt_pos += 1;
            }
        }

        patt_vec.push(cur_pattern);
    }

    patt_vec
}

fn match_pattern(input_line: &str, patterns: &Vec<Pattern>) -> bool {
    let input_chars = input_line.trim_end().chars().collect::<Vec<char>>();
    eprintln!("{:?}", patterns);

    let mut char_idx = 0;
    let mut patt_idx = 0;
    let mut quant_count = 0;

    let first_patt = patterns.first().unwrap();

    if matches!(first_patt, Pattern::StartAnchor) {
        patt_idx += 1;
    }

    while let Some(cur_char) = input_chars.get(char_idx) {
        if let Some(p) = patterns.get(patt_idx) {
            eprintln!("{cur_char} - {p:?} | {char_idx} - {patt_idx}");
            let is_match = match p {
                Pattern::Digit => cur_char.is_digit(10),
                Pattern::Alphanumeric => cur_char.is_alphanumeric(),
                Pattern::PositiveGroup(grp) => grp.contains(*cur_char),
                Pattern::NegativeGroup(grp) => !grp.contains(*cur_char),
                Pattern::Symbol(c) => cur_char.eq(c),
                Pattern::OneOrMore(c) => {
                    if *c == *cur_char {
                        quant_count += 1;
                        char_idx += 1;
                        continue;
                    } else {
                        if quant_count >= 1 {
                            // the one or more test has been met
                            // move on to check the next pattern
                            patt_idx += 1;
                            continue;
                        } else {
                            // one or more test has not been met
                            // false match -> reset pattern checking
                            false
                        }
                    }
                }
                Pattern::StartAnchor => {
                    // If we see a line anchor at any point except the start of the
                    // pattern matching, we're done
                    return false;
                }
                Pattern::EndAnchor => false,
            };

            if is_match {
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
        char_idx += 1;
    }

    // println!("{:?}", patt_idx);
    if patt_idx == patterns.len() {
        return true;
    } else if patterns.len() - patt_idx == 1 {
        return matches!(patterns.last().unwrap(), Pattern::EndAnchor);
    }

    false
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
    let patterns = parse_pattern(&pattern);

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &patterns) {
        // println!("Pattern Found");
        process::exit(0)
    } else {
        // println!("Pattern not found");
        process::exit(1)
    }
}
