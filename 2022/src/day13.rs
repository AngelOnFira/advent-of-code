use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;
use regex::Regex;

type InputType = Vec<(Vec<Token>, Vec<Token>)>;

#[derive(Debug)]
pub enum Token {
    OpeningBracket,
    ClosingBracket,
    Comma,
    Number(i32),
}

#[aoc_generator(day13)]
fn parse_input_day13(input: &str) -> InputType {
    // Chars
    // input.chars().collect()

    // Map to ints
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut x| {
            // There are 2 lines with information then a break
            let mut line1 = Vec::new();
            let mut token_buffer = Vec::new();
            for char in x.next().unwrap().chars() {
                match char {
                    '[' | ']' | ',' => {
                        if !token_buffer.is_empty() {
                            line1.push(Token::Number(
                                token_buffer.iter().join("").parse::<i32>().unwrap(),
                            ));
                            token_buffer.clear();
                        }
                    }
                    _ => {}
                }

                match char {
                    '[' => line1.push(Token::OpeningBracket),
                    ']' => line1.push(Token::ClosingBracket),
                    ',' => line1.push(Token::Comma),
                    _ => token_buffer.push(char),
                }
            }

            let mut line2 = Vec::new();
            let mut token_buffer = Vec::new();
            for char in x.next().unwrap().chars() {
                match char {
                    '[' | ']' | ',' => {
                        if !token_buffer.is_empty() {
                            line2.push(Token::Number(
                                token_buffer.iter().join("").parse::<i32>().unwrap(),
                            ));
                            token_buffer.clear();
                        }
                    }
                    _ => {}
                }

                match char {
                    '[' => line2.push(Token::OpeningBracket),
                    ']' => line2.push(Token::ClosingBracket),
                    ',' => line2.push(Token::Comma),
                    _ => token_buffer.push(char),
                }
            }

            (line1, line2)
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &InputType) -> i32 {
    dbg!(input);
    0
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &InputType) -> i32 {
    0
}
