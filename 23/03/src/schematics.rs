use std::cmp::{max, min};
use std::collections::HashMap;

pub struct Schematic {
    pub parts: Vec<Part>,
    pub gears: Vec<Gear>,
}

#[derive(Debug, Clone, Copy)]
pub struct Part(pub u32);

#[derive(Debug)]
pub struct Gear(pub Part, pub Part);

#[derive(Debug)]
enum Star {
    OneNeighbour(Part),
    TwoNeighbours(Part, Part),
    TooManyNeighbours,
}

impl Star {
    fn add_neighbour(&mut self, part: Part) {
        *self = match *self {
            Star::OneNeighbour(p1) => Star::TwoNeighbours(p1, part),
            Star::TwoNeighbours(_, _) => Star::TooManyNeighbours,
            Star::TooManyNeighbours => Star::TooManyNeighbours,
        };
    }
}

impl From<&str> for Schematic {
    fn from(str: &str) -> Self {
        let mut lines = str.lines().enumerate();
        let (mut maybe_top_line, mut maybe_mid_line, mut maybe_bot_line) =
            (None, lines.next(), lines.next());

        let mut parts: Vec<Part> = Vec::new();
        let mut stars: HashMap<(usize, usize), Star> = HashMap::new();

        while let Some((mid_line_index, mid_line)) = maybe_mid_line {
            // find numbers
            let mid_line_chars: Vec<_> = mid_line.chars().collect();
            let mut index = 0;
            while index < mid_line_chars.len() {
                let curr_char = mid_line_chars[index];
                if let Some(digit) = curr_char.to_digit(10) {
                    let mut number = digit;
                    let mut next_index = index + 1;

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

                    let num_start_pos = index;
                    let num_end_pos = next_index - 1;

                    // check mid line for neighbouring symbols
                    let mid_line_neighbours = [
                        num_start_pos.saturating_sub(1),
                        min(num_end_pos + 1, mid_line_chars.len() - 1),
                    ];
                    for neighbour_index in mid_line_neighbours {
                        let char = mid_line_chars[neighbour_index];
                        if is_symbol(&char) {
                            // the number is a Part Number
                            let part = Part(number);
                            parts.push(part);
                            if char == '*' {
                                stars
                                    .entry((mid_line_index, neighbour_index))
                                    .and_modify(|star| star.add_neighbour(part))
                                    .or_insert(Star::OneNeighbour(part));
                            }
                        }
                    }

                    // check adjacent lines for neighbouring symbols
                    for maybe_line in [maybe_top_line, maybe_bot_line] {
                        if let Some((line_index, line)) = maybe_line {
                            let chars: Vec<_> = line.chars().collect();
                            let neighbours = max(num_start_pos, 1) - 1
                                ..=min(num_end_pos + 1, mid_line_chars.len() - 1);

                            for neighbour_index in neighbours {
                                let char = chars[neighbour_index];
                                if is_symbol(&char) {
                                    // the number is a Part Number
                                    let part = Part(number);
                                    parts.push(part);
                                    if char == '*' {
                                        stars
                                            .entry((line_index, neighbour_index))
                                            .and_modify(|star| star.add_neighbour(part))
                                            .or_insert(Star::OneNeighbour(part));
                                    }
                                }
                            }
                        }
                    }
                    index = next_index;
                } else {
                    index += 1;
                }
            }

            // move down one line
            maybe_top_line = maybe_mid_line;
            maybe_mid_line = maybe_bot_line;
            maybe_bot_line = lines.next();
        }

        Schematic {
            parts,
            gears: stars
                .into_values()
                .filter_map(|s| match s {
                    Star::TwoNeighbours(p1, p2) => Some(Gear(p1, p2)),
                    _ => None,
                })
                .collect(),
        }
    }
}

fn is_symbol(c: &char) -> bool {
    match c {
        '.' => false,
        char if char.to_digit(10).is_some() => false,
        _ => true,
    }
}
