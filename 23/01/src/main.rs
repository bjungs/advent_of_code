// --- Day 1: Trebuchet?! ---
// Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
//
// You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
//
// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//
// You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
//
// As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.
//
// The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.
//
// For example:
//
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
//
// Consider your entire calibration document. What is the sum of all of the calibration values?

// --- Part Two ---
// Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
//
// Equipped with this new information, you now need to find the real first and last digit on each line. For example:
//
// two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen
// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
//
// What is the sum of all of the calibration values?

use std::iter::Iterator;

fn main() {
    let cal_doc = include_str!("../input/calibration_document.txt");
    println!("Day 1");
    println!("Part 1: {}", cal_values_part1(cal_doc));
    println!("Part 2: {}", cal_values_part2(cal_doc));
}

fn cal_values_part1(cal_doc: &str) -> u32 {
    cal_doc
        .lines()
        .map(|line| {
            let (first, last) = line.chars().fold((0, 0), |cal_values, char| {
                char.to_digit(10)
                    .map_or(cal_values, |digit| match cal_values.0 {
                        0 => (digit, digit),
                        _ => (cal_values.0, digit),
                    })
            });
            (first * 10) + last
        })
        .sum()
}

const SPELLED_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn digit_at_word_end(word: &str) -> Option<u32> {
    for (index, &spelled_digit) in SPELLED_DIGITS.iter().enumerate() {
        if word.ends_with(spelled_digit) {
            return Some((index + 1) as u32);
        }
    }
    None
}

fn cal_values_part2(cal_doc: &str) -> u32 {
    cal_doc
        .lines()
        .map(|line| {
            // fold the lines into cal values
            let (first, last) =
                line.chars()
                    .enumerate()
                    .fold((0, 0), |(curr_first, curr_last), (index, char)| {
                        // try to parse each character into a digit
                        let parsed_digit = char
                            .to_digit(10)
                            // if the character is not a digit...
                            // check if the &str forms a SPELLED_DIGIT
                            .or_else(|| digit_at_word_end(&line[..=index]));

                        parsed_digit.map_or(
                            // if still no digit, keep the tuple as-is
                            (curr_first, curr_last),
                            // if a digit was found, update the tuple accordingly
                            |found_digit| {
                                match curr_first {
                                    0 => (found_digit, found_digit), // first digit found => set both digits
                                    _ => (curr_first, found_digit), // from the second digit on, only update the last one
                                }
                            },
                        )
                    });

            // format the result
            (first * 10) + last
        })
        .sum() // we want the SUM of the cal_vals
}
