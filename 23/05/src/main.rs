mod almanac;
mod part1;
mod part2;

fn main() {
    let input = include_str!("../input/input.txt");

    println!("Day 05");
    println!("Part 1: {}", part1::solve(input));
    println!("Part 2: {}", part2::solve(input));
}
