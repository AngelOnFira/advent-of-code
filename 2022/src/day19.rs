use std::{
    collections::{HashMap, HashSet},
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    pub current_robots: HashMap<RobotKinds, i32>,
    pub current_resources: HashMap<Resources, i32>,
    pub time_remaining: i32,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the hashmap by each key
        self.current_robots.iter().for_each(|(k, v)| {
            k.hash(state);
            v.hash(state);
        });
        self.current_resources.iter().for_each(|(k, v)| {
            k.hash(state);
            v.hash(state);
        });
        self.time_remaining.hash(state);
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

fn recursively_find_max_geodes(blueprint: &Blueprint, mut state: State) -> i32 {
    // Base case
    if state.time_remaining == 0 {
        return state.current_resources[&Resources::Geode];
    }

    let mut current_resources_post_mining = state.current_resources.clone();

    // Process the mining
    current_resources_post_mining
        .entry(Resources::Ore)
        .and_modify(|x| *x += state.current_robots[&RobotKinds::Ore]);
    current_resources_post_mining
        .entry(Resources::Clay)
        .and_modify(|x| *x += state.current_robots[&RobotKinds::Clay]);
    current_resources_post_mining
        .entry(Resources::Obsidian)
        .and_modify(|x| *x += state.current_robots[&RobotKinds::Obsidian]);
    current_resources_post_mining
        .entry(Resources::Geode)
        .and_modify(|x| *x += state.current_robots[&RobotKinds::Geode]);

    // Reduce time remaining
    let time_remaining = state.time_remaining - 1;

    // Recursive case
    let mut geodes_score = Vec::new();

    // The case where we get an Ore robot, and current ore robots are less than 3
    if state.current_resources[&Resources::Ore] >= blueprint.ore_cost.ore
        // Heuristic: We can only get 3 ore robots
        && state.current_robots[&RobotKinds::Ore] < 4
    {
        let mut new_current_robots = state.current_robots.clone();
        let mut new_current_resources = current_resources_post_mining.clone();
        new_current_robots
            .entry(RobotKinds::Ore)
            .and_modify(|x| *x += 1);
        new_current_resources
            .entry(Resources::Ore)
            .and_modify(|x| *x -= blueprint.ore_cost.ore);
        geodes_score.push(recursively_find_max_geodes(
            blueprint,
            State {
                current_robots: new_current_robots,
                current_resources: new_current_resources,
                time_remaining,
            },
        ));
    }

    // The case where we get a Clay robot
    // Keep clay below 5
    if state.current_resources[&Resources::Ore] >= blueprint.clay_cost.ore
        && state.current_robots[&RobotKinds::Clay] < 8
    {
        let mut new_current_robots = state.current_robots.clone();
        let mut new_current_resources = current_resources_post_mining.clone();
        new_current_robots
            .entry(RobotKinds::Clay)
            .and_modify(|x| *x += 1);
        new_current_resources
            .entry(Resources::Ore)
            .and_modify(|x| *x -= blueprint.clay_cost.ore);
        geodes_score.push(recursively_find_max_geodes(
            blueprint,
            State {
                current_robots: new_current_robots,
                current_resources: new_current_resources,
                time_remaining,
            },
        ));
    }

    // The case where we get an Obsidian robot
    if state.current_resources[&Resources::Ore] >= blueprint.obsidian_cost.ore
        && state.current_resources[&Resources::Clay] >= blueprint.obsidian_cost.clay
    {
        let mut new_current_robots = state.current_robots.clone();
        let mut new_current_resources = current_resources_post_mining.clone();
        new_current_robots
            .entry(RobotKinds::Obsidian)
            .and_modify(|x| *x += 1);
        new_current_resources
            .entry(Resources::Ore)
            .and_modify(|x| *x -= blueprint.obsidian_cost.ore);
        new_current_resources
            .entry(Resources::Clay)
            .and_modify(|x| *x -= blueprint.obsidian_cost.clay);
        geodes_score.push(recursively_find_max_geodes(
            blueprint,
            State {
                current_robots: new_current_robots,
                current_resources: new_current_resources,
                time_remaining,
            },
        ));
    }

    // The case where we get a Geode robot
    if state.current_resources[&Resources::Ore] >= blueprint.geode_cost.ore
        && state.current_resources[&Resources::Obsidian] >= blueprint.geode_cost.obsidian
    {
        let mut new_current_robots = state.current_robots.clone();
        let mut new_current_resources = current_resources_post_mining.clone();
        new_current_robots
            .entry(RobotKinds::Geode)
            .and_modify(|x| *x += 1);
        new_current_resources
            .entry(Resources::Ore)
            .and_modify(|x| *x -= blueprint.geode_cost.ore);
        new_current_resources
            .entry(Resources::Obsidian)
            .and_modify(|x| *x -= blueprint.geode_cost.obsidian);
        geodes_score.push(recursively_find_max_geodes(
            blueprint,
            State {
                current_robots: new_current_robots,
                current_resources: new_current_resources,
                time_remaining,
            },
        ));
    }

    // The case where we don't spend anything
    if geodes_score.len() != 4 {
        geodes_score.push(recursively_find_max_geodes(
            blueprint,
            State {
                current_robots: state.current_robots.clone(),
                current_resources: current_resources_post_mining.clone(),
                time_remaining,
            },
        ));
    }

    // Return the max
    *geodes_score.iter().max().unwrap()
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
            
            // Find how many geodes can be made in 24 minutes
            let geodes = recursively_find_max_geodes(
                blueprint,
                State {
                    current_robots: current_robots.clone(),
                    current_resources: current_resources.clone(),
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
