use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day7)]
fn parse_input_day7(input: &str) -> HashMap<String, Vec<(String, i32)>> {
    let original = Regex::new(r"(.*\s.*)\s").unwrap();
    let bags = Regex::new(r"(\d+)\s(.*\s.*)\sbag.*").unwrap();
    input
        .lines()
        .map(|line| {
            let split = line.split("bags contain ").collect::<Vec<_>>();
            let bag = original
                .captures(split[0])
                .unwrap()
                .get(1)
                .map_or("".to_string(), |m| m.as_str().to_string());
            if split[1] == "no other bags." {
                return (bag, vec![]);
            } else {
                let inner_bags = split[1]
                    .split(",")
                    .map(|bag_inside| {
                        let caps = bags.captures(bag_inside).unwrap();
                        return (
                            caps.get(2)
                                .map_or("".to_string(), |m| m.as_str().to_string()),
                            caps.get(1)
                                .map_or(0, |m| m.as_str().parse::<i32>().unwrap()),
                        );
                    })
                    .collect::<Vec<(String, i32)>>();
                return (bag, inner_bags);
            }
            unreachable!();
        })
        .collect::<HashMap<String, Vec<(String, i32)>>>()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &HashMap<String, Vec<(String, i32)>>) -> i32 {
    // let mut bags
    input
        .keys()
        .filter(|bag_type| has_shiny_gold(bag_type.to_string(), input))
        .count() as i32
        - 1
}

pub fn has_shiny_gold(color: String, bags: &HashMap<String, Vec<(String, i32)>>) -> bool {
    if color == "shiny gold" {
        return true;
    }
    for bag in bags.get(&color).unwrap() {
        if has_shiny_gold((*bag.0).to_string(), bags) {
            return true;
        }
    }
    return false;
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &HashMap<String, Vec<(String, i32)>>) -> i32 {
    // let mut bags
    count_bags("shiny gold".to_string(), input) - 1
}

pub fn count_bags(color: String, bags: &HashMap<String, Vec<(String, i32)>>) -> i32 {
    let bags_inside = bags.get(&color).unwrap();
    if bags_inside.len() == 0 {
        return 1;
    } else {
        return bags_inside
            .iter()
            .map(|bag| count_bags(bag.0.clone(), bags) * bag.1)
            .sum::<i32>()
            + 1;
    }
}
