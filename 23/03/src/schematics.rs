use std::cmp::{max, min};

pub struct Schematic {
    pub parts: Vec<Part>,
}

#[derive(Debug)]
pub struct Part(pub u32);

impl From<&str> for Schematic {
    fn from(str: &str) -> Self {
        let mut lines = str.lines();
        let (mut maybe_top_line, mut maybe_mid_line, mut maybe_bot_line): (Option<&str>, _, _) =
            (None, lines.next(), lines.next());

        let mut parts = Vec::new();

        while let Some(mid_line) = maybe_mid_line {
            // find numbers
            let mid_line_chars: Vec<_> = mid_line.chars().collect();
            let mut index = 0;
            while index < mid_line_chars.len() {
                let char = mid_line_chars[index];
                let num_start_pos = index;
                let mut num_end_pos = index;
                let mut maybe_number = char.to_digit(10);
                match maybe_number {
                    None => {
                        index += 1;
                        continue;
                    }
                    Some(_) => {
                        let mut next_index = index + 1;
                        while next_index < mid_line_chars.len() {
                            // get next char
                            match mid_line_chars[next_index].to_digit(10) {
                                Some(next_digit) => {
                                    // if digit, add to the number and continue loop
                                    maybe_number = maybe_number.map(|v| v * 10 + next_digit);
                                    next_index += 1;
                                }
                                None => {
                                    break;
                                }
                            }
                        }
                        index = next_index;
                        num_end_pos = next_index - 1;
                    }
                }

                // check neighbours for symbols
                if let Some(number) = maybe_number {
                    // check mid for symbols
                    for neighbour_line_index in [max(num_start_pos, 1) - 1, min(num_end_pos + 1, mid_line_chars.len() - 1)] {
                        if is_symbol(mid_line_chars[neighbour_line_index]) {
                            // the number does indeed represent a Part
                            parts.push(Part(number));
                            break;
                        }
                    }

                    for maybe_line in [maybe_top_line, maybe_bot_line] {
                        if let Some(line) = maybe_line {
                            let indexes = Vec::from_iter(max(num_start_pos, 1) - 1..=min(num_end_pos + 1, mid_line_chars.len() - 1));
                            if has_neighbouring_symbol(line, indexes) {
                                parts.push(Part(number));
                                break;
                            }
                        }
                    }
                }
            }

            maybe_top_line = maybe_mid_line;
            maybe_mid_line = maybe_bot_line;
            maybe_bot_line = lines.next();
        }

        dbg!(&parts);

        Schematic {
            parts,
        }
    }
}

fn has_neighbouring_symbol(line: &str, indexes: Vec<usize>) -> bool {
    let chars: Vec<_> = line.chars().collect();
    has_symbol(chars, indexes)
}


fn has_symbol(chars: Vec<char>, indexes: Vec<usize>) -> bool {
    indexes.iter().any(|i| is_symbol(chars[*i]))
}

fn is_symbol(c: char) -> bool {
    match c {
        '.' => false,
        char if char.to_digit(10).is_some() => false,
        _ => true
    }
}
