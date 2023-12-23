mod part1;
mod part2;
mod scratchcard;

fn main() {
    let input = include_str!("../input/input.txt");

    println!("Day 04");
    println!("Part 1: {}", part1::solve(input));
    println!("Part 2: {}", part2::solve(input));
}
