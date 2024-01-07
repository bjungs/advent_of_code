use crate::almanac::Almanac;
use std::ops::Range;

fn get_seed_ranges(seeds_values: Vec<u64>) -> Vec<Range<u64>> {
    let mut range_start = None;
    let mut range_length = None;

    let mut ranges = Vec::new();
    for seed_value in seeds_values {
        if range_start.is_some() {
            if range_length.is_some() {
                let start = range_start
                    .take()
                    .expect("We just checked that this value is Some.");
                let length = range_length
                    .take()
                    .expect("We just checked that this value is Some.");
                ranges.push(start..start + length)
            } else {
                range_length = Some(seed_value);
            }
        } else {
            range_start = Some(seed_value);
        }
    }

    ranges
}

pub fn solve(input: &str) -> u64 {
    let seed_ranges: Vec<Range<u64>> = get_seed_ranges(
        input
            .lines()
            .nth(0)
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
