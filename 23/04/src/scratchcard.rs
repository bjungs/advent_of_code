#[derive(Debug)]
pub struct ScratchCard {
    lucky_numbers: Vec<u32>,
    game_numbers: Vec<u32>,
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
