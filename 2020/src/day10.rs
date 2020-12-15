use std::collections::HashMap;

#[aoc_generator(day10)]
fn parse_input_day10(input: &str) -> Vec<i64> {
    let mut joltage_list: Vec<i64> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    joltage_list.sort();
    joltage_list
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Vec<i64>) -> i64 {
    let mut joltage_list = input.clone();

    let mut joltage = 0;
    let mut jumps = [0; 3];
    jumps[2] += 1;

    while joltage < joltage_list[(joltage_list.len() - 1) as usize] {
        for i in 0..3 {
            if joltage_list.contains(&(joltage + i + 1)) {
                jumps[i as usize] += 1;
                joltage += i + 1;
                break;
            }
        }
    }
    jumps[0] * jumps[2]
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Vec<i64>) -> i64 {
    let mut joltage_list = input.clone();
    joltage_list.insert(0, 0);
    joltage_list.reverse();
    let max_element = joltage_list[0] + 3;
    let mut found_nums: HashMap<i64, i64> = HashMap::new();
    found_nums.insert(joltage_list[0] + 3, 1);
    for joltage in joltage_list.clone() {
        let mut total = 0;
        for i in 0..3 {
            let test_joltage = joltage + i + 1;
            if joltage_list.contains(&test_joltage) || test_joltage == max_element {
                total += found_nums.get(&test_joltage).unwrap();
            }
        }
        found_nums.insert(joltage, total);
    }

    *found_nums.get(&0).unwrap()
}
