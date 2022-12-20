use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::FromIterator,
};

use itertools::Itertools;
use regex::Regex;

type InputType = Vec<i32>;

#[aoc_generator(day20)]
fn parse_input_day20(input: &str) -> InputType {
    input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &InputType) -> i32 {
    let mut change_list: VecDeque<i32> = (0..input.len()).into_iter().map(|x| x as i32).collect();

    for i in 0..input.len() {
        // debug the list
        // for j in 0..change_list.len() {
        //     print!("{} ", input[change_list[j] as usize]);
        // }
        // println!();
        // Find i in the change_list, and shift it back or forward, moving other
        // elements, by the value of input[i]. Make sure to handle wrapping
        // around the end of the list.
        let index = change_list.iter().position(|&x| x == i as i32).unwrap();
        let mut new_index = index as i32 + input[i];
        let mut flag = false;
        while new_index <= 0 {
            flag = true;
            new_index += change_list.len() as i32;
            new_index -= 1;
        }
        if flag {
        }
        if new_index >= change_list.len() as i32 {
            new_index %= change_list.len() as i32;
            new_index += 1;
        }

        change_list.remove(index);
        change_list.insert(new_index as usize, i as i32);
    }

    for j in 0..change_list.len() {
        print!("{} ", input[change_list[j] as usize]);
    }
    println!();

    // Return the sum of the 1000th, 2000th, and 3000th numbers of the list,
    // wrapping around
    let mut sum = 0;

    // Find where 0 is in the list
    let zero_index = input.iter().position(|&x| x == 0).unwrap();
    let zero_curr_pos = change_list.iter().position(|&x| x == zero_index as i32).unwrap();

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
pub fn solve_part2(input: &InputType) -> i32 {
    0
}
