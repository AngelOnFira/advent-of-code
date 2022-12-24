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

        other_heuristic.cmp(&self_heuristic)
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
    let entrance = (1, 0);
    let exit = (62, 26);

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

                let new_x = if new_x < 1 {
                    122
                } else if new_x > 122 {
                    1
                } else {
                    new_x
                };

                let new_y = if new_y < 1 {
                    26
                } else if new_y > 26 {
                    1
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

    // Find a path from the beginning to the exit

    let mut queue = BinaryHeap::new();
    queue.push(Group {
        position: entrance,
        moves: 0,
        distance_to_exit: 0,
    });

    let mut seen = HashSet::new();

    while !queue.is_empty() {
        let current = queue.pop().unwrap();

        if current.position == exit {
            println!("Found exit in {} moves", current.moves);
            break;
        }

        let (x, y) = current.position;

        let new_group_left = Group {
            position: (x - 1, y),
            moves: current.moves + 1,
            distance_to_exit: ((x - 1 - exit.0).abs() + (y - exit.1).abs()) as usize,
        };
        if x > 1
            && !seen.contains(&new_group_left)
            && world_frames[current.moves % world_frames.len()]
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
        if x < 122
            && !seen.contains(&new_group_right)
            && world_frames[current.moves % world_frames.len()]
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
        if y > 1
            && !seen.contains(&new_group_up)
            && world_frames[current.moves % world_frames.len()]
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
        if y < 26
            && !seen.contains(&new_group_down)
            && world_frames[current.moves % world_frames.len()]
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
