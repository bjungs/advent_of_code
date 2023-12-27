mod almanac;
mod part1;

fn main() {
    let input = include_str!("../input/test_input.txt");

    println!("Day 05");
    println!("Part 1: {}", part1::solve(input));
    // println!("Part 2: {}", part2::solve(input));
}
