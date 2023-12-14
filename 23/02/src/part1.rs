use day02::game::{Color, Game};
use std::collections::HashMap;

pub fn solve() -> u32 {
    let input = include_str!("../input/part1.txt");

    input
        .lines()
        .map(Game::try_from)
        .map(Result::unwrap)
        .filter(|game| {
            game.is_possible(&HashMap::from([
                (Color::Red, 12),
                (Color::Green, 13),
                (Color::Blue, 14),
            ]))
        })
        .map(|game| game.id as u32)
        .sum()
}
