#[derive(Debug)]
struct ScratchCard {
    lucky_numbers: Vec<u32>,
    game_numbers: Vec<u32>,
}

impl ScratchCard {
    pub fn new(lucky_numbers: Vec<u32>, game_numbers: Vec<u32>) -> ScratchCard {
        ScratchCard {
            lucky_numbers,
            game_numbers,
        }
    }

    fn calculate_points(&self) -> u32 {
        42
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

        ScratchCard::new(lucky_numbers, game_numbers)
    }
}

pub fn solve(input: &str) -> u32 {
    let scratchcards: Vec<ScratchCard> = input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .expect("format should be \"Card {number}: {lucky_numbers} | {game_numbers}\"")
        })
        .map(|(_, sc_input)| ScratchCard::from(sc_input))
        .collect();

    dbg!(&scratchcards);

    scratchcards.iter().map(|sc| sc.calculate_points()).sum()
}
