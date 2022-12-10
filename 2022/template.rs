use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;
use regex::Regex;

type InputType = Vec<(String, i32)>;

#[aoc_generator(day8)]
fn parse_input_day8(input: &str) -> InputType {
    // Chars
    // input.chars().collect()

    // Map to ints
    // input.lines().map(|x| x.parse::<i32>().unwrap()).collect()

    // Regex
    input.lines().map(|x| {}).collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &InputType) -> i32 {
    0
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &InputType) -> i32 {
    0
}
