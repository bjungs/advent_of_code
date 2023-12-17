use std::cmp::{max, min};

pub struct Schematic {
    pub parts: Vec<Part>,
}

#[derive(Debug)]
pub struct Part(pub u32);

impl From<&str> for Schematic{
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
                if let Some(mut number) = mid_line_chars[index].to_digit(10) {
                    let mut next_index = index + 1;
                    let num_start_pos = index;
                    let num_end_pos;
                    while next_index < mid_line_chars.len() {
                        // get next char
                        if let Some(next_digit) = mid_line_chars[next_index].to_digit(10) {
                            // if digit, add to the number and continue loop
                            number = number * 10 + next_digit;
                            next_index += 1;
                        } else {
                            break;
                        }
                    }
                    index = next_index;
                    num_end_pos = next_index - 1;

                    // check mid line for neighbouring symbols
                    let neighbours = [
                        max(num_start_pos, 1) - 1,
                        min(num_end_pos + 1, mid_line_chars.len() - 1),
                    ];
                    for neighbour_index in neighbours {
                        if is_symbol(mid_line_chars[neighbour_index]) {
                            // the number does indeed represent a Part
                            parts.push(Part(number));
                            break;
                        }
                    }

                    // check adjacent lines for neighbouring symbols
                    for maybe_line in [maybe_top_line, maybe_bot_line] {
                        if let Some(line) = maybe_line {
                            let neighbours = &line[max(num_start_pos, 1) - 1
                                ..=min(num_end_pos + 1, mid_line_chars.len() - 1)];
                            if neighbours.chars().any(is_symbol) {
                                parts.push(Part(number));
                                break;
                            }
                        }
                    }
                } else {
                    // check if it is a gear, and check its neighbours
                    index += 1;
                }
            }

            // move down one line
            maybe_top_line = maybe_mid_line;
            maybe_mid_line = maybe_bot_line;
            maybe_bot_line = lines.next();
        }

        Schematic { parts }
    }
}

fn is_symbol(c: char) -> bool {
    match c {
        '.' => false,
        char if char.to_digit(10).is_some() => false,
        _ => true,
    }
}
