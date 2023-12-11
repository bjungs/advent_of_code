use std::ops::{Add, AddAssign};

// --- Day 1: Not Quite Lisp ---
// Santa was hoping for a white Christmas, but his weather machine's "snow" function is powered by stars, and he's fresh out! To save Christmas, he needs you to collect fifty stars by December 25th.
//
// Collect stars by helping Santa solve puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//
// Here's an easy puzzle to warm you up.
//
// Santa is trying to deliver presents in a large apartment building, but he can't find the right floor - the directions he got are a little confusing. He starts on the ground floor (floor 0) and then follows the instructions one character at a time.
//
// An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means he should go down one floor.
//
// The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors.
//
// For example:
//
// (()) and ()() both result in floor 0.
// ((( and (()(()( both result in floor 3.
// ))((((( also results in floor 3.
// ()) and ))( both result in floor -1 (the first basement level).
// ))) and )())()) both result in floor -3.
// To what floor do the instructions take Santa?
//
// --- Part Two ---
// Now, given the same instructions, find the position of the first character that causes him to enter the basement (floor -1). The first character in the instructions has position 1, the second character has position 2, and so on.
//
// For example:
//
// ) causes him to enter the basement at character position 1.
// ()()) causes him to enter the basement at character position 5.
// What is the position of the character that causes Santa to first enter the basement?

enum Step {
    Up,
    Down,
}

impl From<&Step> for i32 {
    fn from(step: &Step) -> Self {
        match step {
            Step::Up => 1,
            Step::Down => -1,
        }
    }
}

impl Add<&Step> for i32 {
    type Output = i32;

    fn add(self, rhs: &Step) -> Self::Output {
        self + i32::from(rhs)
    }
}

impl AddAssign<&Step> for i32 {
    fn add_assign(&mut self, rhs: &Step) {
        *self += i32::from(rhs)
    }
}

struct Input {
    steps: Vec<Step>
}

fn process_input(str_input: &str) -> Input {
    let steps = str_input.chars().filter_map(|c| {
        match c {
            '(' => Some(Step::Up),
            ')' => Some(Step::Down),
            _ => None,
        }
    }).collect();

    Input { steps }
}

fn main() {
    let input = process_input(include_str!("input"));
    println!("Day 1");
    println!("\tPart 1: {}", final_floor(&input));
    println!("\tPart 2: {}", first_time_basement(&input));
}

fn final_floor(input: &Input) -> i32 {
    input.steps.iter().fold(0, |acc, step| acc + step)
}

fn first_time_basement(input: &Input) -> usize {
    let mut floor = 0;
    for (idx, step) in input.steps.iter().enumerate() {
        floor += step;
        if floor < 0 {
            return idx + 1;
        }
    }
    panic!("Santa never reached the basement!")
}
