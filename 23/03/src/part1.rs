use crate::part1::Element::{Empty, Symbol};

enum Element<'a> {
    Number(&'a u32),
    Symbol,
    Empty,
}

pub fn solve_old() -> u32 {
    let input = include_str!("../input/input.txt");

    let mut lines = input.lines();

    let (maybe_top, maybe_mid, maybe_bot): (Option<&str>, _, _) = (None, lines.next(), lines.next());

    let mut part_numbers: Vec<u32> = vec!();

    while let Some(mid) = maybe_mid {
        let maybe_top_chars: Option<Vec<_>>= maybe_top.map(|top| top.char_indices().collect());
        let mid_chars: Vec<_> = mid.char_indices().collect();
        let maybe_bot_chars: Option<Vec<_>> = maybe_bot.map(|top| top.char_indices().collect());

        'line: for (mut index, char) in &mid_chars {
            if let Some(mut number) = char.to_digit(10) {
                // character is a digit, peek next character(s) so we can find the number
                let mut next_index: usize = index + 1;
                while let Some(Some(next_digit)) = mid_chars.get(next_index).map(|(_, next_char)| next_char.to_digit(10)) {
                    dbg!(number);
                    number = number * 10 + next_digit;
                    next_index += 1;
                };

                // check for symbols around it
                if let Some(ref top_chars) = maybe_top_chars {
                    for lookup_index in index - 1..=next_index {
                        if let Some((_, char)) = top_chars.get(lookup_index) {
                            match char {
                                '.' => {}
                                _ => {
                                    if let None = char.to_digit(10) {
                                        // not a number, must be a symbol
                                        part_numbers.push(number);
                                        index = next_index;
                                        continue 'line;
                                    }
                                }
                            };
                        }
                    }
                }

                if let Some(ref bot_chars) = maybe_bot_chars {
                    for lookup_index in index - 1..=next_index {
                        if let Some((_, char)) = bot_chars.get(lookup_index) {
                            match char {
                                '.' => {}
                                _ => {
                                    if let None = char.to_digit(10) {
                                        // not a number, must be a symbol
                                        part_numbers.push(number);
                                        index = next_index;
                                        continue 'line;
                                    }
                                }
                            };
                        }
                    }
                }
            } else {
                println!("not number");
            }
        }

    }

    part_numbers.iter().sum()
}
