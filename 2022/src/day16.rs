use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;
use regex::Regex;

type InputType = HashMap<String, (i32, Vec<String>)>;

#[aoc_generator(day16)]
fn parse_input_day16(input: &str) -> InputType {
    // Valve QE has flow rate=3; tunnels lead to valves OU, ME, UX, AX, TW
    // Valve TN has flow rate=16; tunnels lead to valves UW, CG, WB
    // Valve UX has flow rate=0; tunnels lead to valves AA, QE
    // Valve QN has flow rate=25; tunnel leads to valve SD

    // Regex
    input.lines().map(|x| {
        let re = Regex::new(r"Valve (?P<name>\w+) has flow rate=(?P<flow>\d+); tunnels lead to valves (?P<leads>.*)").unwrap();
        let caps = re.captures(x).unwrap();
        (
            caps["name"].to_string(),
            (
                caps["flow"].parse::<i32>().unwrap(),
                caps["leads"].split(", ").map(|x| x.to_string()).collect(),
            ),
        )
    }).collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Path {
    opened_valves: HashSet<String>,
    current_valve: String,
    total_flow: i32,
    time_remaining: i32,
}

impl Path {
    fn get_current_flow(&self, input: &InputType) -> i32 {
        self.total_flow
            + self
                .opened_valves
                .iter()
                .map(|x| input.get(x).unwrap().0)
                .sum::<i32>()
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &InputType) -> i32 {
    let mut current_paths: HashSet<Path> = HashSet::new();

    // Start with the first valve
    current_paths.insert(Path {
        opened_valves: HashSet::new(),
        current_valve: "AA".to_string(),
        total_flow: 0,
        time_remaining: 20,
    });

    let mut flow_list = Vec::new();

    while current_paths.len() > 0 {
        let mut new_paths = Vec::new();

        for path in current_paths.iter() {
            // If we're at the end of our time, add the flow of this path to the
            // flow list
            if path.time_remaining == 1 {
                flow_list.push(path.clone());
                continue;
            }

            // Otherwise, start calcualting flow
            let current_valve = path.current_valve.clone();
            let (flow, leads) = input.get(&current_valve).unwrap();

            // Split in two. Either continue moving, or open this valve.
            let current_flow = path.get_current_flow(input);

            // Open this valve if it's not already open and the flow here isn't 0
            if !path.opened_valves.contains(&current_valve) && *flow != 0 {
                let mut new_opened_valves = path.opened_valves.clone();
                new_opened_valves.insert(current_valve.clone());
                new_paths.push(Path {
                    opened_valves: new_opened_valves,
                    current_valve: current_valve.clone(),
                    total_flow: path.total_flow + current_flow,
                    time_remaining: path.time_remaining - 1,
                });
            }

            // Add a new path for each lead
            for lead in leads {
                new_paths.push(Path {
                    opened_valves: path.opened_valves.clone(),
                    current_valve: lead.clone(),
                    total_flow: path.total_flow + current_flow,
                    time_remaining: path.time_remaining - 1,
                });
            }
        }

        current_paths = new_paths;
    }

    // Print the max flow path
    println!(
        "{:?}",
        flow_list
            .iter()
            .max_by_key(|x| x.get_current_flow(input))
            .unwrap()
    );

    // Find the max flow
    flow_list
        .iter()
        .map(|x| x.get_current_flow(input))
        .max()
        .unwrap()
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &InputType) -> i32 {
    0
}
