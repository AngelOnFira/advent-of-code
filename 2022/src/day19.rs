use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hasher,
    iter::FromIterator,
};

use itertools::Itertools;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use std::hash::Hash;

type InputType = Vec<Blueprint>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Blueprint {
    pub ore_cost: Price,
    pub clay_cost: Price,
    pub obsidian_cost: Price,
    pub geode_cost: Price,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Price {
    pub ore: i32,
    pub clay: i32,
    pub obsidian: i32,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum RobotKinds {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Resources {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd)]
pub struct State {
    pub current_robots: [i32; 4],
    pub current_resources: [i32; 4],
    pub time_remaining: i32,
}

impl State {
    fn mining(&mut self) {
        self.current_resources[0] += self.current_robots[0];
        self.current_resources[1] += self.current_robots[1];
        self.current_resources[2] += self.current_robots[2];
        self.current_resources[3] += self.current_robots[3];
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.current_resources[3].cmp(&other.current_resources[3])
    }
}

#[aoc_generator(day19)]
fn parse_input_day19(input: &str) -> InputType {
    // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 9 clay. Each geode robot costs 3 ore and 9 obsidian.

    // Regex
    input.lines().map(|x| {
        let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
        let caps = re.captures(x).unwrap();
        let ore_cost = Price {
            ore: caps[2].parse::<i32>().unwrap(),
            clay: 0,
            obsidian: 0,
        };
        let clay_cost = Price {
            ore: caps[3].parse::<i32>().unwrap(),
            clay: 0,
            obsidian: 0,
        };
        let obsidian_cost = Price {
            ore: caps[4].parse::<i32>().unwrap(),
            clay: caps[5].parse::<i32>().unwrap(),
            obsidian: 0,
        };
        let geode_cost = Price {
            ore: caps[6].parse::<i32>().unwrap(),
            clay: 0,
            obsidian: caps[7].parse::<i32>().unwrap(),
        };
        Blueprint {
            ore_cost,
            clay_cost,
            obsidian_cost,
            geode_cost,
        }
    }).collect()
}

fn recursively_find_max_geodes(blueprint: &Blueprint, state: State) -> i32 {
    let mut state_queue = BinaryHeap::new();
    // let mut visited_states = HashSet::new();
    let mut curr_max = 0;

    state_queue.push(state.clone());

    while state_queue.len() > 0 {
        // If the state len % 100_000 == 0, print the length
        // if state_queue.len() % 100_000 == 0 {
        //     println!("State queue length: {}", state_queue.len());
        // }

        // If the state_queue is larger than 1_000_000, cull the last 100_000
        if state_queue.len() > 1_000_000 {
            let mut new_state_queue = BinaryHeap::new();
            for (i, state) in state_queue.into_iter().enumerate() {
                if i < 900_000 {
                    new_state_queue.push(state);
                }
            }
            // println!("Culled 100_000 states");
            state_queue = new_state_queue;
        }

        let mut state = state_queue.pop().unwrap();

        // // If any resources are >40, discard this state
        // if state.current_resources[0] > 40
        //     || state.current_resources[&Resources::Clay] > 40
        //     || state.current_resources[&Resources::Obsidian] > 40
        //     || state.current_resources[&Resources::Geode] > 40
        // {
        //     continue;
        // }

        // Base case
        if state.time_remaining == 0 {
            if state.current_resources[3] > curr_max {
                println!("New max: {}", state.current_resources[3]);
                curr_max = state.current_resources[3];
            }
        }

        let current_resources_post_mining = state.current_resources.clone();

        // Process the mining
        state.mining();

        // Reduce time remaining
        let time_remaining = state.time_remaining - 1;

        // The case where we get an Ore robot, and current ore robots are less than 3
        if current_resources_post_mining[0] >= blueprint.ore_cost.ore
        // Heuristic: We can only get 3 ore robots
        // && state.current_robots[0] < 4
        {
            let mut new_current_robots = state.current_robots.clone();
            let mut new_current_resources = state.current_resources.clone();
            new_current_robots[0] += 1;
            new_current_resources[0] -= blueprint.ore_cost.ore;

            let new_state = State {
                current_robots: new_current_robots,
                current_resources: new_current_resources,
                time_remaining,
            };

            state_queue.push(new_state);
        }

        // The case where we get a Clay robot
        // Keep clay below 5
        if current_resources_post_mining[0] >= blueprint.clay_cost.ore
        // && state.current_robots[1] < 5
        {
            let mut new_current_robots = state.current_robots.clone();
            let mut new_current_resources = state.current_resources.clone();
            new_current_robots[1] += 1;
            new_current_resources[0] -= blueprint.clay_cost.ore;

            let new_state = State {
                current_robots: new_current_robots,
                current_resources: new_current_resources,
                time_remaining,
            };

            state_queue.push(new_state);
        }

        // The case where we get an Obsidian robot
        if current_resources_post_mining[0] >= blueprint.obsidian_cost.ore
            && current_resources_post_mining[1] >= blueprint.obsidian_cost.clay
        {
            let mut new_current_robots = state.current_robots.clone();
            let mut new_current_resources = state.current_resources.clone();
            new_current_robots[2] += 1;
            new_current_resources[0] -= blueprint.obsidian_cost.ore;
            new_current_resources[1] -= blueprint.obsidian_cost.clay;

            let new_state = State {
                current_robots: new_current_robots,
                current_resources: new_current_resources,
                time_remaining,
            };

            state_queue.push(new_state);
        }

        // The case where we get a Geode robot
        if current_resources_post_mining[0] >= blueprint.geode_cost.ore
            && current_resources_post_mining[2] >= blueprint.geode_cost.obsidian
        {
            let mut new_current_robots = state.current_robots.clone();
            let mut new_current_resources = state.current_resources.clone();
            new_current_robots[3] += 1;
            new_current_resources[0] -= blueprint.geode_cost.ore;
            new_current_resources[2] -= blueprint.geode_cost.obsidian;

            let new_state = State {
                current_robots: new_current_robots,
                current_resources: new_current_resources,
                time_remaining,
            };

            state_queue.push(new_state);
        }

        // The case where we don't spend anything
        // if geodes_score.len() != 4 {
        let new_state = State {
            current_robots: state.current_robots.clone(),
            current_resources: state.current_resources.clone(),
            time_remaining,
        };

        state_queue.push(new_state);
        // }
    }

    curr_max
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &InputType) -> i32 {
    let current_robots: HashMap<RobotKinds, i32> = HashMap::from_iter(vec![
        (RobotKinds::Ore, 1),
        (RobotKinds::Clay, 2),
        (RobotKinds::Obsidian, 0),
        (RobotKinds::Geode, 0),
    ]);

    let current_resources: HashMap<Resources, i32> = HashMap::from_iter(vec![
        (Resources::Ore, 0),
        (Resources::Clay, 0),
        (Resources::Obsidian, 0),
        (Resources::Geode, 0),
    ]);

    // Test each Blueprint
    input
        .par_iter()
        .enumerate()
        .map(|(i, blueprint)| {
            // Start by assuming we go for clay right away. Subtract the number
            // of turns required to get a clay robot from the total time
            let time_remaining = 24 - blueprint.clay_cost.ore * 2;
            let time_remaining = 24;

            // Find how many geodes can be made in 24 minutes
            let geodes = recursively_find_max_geodes(
                blueprint,
                State {
                    current_robots: [1, 0, 0, 0],
                    current_resources: [0, 0, 0, 0],
                    time_remaining: time_remaining,
                },
            );

            println!("Blueprint {}: {}", i, geodes);

            geodes * (i as i32 + 1)
        })
        .sum()
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &InputType) -> i32 {
    0
}
