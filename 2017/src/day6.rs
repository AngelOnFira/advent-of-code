use std::collections::HashMap;
use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut memory: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mem_size: usize = memory.len();

    let mut seen: HashSet<Vec<i32>> = HashSet::new();
    let mut counter = 0;
    loop {
        if !seen.insert(memory.clone()) {
            return counter;
        }
        let max = memory.iter().max().unwrap().clone();
        let pos = memory.iter().position(|&e| e == max).unwrap();
        memory[pos] = 0;

        for i in 1..=max {
            memory[(pos + i as usize) % mem_size] += 1;
        }

        counter += 1
    }
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut memory: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mem_size: usize = memory.len();

    let mut seen: HashMap<Vec<i32>, i32> = HashMap::new();
    let mut counter = 0;
    loop {
        let solution = match seen.contains_key(&memory.clone()) {
            false => {
                seen.insert(memory.clone(), counter);
                -1
            }
            true => return counter - seen.get(&memory.clone()).unwrap(),
        };
        if solution != -1 {
            return solution;
        }

        let max = memory.iter().max().unwrap().clone();
        let pos = memory.iter().position(|&e| e == max).unwrap();
        memory[pos] = 0;

        for i in 1..=max {
            memory[(pos + i as usize) % mem_size] += 1;
        }

        counter += 1
    }
}
