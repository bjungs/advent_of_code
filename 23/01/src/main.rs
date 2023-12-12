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

use std::iter::{Iterator};

fn main() {
    let calibration_document = include_str!("../input/input.txt");
    println!("Day 1");
    println!("Part 1: {}", calibration_values_part1(calibration_document));
    println!("Part 2: {}", calibration_values_part2(calibration_document));
}

fn calibration_values_part1(calibration_document: &str) -> u32 {
    calibration_document.lines().map(|line| {
        let (mut first, mut last) = (0, 0);
        for char in line.chars() {
            if let Some(digit) = char.to_digit(10) {
                match first {
                    0 => {
                        first = digit;
                        last = digit;
                    },
                    _ => last = digit
                }
            }
        }
        first * 10 + last
    }).sum()
}

const SPELLED_DIGITS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn digit_at_word_end(word: &str) -> Option<u32> {
    for (index, &spelled_digit) in SPELLED_DIGITS.iter().enumerate() {
        if word.ends_with(spelled_digit) {
            return Some((index + 1) as u32)
        }
    }
    None
}

fn calibration_values_part2(calibration_document: &str) -> u32 {
    // word buffer to keep track of potential SPELLED_DIGITS
    let mut buffer: String = String::new();

    calibration_document.lines().map(|line| {
        // accumulate lines into (first digit, last digit) tuples
        let (first, last) = line.chars().fold((0, 0), |acc, char| {
            // try to parse character into a digit
            let parsed_digit = char
                .to_digit(10)
                .or_else(|| {
                    // character is not a digit
                    //  push it to the word buffer and check if it forms a SPELLED_DIGIT
                    buffer.push(char);
                    digit_at_word_end(buffer.as_str())
                });

            parsed_digit.map_or(
                // no digit found, keep tuple as-is
                acc,
                // digit found, update the tuple accordingly
                |digit| {
                match acc.0 {
                    0 => (digit, digit), // first digit encountered, set both digits
                    _ => (acc.0, digit), // from the second on, only update the last digit
                }
            })
        });

        // clear buffer for next line iteration
        buffer.clear();

        // format the result as expected ({first}{last})
        (first * 10) + last
    }).sum()
}
