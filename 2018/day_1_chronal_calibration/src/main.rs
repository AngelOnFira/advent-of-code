use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let second_frequency = find_frequency();
    println!("{}", second_frequency);
}

fn find_frequency() -> i32 {
    let mut frequency: i32 = 0;
    let mut frequency_set: HashSet<i32> = HashSet::new();
    let mut found: bool = false;

    while !found {
        let file = File::open("src/input.txt").unwrap();
        for line in BufReader::new(file).lines() {
            if frequency_set.contains(&frequency) {
                found = true;
                break
            }
            frequency_set.insert(frequency);
            let change = String::from(line.unwrap());

            let mut mult = 1;
            if &change[0..1] == "-" {
                mult = -1;
            }

            frequency += mult * &change[1..].parse::<i32>().unwrap();
        }
    }

    frequency
}