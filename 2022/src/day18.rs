use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

type InputType = Vec<(i32, i32, i32)>;

#[aoc_generator(day18)]
fn parse_input_day18(input: &str) -> InputType {
    // 8,16,18
    // 12,15,19
    // 14,19,12
    // 4,6,8
    // 16,8,16

    input
        .lines()
        .map(|x| {
            let mut iter = x.split(",").map(|x| x.parse::<i32>().unwrap());
            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &InputType) -> i32 {
    // In 3d, count all the sides of the cubes that don't touch another cube

    let mut surface_area = 0;
    for cube in input {
        let directions = vec![
            (0, 1, 0),
            (0, -1, 0),
            (1, 0, 0),
            (-1, 0, 0),
            (0, 0, 1),
            (0, 0, -1),
        ];
        for side in 0..6 {
            let mut found = false;
            for other_cube in input {
                if other_cube == cube {
                    continue;
                }
                if other_cube.0 == cube.0 + directions[side].0
                    && other_cube.1 == cube.1 + directions[side].1
                    && other_cube.2 == cube.2 + directions[side].2
                {
                    found = true;
                    break;
                }
            }
            if !found {
                surface_area += 1;
            }
        }
    }
    surface_area
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &InputType) -> i32 {
    // Set up a map of all the cubes
    let shape: HashSet<(i32, i32, i32)> = HashSet::from_iter(input.iter().cloned());

    let outside = (
        shape.iter().map(|x| x.0).min().unwrap() - 1,
        shape.iter().map(|x| x.1).min().unwrap() - 1,
        shape.iter().map(|x| x.2).min().unwrap() - 1,
    );

    // Make a 3d grid around all of the cubes
    (shape.iter().map(|x| x.0).min().unwrap() - 1..shape.iter().map(|x| x.0).max().unwrap() + 2)
        .into_par_iter()
        .map(|x| {
            let mut surface_area = 0;
            for y in shape.iter().map(|x| x.1).min().unwrap() - 1
                ..shape.iter().map(|x| x.1).max().unwrap() + 2
            {
                for z in shape.iter().map(|x| x.2).min().unwrap() - 1
                    ..shape.iter().map(|x| x.2).max().unwrap() + 2
                {
                    // Make sure there is a path from here to the outside
                    if !find_path(&shape, (x, y, z), outside) {
                        continue;
                    }

                    // Add a surface area for each neighbor that is there
                    let directions = vec![
                        (0, 1, 0),
                        (0, -1, 0),
                        (1, 0, 0),
                        (-1, 0, 0),
                        (0, 0, 1),
                        (0, 0, -1),
                    ];
                    for side in 0..6 {
                        let new_pos = (
                            x + directions[side].0,
                            y + directions[side].1,
                            z + directions[side].2,
                        );
                        if shape.contains(&new_pos) {
                            surface_area += 1;
                        }
                    }
                }
            }
            surface_area
        })
        .sum()
}

fn find_path(
    shape: &HashSet<(i32, i32, i32)>,
    mut pos: (i32, i32, i32),
    target: (i32, i32, i32),
) -> bool {
    let mut visited = HashSet::new();
    let mut queue = vec![pos];
    while !queue.is_empty() {
        let mut new_queue = vec![];
        for pos in queue {
            if pos == target {
                return true;
            }
            visited.insert(pos);
            let directions = vec![
                (0, 1, 0),
                (0, -1, 0),
                (1, 0, 0),
                (-1, 0, 0),
                (0, 0, 1),
                (0, 0, -1),
            ];
            for side in 0..6 {
                let new_pos = (
                    pos.0 + directions[side].0,
                    pos.1 + directions[side].1,
                    pos.2 + directions[side].2,
                );
                if !visited.contains(&new_pos) && shape.contains(&new_pos) {
                    new_queue.push(new_pos);
                }
            }
        }
        queue = new_queue;
    }
    false
}
