use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;
use regex::Regex;

type InputType = HashMap<(i32, i32), bool>;

#[aoc_generator(day23)]
fn parse_input_day23(input: &str) -> InputType {
    // Chars
    // input.chars().collect()

    // Map to ints
    // input.lines().map(|x| x.parse::<i128>().unwrap()).collect()

    // Regex
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((x as i32, y as i32), c == '#'))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn get_map_bounds(map: &InputType) -> ((i32, i32), (i32, i32)) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    map.iter().filter(|(_, v)| **v).for_each(|((x, y), _)| {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    });

    ((min_x, min_y), (max_x, max_y))
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &InputType) -> i128 {
    // Do a game of life simulation
    let mut map = input.clone();

    let map_bounds = get_map_bounds(&map);

    // Print out the map
    for y in map_bounds.0 .1..=map_bounds.1 .1 {
        for x in map_bounds.0 .0..=map_bounds.1 .0 {
            if *map.get(&(x, y)).unwrap_or(&false) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();

    for _ in 0..10 {
        // There are 2 step

        // Step 1
        // If the 8 adjacent cells are empty, do nothing
        // Otherwise:
        // - If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
        // - If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
        // - If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
        // - If there is no Elf in the E, NE, or SE adjacent positions, the Elf
        //   proposes moving east one step.

        // Step 2
        // After each Elf has had a chance to propose a move, the second half of
        // the round can begin. Simultaneously, each Elf moves to their proposed
        // destination tile if they were the only Elf to propose moving to that
        // position. If two or more Elves propose moving to the same position,
        // none of those Elves move.

        // Finally, at the end of the round, the first direction the Elves
        // considered is moved to the end of the list of directions. For
        // example, during the second round, the Elves would try proposing a
        // move to the south first, then west, then east, then north. On the
        // third round, the Elves would first consider west, then east, then
        // north, then south.

        let mut new_map = HashMap::new();

        let mut proposed_moves = HashMap::new();

        for ((x, y), elf) in map.iter() {
            if !elf {
                continue;
            }

            let x = *x;
            let y = *y;

            let mut proposed_move = None;

            // Check if there is an elf in the 8 adjacent cells
            let mut elf_in_adjacent = false;

            for (dx, dy) in vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                if *map.get(&(x + dx, y + dy)).unwrap_or(&false) {
                    elf_in_adjacent = true;
                    break;
                }
            }

            if !elf_in_adjacent {
                new_map.insert((x, y), true);
                proposed_moves.insert((x, y), (x, y));
                continue;
            }

            // Check if there is an elf in the N, NE, or NW adjacent positions
            if !map.get(&(x - 1, y - 1)).unwrap_or(&false)
                && !map.get(&(x, y - 1)).unwrap_or(&false)
                && !map.get(&(x + 1, y - 1)).unwrap_or(&false)
            {
                proposed_move = Some((x, y - 1));
            }
            // Check if there is an elf in the S, SE, or SW adjacent positions
            else if !map.get(&(x - 1, y + 1)).unwrap_or(&false)
                && !map.get(&(x, y + 1)).unwrap_or(&false)
                && !map.get(&(x + 1, y + 1)).unwrap_or(&false)
            {
                proposed_move = Some((x, y + 1));
            }
            // Check if there is an elf in the W, NW, or SW adjacent positions
            else if !map.get(&(x - 1, y - 1)).unwrap_or(&false)
                && !map.get(&(x - 1, y)).unwrap_or(&false)
                && !map.get(&(x - 1, y + 1)).unwrap_or(&false)
            {
                proposed_move = Some((x - 1, y));
            }
            // Check if there is an elf in the E, NE, or SE adjacent positions
            else if !map.get(&(x + 1, y - 1)).unwrap_or(&false)
                && !map.get(&(x + 1, y)).unwrap_or(&false)
                && !map.get(&(x + 1, y + 1)).unwrap_or(&false)
            {
                proposed_move = Some((x + 1, y));
            }

            proposed_moves.insert((x, y), proposed_move.unwrap());

            // println!("Elf at ({}, {}) proposed move to {:?}", x, y,
            // proposed_move);
        }

        // println!("Proposed moves: {:?}", proposed_moves);

        for ((x, y), proposed_move) in proposed_moves.iter() {
            let x = *x;
            let y = *y;

            let proposed_move = *proposed_move;

            // Check if any other elves proposed moving to this position
            let mut other_elf_proposed = false;

            for ((x2, y2), proposed_move2) in proposed_moves.iter() {
                let x2 = *x2;
                let y2 = *y2;

                let proposed_move2 = *proposed_move2;

                if x == x2 && y == y2 {
                    continue;
                }

                if proposed_move == proposed_move2 {
                    other_elf_proposed = true;
                    break;
                }
            }

            if other_elf_proposed {
                new_map.insert((x, y), true);

                continue;
            }

            // Move the elf
            new_map.insert(proposed_move, true);
        }

        let map_bounds = get_map_bounds(&map);

        // Print out the map
        for y in map_bounds.0 .1..=map_bounds.1 .1 {
            for x in map_bounds.0 .0..=map_bounds.1 .0 {
                if *map.get(&(x, y)).unwrap_or(&false) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        println!("");

        map = new_map;
    }

    // Find the area of the rectangle that contains all elves
    let map_bounds = get_map_bounds(&map);

    ((map_bounds.1 .0 - map_bounds.0 .0 + 1) * (map_bounds.1 .1 - map_bounds.0 .1 + 1)) as i128
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &InputType) -> i128 {
    0
}
