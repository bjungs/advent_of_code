#[derive(Debug)]
pub struct ScratchCard {
    lucky_numbers: Vec<u32>,
    game_numbers: Vec<u32>,
}

pub struct ScratchCardCollection(Vec<ScratchCard>);

impl ScratchCardCollection {
    pub fn calculate_points(&self) -> u32 {
        self.0.iter().map(|sc| sc.calculate_points()).sum()
    }

    pub fn count_winning(&self) -> u32 {
        self.0
            .iter()
            .enumerate()
            .fold(
                Vec::new(),
                |mut sc_count_vec: Vec<(&ScratchCard, u32)>, (idx, sc)| {
                    let count = match sc_count_vec.get(idx) {
                        Some((_, count)) => *count,
                        None => {
                            // set initial count
                            sc_count_vec.push((&sc, 1));
                            1
                        }
                    };

                    // for every winning number, add 1 more of the next SCs for every copy of this SC
                    let winning_numbers_count = sc.winning_numbers().iter().count();
                    for winning_number_idx in 0..winning_numbers_count {
                        let next_sc_idx = idx + winning_number_idx + 1;
                        match sc_count_vec.get_mut(next_sc_idx) {
                            Some((_, next_count)) => {
                                *next_count += count;
                            }
                            None => {
                                sc_count_vec.insert(next_sc_idx, (sc, 1 + count));
                            }
                        }
                    }

                    sc_count_vec
                },
            )
            .iter()
            .map(|(_, count)| *count)
            .sum()
    }
}

impl From<&str> for ScratchCardCollection {
    fn from(str: &str) -> Self {
        let scratchcards = str.lines()
            .map(|line| {
                line.split_once(": ")
                    .expect("format should be \"Card {number}: {lucky_numbers} | {game_numbers}\"")
            })
            .map(|(_, sc_input)| ScratchCard::from(sc_input))
            .collect();

        ScratchCardCollection(scratchcards)
    }
}

impl ScratchCard {
    pub fn calculate_points(&self) -> u32 {
        self.winning_numbers()
            .iter()
            .fold(0, |points, _| match points {
                0 => 1,
                _ => points * 2,
            })
    }

    pub fn winning_numbers(&self) -> Vec<&u32> {
        self.game_numbers
            .iter()
            .filter(|&number| self.lucky_numbers.contains(number))
            .collect()
    }
}

fn number_sequence(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|value| {
            value
                .parse::<u32>()
                .expect("Scratchcard numbers should be valid u32")
        })
        .collect()
}

impl From<&str> for ScratchCard {
    fn from(str: &str) -> Self {
        let (lucky_numbers_str, game_numbers_str) = str
            .split_once(" | ")
            .expect("Scratchcard format should be \"{lucky_numbers} | {game_numbers}\"");

        let lucky_numbers = number_sequence(lucky_numbers_str);
        let game_numbers = number_sequence(game_numbers_str);

        ScratchCard {
            lucky_numbers,
            game_numbers,
        }
    }
}
