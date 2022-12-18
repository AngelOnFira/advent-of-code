use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;
use petgraph::{prelude::UnGraph, Graph};
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

#[derive(Debug, Clone)]
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
    // Create a graph
    let mut node_lookup = HashMap::new();
    let mut graph = Graph::<&str, &str>::new();

    // Add all the nodes
    for (name, (_, leads)) in input.iter() {
        let node = graph.add_node(name);
        node_lookup.insert(name, node);
    }

    // Add all the edges
    for (name, (_, leads)) in input.iter() {
        for lead in leads {
            graph.add_edge(*node_lookup.get(name).unwrap(), *node_lookup.get(lead).unwrap(), "");
        }
    }

    // Turn all the valve names in to numbers in a hashmap
    let mut valve_names = input
        .keys()
        .enumerate()
        .map(|(i, x)| (x.to_string(), i as i32))
        .collect::<HashMap<_, _>>();

    // let nodes = &input
    //     .iter()
    //     .flat_map(|(name, (_, leads))| {
    //         leads
    //             .iter()
    //             .map(|x| {
    //                 (
    //                     *valve_names.get(name).unwrap(),
    //                     *valve_names.get(x).unwrap(),
    //                 )
    //             })
    //             .collect::<Vec<(i32, i32)>>()
    //     })
    //     .collect::<Vec<(i32, i32)>>();

    // dbg!(&nodes);
    // dbg!(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

    // let graph = UnGraph::<i32, ()>::from_edges(
    //     nodes.as_slice(),
    // );

    // let g = UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

    // Print the max flow path
    // println!(
    //     "{:?}",
    //     flow_list
    //         .iter()
    //         .max_by_key(|x| x.get_current_flow(input))
    //         .unwrap()
    // );

    // // Find the max flow
    // flow_list
    //     .iter()
    //     .map(|x| x.get_current_flow(input))
    //     .max()
    //     .unwrap()
    0
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &InputType) -> i32 {
    0
}
