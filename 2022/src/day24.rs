use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;
use regex::Regex;

type InputType = Vec<Blizzard>;

pub enum MapKind {
    Empty,
    Wall,
    Up,
    Down,
    Left,
    Right,
}

pub enum Entity {
    Blizzard(Blizzard),
    Group(Group),
    Wall(Wall),
}

pub struct Wall {
    pub position: (i32, i32),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Blizzard {
    pub position: (i32, i32),
    pub direction: i32,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub struct Group {
    pub position: (i32, i32),
    pub moves: usize,
    pub distance_to_exit: usize,
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // The heuristic is number of moves + distance to exit

        let self_heuristic = self.moves + self.distance_to_exit;
        let other_heuristic = other.moves + other.distance_to_exit;

        // self_heuristic.cmp(&other_heuristic)
        // other_heuristic.cmp(&self_heuristic)
        self.distance_to_exit.cmp(&other.distance_to_exit).reverse()
    }
}

#[aoc_generator(day24)]
fn parse_input_day24(input: &str) -> InputType {
    // #.########################################################################################################################
    // #>^v<.<<^v^<><>>>^<vv>^>><<<^^.^v^<<v^v<v>^^>^^<^><^>.^>v<v<v.v<>vv<^^^vvv<.^v^<v^.^.>.<^<^>v^.v^v<>^vv^<<>>v^<<.<vvv^.<<#

    // Regex
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '>' => Some(Blizzard {
                        position: (x as i32, y as i32),
                        direction: 0,
                    }),
                    'v' => Some(Blizzard {
                        position: (x as i32, y as i32),
                        direction: 1,
                    }),
                    '<' => Some(Blizzard {
                        position: (x as i32, y as i32),
                        direction: 2,
                    }),
                    '^' => Some(Blizzard {
                        position: (x as i32, y as i32),
                        direction: 3,
                    }),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &InputType) -> i128 {
    // let entrance = (1, 0);
    // let exit = (62, 25);

    let entrance = (1, 0);
    let exit = (6, 4);

    // Find the bounds, which is 1 less and 1 greater than any of the blizzards
    // positions
    let min_x = input.iter().map(|b| b.position.0).min().unwrap();
    let max_x = input.iter().map(|b| b.position.0).max().unwrap();
    let min_y = input.iter().map(|b| b.position.1).min().unwrap();
    let max_y = input.iter().map(|b| b.position.1).max().unwrap();

    let mut states = HashSet::new();

    let mut last_frame = input.clone();

    let mut world_frames = Vec::new();
    world_frames.push(last_frame.clone());
    states.insert(last_frame.clone());

    loop {
        // Go through each blizzard, move it in it's direction. If it runs out
        // of bounds (x = 1..=122, y = 1..=26), it wraps around.

        let mut this_frame = last_frame
            .iter()
            .map(|b| {
                let (x, y) = b.position;
                let direction = b.direction;
                let (new_x, new_y) = match direction {
                    0 => (x + 1, y),
                    1 => (x, y + 1),
                    2 => (x - 1, y),
                    3 => (x, y - 1),
                    _ => panic!("Invalid direction"),
                };

                let new_x = if new_x < min_x {
                    max_x
                } else if new_x > max_x {
                    min_x
                } else {
                    new_x
                };

                let new_y = if new_y < min_y {
                    max_y
                } else if new_y > max_y {
                    min_y
                } else {
                    new_y
                };

                Blizzard {
                    position: (new_x, new_y),
                    direction,
                }
            })
            .collect::<Vec<_>>();

        // Check if we've seen this frame before
        if states.contains(&this_frame) {
            // We've seen this frame before, so we're in a loop
            break;
        } else {
            // We haven't seen this frame before, so add it to the states
            states.insert(this_frame.clone());
        }

        // Add this frame to the world frames
        world_frames.push(this_frame.clone());

        // Update the last frame
        last_frame = this_frame.clone();
    }

    // Print the loop size
    println!("Loop size: {}", world_frames.len());

    // // Draw the first 5 maps
    // for (i, frame) in world_frames.iter().enumerate().take(10) {
    //     println!("Frame {}", i);
    //     for y in min_y..=max_y {
    //         for x in min_x..=max_x {
    //             if frame.iter().any(|b| b.position == (x, y)) {
    //                 match frame.iter().find(|b| b.position == (x, y)).unwrap().direction {
    //                     0 => print!(">"),
    //                     1 => print!("v"),
    //                     2 => print!("<"),
    //                     3 => print!("^"),
    //                     _ => panic!("Invalid direction"),
    //                 }
    //             } else {
    //                 print!(".");
    //             }
    //         }
    //         println!();
    //     }
    // }

    // Find a path from the beginning to the exit

    let mut queue = BinaryHeap::new();
    queue.push(Group {
        position: entrance,
        moves: 0,
        distance_to_exit: 1000,
    });

    let mut seen = HashSet::new();

    let mut min_distance = 1000;

    while !queue.is_empty() {
        let current = queue.pop().unwrap();

        if current.distance_to_exit < min_distance {
            min_distance = current.distance_to_exit;
            dbg!(&current);
        }

        // // Discard the state if the moves is larger than 20
        // if current.moves > 324 {
        //     continue;
        // }

        if current.position == exit {
            println!("Found exit in {} moves", current.moves + 1);
            break;
        }

        let (x, y) = current.position;

        // If nothing is going to move here, choose to wait
        if !world_frames[(current.moves + 1) % world_frames.len()]
            .iter()
            .any(|b| b.position == (x, y))
        {
            let new_group = Group {
                position: (x, y),
                moves: current.moves + 1,
                distance_to_exit: ((x - exit.0).abs() + (y - exit.1).abs()) as usize,
            };
            if !seen.contains(&new_group) {
                seen.insert(new_group.clone());
                queue.push(new_group);
            }
        }

        let new_group_left = Group {
            position: (x - 1, y),
            moves: current.moves + 1,
            distance_to_exit: ((x - 1 - exit.0).abs() + (y - exit.1).abs()) as usize,
        };
        if x > min_x
            && !seen.contains(&new_group_left)
            && !world_frames[(current.moves + 1) % world_frames.len()]
                .iter()
                .any(|b| b.position == (x - 1, y))
        {
            seen.insert(new_group_left.clone());
            queue.push(new_group_left);
        }

        let new_group_right = Group {
            position: (x + 1, y),
            moves: current.moves + 1,
            distance_to_exit: ((x + 1 - exit.0).abs() + (y - exit.1).abs()) as usize,
        };
        if x < max_x
            && !seen.contains(&new_group_right)
            && !world_frames[(current.moves + 1) % world_frames.len()]
                .iter()
                .any(|b| b.position == (x + 1, y))
        {
            seen.insert(new_group_right.clone());
            queue.push(new_group_right);
        }

        let new_group_up = Group {
            position: (x, y - 1),
            moves: current.moves + 1,
            distance_to_exit: ((x - exit.0).abs() + (y - 1 - exit.1).abs()) as usize,
        };
        if y > min_y
            && !seen.contains(&new_group_up)
            && !world_frames[(current.moves + 1) % world_frames.len()]
                .iter()
                .any(|b| b.position == (x, y - 1))
        {
            seen.insert(new_group_up.clone());
            queue.push(new_group_up);
        }

        let new_group_down = Group {
            position: (x, y + 1),
            moves: current.moves + 1,
            distance_to_exit: ((x - exit.0).abs() + (y + 1 - exit.1).abs()) as usize,
        };
        if y < max_y
            && !seen.contains(&new_group_down)
            && !world_frames[(current.moves + 1) % world_frames.len()]
                .iter()
                .any(|b| b.position == (x, y + 1))
        {
            seen.insert(new_group_down.clone());
            queue.push(new_group_down);
        }
    }

    0
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &InputType) -> i128 {
    0
}
