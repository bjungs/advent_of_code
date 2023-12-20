mod part1;
fn main() {
    let input = include_str!("../input/input.txt");

    println!("Day 04");
    println!("Part 1: {}", part1::solve(input));
}
