use crate::almanac::Almanac;

pub fn solve(input: &str) -> u64 {
    let seeds: Vec<u64> = input
        .lines()
        .next()
        .expect("First line should contain the seed numbers")
        .split_once(": ")
        .expect("Malformed seed numbers line.")
        .1
        .split_whitespace()
        .map(|value| value.parse().expect("Seed number should be a valid u64."))
        .collect();

    let almanac = Almanac::from(
        input
            .lines()
            .skip(2)
            .collect::<Vec<&str>>()
            .join("\n")
            .as_str(),
    );

    almanac.closest_seed(&seeds)
}
