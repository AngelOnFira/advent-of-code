use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use regex::Regex;

type InputType = Vec<Elf>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Elf {
    pub x: i32,
    pub y: i32,
    pub direction: i32,
    pub proposed_move: Option<(i32, i32)>,
}

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
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some(Elf {
                            x: x as i32,
                            y: y as i32,
                            direction: 0,
                            proposed_move: None,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn get_map_bounds(map: &InputType) -> ((i32, i32), (i32, i32)) {
    let mut min_x = None;
    let mut max_x = None;
    let mut min_y = None;
    let mut max_y = None;

    map.iter().for_each(|elf| {
        if min_x.is_none() || elf.x < min_x.unwrap() {
            min_x = Some(elf.x);
        }
        if max_x.is_none() || elf.x > max_x.unwrap() {
            max_x = Some(elf.x);
        }
        if min_y.is_none() || elf.y < min_y.unwrap() {
            min_y = Some(elf.y);
        }
        if max_y.is_none() || elf.y > max_y.unwrap() {
            max_y = Some(elf.y);
        }
    });

    (
        (min_x.unwrap(), min_y.unwrap()),
        (max_x.unwrap(), max_y.unwrap()),
    )
}

fn is_elf_at(map: &InputType, x: i32, y: i32) -> bool {
    map.iter().any(|elf| elf.x == x && elf.y == y)
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &InputType) -> i128 {
    // Do a game of life simulation
    let mut map = input.clone();

    let mut global_dir = 0;

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

        let mut new_map = Vec::new();

        for elf in map.iter() {
            let x = elf.x;
            let y = elf.y;

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
                if is_elf_at(&map, x + dx, y + dy) {
                    elf_in_adjacent = true;
                    break;
                }
            }

            if !elf_in_adjacent {
                new_map.push(Elf {
                    x,
                    y,
                    direction: elf.direction,
                    proposed_move: None,
                });
                continue;
            }

            let direction_list = [
                ([(-1, -1), (0, -1), (1, -1)], (x, y - 1)),
                ([(-1, 1), (0, 1), (1, 1)], (x, y + 1)),
                ([(-1, -1), (-1, 0), (-1, 1)], (x - 1, y)),
                ([(1, -1), (1, 1), (1, 0)], (x + 1, y)),
            ];

            let mut flag = false;
            for dir_idx in 0..direction_list.len() {
                let (direction, proposed_move) =
                    direction_list[(dir_idx + global_dir) % direction_list.len()];

                if !direction
                    .iter()
                    .any(|(dx, dy)| is_elf_at(&map, x + dx, y + dy))
                {
                    new_map.push(Elf {
                        x,
                        y,
                        direction: elf.direction,
                        proposed_move: Some(proposed_move),
                    });
                    flag = true;
                    break;
                }
            }

            if !flag {
                new_map.push(Elf {
                    x,
                    y,
                    direction: elf.direction,
                    proposed_move: None,
                });
            }
        }

        for i in 0..new_map.len() {
            for j in 0..new_map.len() {
                if i == j {
                    continue;
                }

                if new_map[i].proposed_move == new_map[j].proposed_move {
                    new_map[i].proposed_move = None;
                    new_map[j].proposed_move = None;
                }
            }
        }

        for elf in new_map.iter_mut() {
            if let Some((x, y)) = elf.proposed_move {
                elf.x = x;
                elf.y = y;
            }
            elf.proposed_move = None;
        }

        map = new_map;
        global_dir += 1;
    }

    // Find the area of the rectangle that contains all elves
    let map_bounds = get_map_bounds(&map);

    ((map_bounds.1 .0 - map_bounds.0 .0 + 1) * (map_bounds.1 .1 - map_bounds.0 .1 + 1)
        - map.len() as i32) as i128
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &InputType) -> i128 {
    // Do a game of life simulation
    let mut map = input.clone();

    let mut global_dir = 0;

    let mut total_rounds = 1;
    
    loop {
        let mut new_map = map
            .par_iter()
            .map(|elf| {
                let x = elf.x;
                let y = elf.y;

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
                    if is_elf_at(&map, x + dx, y + dy) {
                        elf_in_adjacent = true;
                        break;
                    }
                }

                if !elf_in_adjacent {
                    return Elf {
                        x,
                        y,
                        direction: elf.direction,
                        proposed_move: None,
                    };
                }

                let direction_list = [
                    ([(-1, -1), (0, -1), (1, -1)], (x, y - 1)),
                    ([(-1, 1), (0, 1), (1, 1)], (x, y + 1)),
                    ([(-1, -1), (-1, 0), (-1, 1)], (x - 1, y)),
                    ([(1, -1), (1, 1), (1, 0)], (x + 1, y)),
                ];

                for dir_idx in 0..direction_list.len() {
                    let (direction, proposed_move) =
                        direction_list[(dir_idx + global_dir) % direction_list.len()];

                    if !direction
                        .iter()
                        .any(|(dx, dy)| is_elf_at(&map, x + dx, y + dy))
                    {
                        return Elf {
                            x,
                            y,
                            direction: elf.direction,
                            proposed_move: Some(proposed_move),
                        };
                    }
                }

                return Elf {
                    x,
                    y,
                    direction: elf.direction,
                    proposed_move: None,
                };
            })
            .collect::<Vec<_>>();

        let proposed_move_map = new_map.iter().fold(HashMap::new(), |mut acc, elf| {
            if let Some(proposed_move) = elf.proposed_move {
                acc.entry(proposed_move).or_insert(0);
                *acc.get_mut(&proposed_move).unwrap() += 1;
            }
            acc
        });

        // Check if no elf has moved
        if new_map.iter().all(|elf| elf.proposed_move.is_none()) {
            return total_rounds;
        }

        new_map.par_iter_mut().for_each(|elf| {
            if elf.proposed_move.is_some()
                && *proposed_move_map.get(&elf.proposed_move.unwrap()).unwrap() == 1
            {
                elf.x = elf.proposed_move.unwrap().0;
                elf.y = elf.proposed_move.unwrap().1;
            }
            elf.proposed_move = None;
        });

        map = new_map;
        global_dir += 1;
        total_rounds += 1;
    }
}
