use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;
use regex::Regex;

type InputType = HashMap<Monkey, Job>;

type Monkey = String;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Job {
    Yell(i64),
    YellSum(Monkey, Monkey),
    YellMul(Monkey, Monkey),
    YellDiv(Monkey, Monkey),
    YellSub(Monkey, Monkey),
    YellUnknown,
}

#[aoc_generator(day21)]
fn parse_input_day21(input: &str) -> InputType {
    // vgmc: vvwl * snqc
    // tbfj: 5
    // svsf: fgcj + tgzh
    // blzm: tffz * wfct
    // cgbg: 3
    // nzgg: blcv + whnm
    // rpdc: 8
    // hvpf: tdmn + qtqw
    // svtv: 2
    // jjbl: bbjp + djjz
    // bgqn: btjr * lgnw
    // pjgv: 3
    // pcfq: 8
    // pmqd: 7

    // Regex
    input
        .lines()
        .map(|x| {
            let re = Regex::new(r"(\w+): (.*)").unwrap();
            let caps = re.captures(x).unwrap();
            let monkey = caps[1].to_string();
            let job = if caps[2].contains("+") {
                let (a, b) = caps[2]
                    .split(" + ")
                    .map(|x| x.to_string())
                    .collect_tuple()
                    .unwrap();
                Job::YellSum(a, b)
            } else if caps[2].contains("*") {
                let (a, b) = caps[2]
                    .split(" * ")
                    .map(|x| x.to_string())
                    .collect_tuple()
                    .unwrap();
                Job::YellMul(a, b)
            } else if caps[2].contains("/") {
                let (a, b) = caps[2]
                    .split(" / ")
                    .map(|x| x.to_string())
                    .collect_tuple()
                    .unwrap();
                Job::YellDiv(a, b)
            } else if caps[2].contains("-") {
                let (a, b) = caps[2]
                    .split(" - ")
                    .map(|x| x.to_string())
                    .collect_tuple()
                    .unwrap();
                Job::YellSub(a, b)
            } else {
                Job::Yell(caps[2].parse::<i64>().unwrap())
            };

            (monkey, job)
        })
        .collect()
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &InputType) -> i64 {
    // Figure out what the monkey "root" is yelling

    fn calculate_yell(monkey: &Monkey, input: &InputType, memo: &mut HashMap<Monkey, i64>) -> i64 {
        if memo.contains_key(monkey) {
            return *memo.get(monkey).unwrap();
        }

        let job = input.get(monkey).unwrap();
        let yell = match job {
            Job::Yell(x) => *x,
            Job::YellSum(a, b) => calculate_yell(a, input, memo) + calculate_yell(b, input, memo),
            Job::YellMul(a, b) => calculate_yell(a, input, memo) * calculate_yell(b, input, memo),
            Job::YellDiv(a, b) => calculate_yell(a, input, memo) / calculate_yell(b, input, memo),
            Job::YellSub(a, b) => calculate_yell(a, input, memo) - calculate_yell(b, input, memo),
            Job::YellUnknown => panic!("Unknown job"),
        };

        memo.insert(monkey.to_string(), yell);
        yell
    }

    let mut memo = HashMap::new();
    calculate_yell(&"root".to_string(), input, &mut memo)
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &InputType) -> i64 {
    // Figure out what the monkey "root" is yelling

    let mut input = input.clone();

    fn calculate_yell(
        monkey: &Monkey,
        input: &InputType,
        memo: &mut HashMap<Monkey, &String>,
    ) -> String {
        let job = input.get(monkey).unwrap();
        let yell = match job {
            Job::Yell(x) => format!("{}", x),
            Job::YellSum(a, b) => format!(
                "({} + {})",
                calculate_yell(a, input, memo),
                calculate_yell(b, input, memo)
            ),
            Job::YellMul(a, b) => format!(
                "({} * {})",
                calculate_yell(a, input, memo),
                calculate_yell(b, input, memo)
            ),
            Job::YellDiv(a, b) => format!(
                "({} / {})",
                calculate_yell(a, input, memo),
                calculate_yell(b, input, memo)
            ),
            Job::YellSub(a, b) => format!(
                "({} - {})",
                calculate_yell(a, input, memo),
                calculate_yell(b, input, memo)
            ),
            Job::YellUnknown => "x".to_string(),
        };

        yell
    }

    let mut flag = false;
    let mut i = 0;

    input.insert("humn".to_string(), Job::YellUnknown);
    let mut memo = HashMap::new();
    println!();
    println!("{}", calculate_yell(&"lttc".to_string(), &input, &mut memo));
    println!();
    println!("{}", calculate_yell(&"pfjc".to_string(), &input, &mut memo));
    println!();

    unreachable!();
}
