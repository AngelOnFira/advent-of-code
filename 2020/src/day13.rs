use itertools::Itertools;
use rayon::prelude::*;

#[aoc_generator(day13)]
fn parse_input_day13(input: &str) -> Vec<(i64, i64)> {
    let lines: Vec<&str> = input.lines().collect();

    let mut out: Vec<(i64, i64)> = Vec::new();
    for (i, value) in lines[1].split(",").enumerate() {
        if value != "x" {
            out.push((i as i64, value.parse::<i64>().unwrap()))
        }
    }
    println!("{:?}", out);
    out
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Vec<(i64, i64)>) -> i64 {
    let skip = input.iter().fold(1, |acc, x| {
        if x.0 % input[0].1 == 0 {
            return acc * x.1;
        }
        acc
    });

    let upper_bound = input.iter().fold(1, |acc, x| acc * x.1);
    println!("skip {}", skip);
    println!("upper bound {}", upper_bound);

    // let mut count = 0;
    // let start: i64 = 0;
    // let start: i64 = 100000000000000;
    let _ = ((100000000000000 / skip)..upper_bound / skip)
        .into_par_iter()
        .filter(|x| {
            let counter = x * skip + input[0].1;
            if input.iter().all(|(i, val)| (counter + i) % val == 0) {
                println!("FOUND {}", counter);
                return true;
            }
            // if counter % (skip * 1_000_000_000) == 0 {
            //     println!("{}", counter);
            // }
            return false;
        })
        .count();
    12
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
