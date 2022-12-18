use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;
use regex::Regex;

type InputType = Vec<Direction>;

pub enum Direction {
    Left,
    Right,
}

#[aoc_generator(day17)]
fn parse_input_day17(input: &str) -> InputType {
    // ><<>>>><>><<<<>><<<><<<>>>><<>>

    // Map to ints
    input
        .lines()
        .map(|x| match x {
            "<" => Direction::Left,
            ">" => Direction::Right,
            _ => panic!("Unknown direction"),
        })
        .collect()
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &InputType) -> i32 {
    // ####

    // .#.
    // ###
    // .#.

    // ..#
    // ..#
    // ###

    // #
    // #
    // #
    // #

    // ##
    // ##

    let mut shapes_idx = 0;
    let shapes = vec![
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    let mut instructions_idx = 0;

    let mut world_map: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..2022 {
        let mut in_place = false;

        while !in_place {
            // Get the current spawn point
            // Get the highest y value in the world map
            let highest = world_map.iter().map(|x| x.1).max().unwrap();
            let mut current_point = (highest + 4, 2);

            let current_piece = shapes[shapes_idx % shapes.len()];

            // Follow the next instruction if it's allowed
            let next_instruction = input[instructions_idx % input.len()];

            match next_instruction {
                Direction::Left => {
                    current_point.0 -= 1;
                }
                Direction::Right => {
                    current_point.0 += 1;
                }
            }

            // See if any of the points are outside of the wall
            

        }


    }

    // Return the highest point in the world map
    world_map.iter().map(|x| x.1).max().unwrap()

}

#[aoc(day17, part2)]
pub fn solve_part2(input: &InputType) -> i32 {
    0
}
