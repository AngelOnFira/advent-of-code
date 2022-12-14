use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use chumsky::prelude::*;
use eval::{eval, to_value, Value};
use itertools::Itertools;
use regex::Regex;

type InputType = Vec<(Value, Value)>;

#[derive(Debug)]
pub enum Token {
    OpeningBracket,
    ClosingBracket,
    Comma,
    Number(i32),
}

#[derive(Debug)]
enum Expr {
    Num(f64),
    Var(String),

    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),

    Call(String, Vec<Expr>),
    Let {
        name: String,
        rhs: Box<Expr>,
        then: Box<Expr>,
    },
    Fn {
        name: String,
        args: Vec<String>,
        body: Box<Expr>,
        then: Box<Expr>,
    },
}

fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    let int = text::int(10)
        .map(|s: String| Expr::Num(s.parse().unwrap()))
        .padded();

    int.then_ignore(end())
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
            let line_1 = eval(x.next().unwrap()).unwrap();
            let line_2 = eval(x.next().unwrap()).unwrap();
            (line_1, line_2)
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
