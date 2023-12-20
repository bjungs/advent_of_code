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

impl From<&str> for ScratchCard {
    fn from(str: &str) -> Self {
        ScratchCard::new(vec![], vec![])
    }
}

pub fn solve(input: &str) -> u32 {
    let scratchcards: Vec<ScratchCard> = input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .expect("format should be Card {number}: {lucky_numbers} | {game_numbers}")
        })
        .map(|(_, sc_input)| ScratchCard::from(sc_input))
        .collect();

    dbg!(&scratchcards);

    scratchcards.iter().map(|sc| sc.calculate_points()).sum()
}
