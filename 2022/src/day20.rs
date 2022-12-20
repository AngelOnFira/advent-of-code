use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::FromIterator,
};

use itertools::Itertools;
use regex::Regex;

type InputType = Vec<i64>;

#[aoc_generator(day20)]
fn parse_input_day20(input: &str) -> InputType {
    input.lines().map(|x| x.parse::<i64>().unwrap()).collect()
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &InputType) -> i64 {
    let mut change_list: VecDeque<i64> = (0..input.len()).into_iter().map(|x| x as i64).collect();
    let second_input = input.clone();
    let input: Vec<i64> = input
        .iter()
        .map(|x| {
            return *x;
            return *x * 811589153;
            let a = *x * 811589153;
            let mut flag = false;
            if a < 0 {
                flag = true;
            }
            let b = a.abs() % change_list.len() as i64;
            if flag {
                -b
            } else {
                b
            }
        })
        .collect();

    // for _ in 0..10 {
        for i in 0..input.len() {
            // debug the list
            for j in 0..change_list.len() {
                print!("{} ", input[change_list[j] as usize]);
            }
            println!();
            // Find i in the change_list, and shift it back or forward, moving other
            // elements, by the value of input[i]. Make sure to handle wrapping
            // around the end of the list.
            let index = change_list.iter().position(|&x| x == i as i64).unwrap();
            let mut new_index = index as i64 + input[i];
            if new_index < 0 {
                // Get how many times we need to wrap around
                let times = new_index.abs() / (change_list.len() as i64);
                new_index += times * change_list.len() as i64;
                new_index -= times;
            }
            while new_index <= 0 {
                new_index += change_list.len() as i64;
                new_index -= 1;
            }
            if new_index >= change_list.len() as i64 {
                let times = new_index / (change_list.len() as i64);
                new_index -= times * change_list.len() as i64;
                new_index += times;
            }
            while new_index >= change_list.len() as i64 {
                new_index -= change_list.len() as i64;
                new_index += 1;
            }

            change_list.remove(index);
            change_list.insert(new_index as usize, i as i64);

            // If we inserted right before the end, move the last item to the
            // front
            if new_index == change_list.len() as i64 - 2 {
                let last = change_list.pop_back().unwrap();
                change_list.push_front(last);
            }

            // If we inserted right after the beginning, move the first item to
            // the end
            if new_index == 1 {
                let first = change_list.pop_front().unwrap();
                change_list.push_back(first);
            }
        }
        for j in 0..change_list.len() {
            print!("{} ", input[change_list[j] as usize]);
        }
        println!();

        println!("Done an iteration");
        // break;
    // }

    // Return the sum of the 1000th, 2000th, and 3000th numbers of the list,
    // wrapping around
    let mut sum = 0;

    // Find where 0 is in the list
    let zero_index = input.iter().position(|&x| x == 0).unwrap();
    let zero_curr_pos = change_list
        .iter()
        .position(|&x| x == zero_index as i64)
        .unwrap();

    dbg!(zero_curr_pos);
    dbg!(zero_index);

    for i in 1..=3 {
        let index = change_list[(zero_curr_pos + i * 1000) % change_list.len()];
        dbg!(input[index as usize]);
        sum += input[index as usize];
    }
    sum
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &InputType) -> i64 {
    0
}
