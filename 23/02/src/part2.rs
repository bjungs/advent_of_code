use day02::game::Game;

pub fn solve() -> u32 {
    let input = include_str!("../input/input.txt");

    input
        .lines()
        .map(Game::try_from)
        .map(Result::unwrap)
        .map(|game| game.max())
        .map(|max_cubes| max_cubes.values().product::<u32>())
        .sum()
}
