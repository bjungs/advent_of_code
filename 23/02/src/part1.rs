use day02::game::validation::CubeMap;
use day02::game::{Color, Game};

pub fn solve() -> u32 {
    let input = include_str!("../input/input.txt");

    input
        .lines()
        .map(Game::try_from)
        .map(Result::unwrap)
        .filter(|game| {
            game.is_possible(&CubeMap::from([
                (Color::Red, 12),
                (Color::Green, 13),
                (Color::Blue, 14),
            ]))
        })
        .map(|game| game.id as u32)
        .sum()
}
