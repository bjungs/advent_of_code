use crate::almanac::Almanac;
use std::ops::Range;

fn get_seed_ranges(seeds_values: Vec<u64>) -> Vec<Range<u64>> {
    seeds_values
        .chunks(2)
        .map(|chunk| {
            assert_eq!(
                chunk.len(),
                2,
                "The seed ranges input should always contain numbers in pairs"
            );
            let [start, length]: [u64; 2] = chunk.try_into().unwrap();
            start..(start + length)
        })
        .collect::<Vec<_>>()
}

pub fn solve(input: &str) -> u64 {
    let seed_ranges: Vec<Range<u64>> = get_seed_ranges(
        input
            .lines()
            .next()
            .expect("First line should contain the seed ranges.")
            .split_once(": ")
            .expect("Malformed seed numbers line.")
            .1
            .split_whitespace()
            .map(|value| value.parse().expect("Seed number should be a valid u64."))
            .collect(),
    );

    let almanac = Almanac::from(
        input
            .lines()
            .skip(2)
            .collect::<Vec<&str>>()
            .join("\n")
            .as_str(),
    );

    almanac.closest_seed_range(&seed_ranges)
}
