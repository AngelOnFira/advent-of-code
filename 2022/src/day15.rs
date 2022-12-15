use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

type InputType = Vec<((i32, i32), (i32, i32))>;

#[aoc_generator(day15)]
fn parse_input_day15(input: &str) -> InputType {
    // Sensor at x=2288642, y=2282562: closest beacon is at x=1581951, y=2271709
    // Sensor at x=2215505, y=2975419: closest beacon is at x=2229474, y=3709584
    // Sensor at x=275497, y=3166843: closest beacon is at x=-626874, y=3143870

    // Regex
    input.lines().map(|x| {
        let re = Regex::new(r"Sensor at x=(?P<x>-?\d+), y=(?P<y>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)").unwrap();
        let caps = re.captures(x).unwrap();
        (
            (caps["x"].parse::<i32>().unwrap(), caps["y"].parse::<i32>().unwrap()),
            (caps["bx"].parse::<i32>().unwrap(), caps["by"].parse::<i32>().unwrap()),
        )
    }).collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &InputType) -> i32 {
    // Get the range of each sensor
    let ranges: Vec<((i32, i32), i32)> = input
        .iter()
        .map(|((x, y), (bx, by))| {
            let dist = (bx - x).abs() + (by - y).abs();
            ((*x, *y), dist)
        })
        .collect();

    let beacon_positions = input
        .iter()
        .map(|((x, y), (bx, by))| (*bx, *by))
        .collect::<HashSet<_>>();

    let y = 2_000_000;

    let range = 100_000_000;

    (-1 * range..=range)
        .into_par_iter()
        .filter(|x| {
            // If we're within range of any sensor, but we're not at the
            // location of a beacon, we're valid
            if beacon_positions.contains(&(*x, y)) {
                return false;
            }

            ranges.iter().any(|((x1, y1), dist)| {
                let dist1 = (x1 - x).abs() + (y1 - y).abs();
                dist1 <= *dist
            })
        })
        .count() as i32
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &InputType) -> i32 {
    // Get the range of each sensor
    let ranges: Vec<((i32, i32), i32)> = input
        .iter()
        .map(|((x, y), (bx, by))| {
            let dist = (bx - x).abs() + (by - y).abs();
            ((*x, *y), dist)
        })
        .collect();

    let beacon_positions = input
        .iter()
        .map(|((x, y), (bx, by))| (*bx, *by))
        .collect::<HashSet<_>>();

    let y = 2_000_000;

    let x_min = 0;
    let y_min = 0;
    let x_max = 4_000_000;
    let y_max = 4_000_000;

    let range = 100_000_000;

    (x_min..=x_max)
        .into_par_iter()
        .map(|x| {
            (y_min..=y_max)
                .into_iter()
                .filter(|y| {
                    // If we're within range of any sensor, but we're not at the
                    // location of a beacon, we're valid
                    if beacon_positions.contains(&(x, *y)) {
                        return false;
                    }

                    if ranges.iter().any(|((x1, y1), dist)| {
                        let dist1 = (x1 - x).abs() + (y1 - y).abs();
                        dist1 <= *dist
                    }) {
                        return false;
                    }
                    // Print the position
                    println!("{} {}", x, y);
                    return true;
                })
                .count() as i32
        })
        .count() as i32
}
