#[derive(Debug)]
struct ScratchCard {
    lucky_numbers: Vec<u32>,
    game_numbers: Vec<u32>,
}

impl From<&str> for ScratchCard {
    fn from(str: &str) -> Self {
        ScratchCard {
            lucky_numbers: vec![],
            game_numbers: vec![],
        }
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

    dbg!(scratchcards);

    42
}
