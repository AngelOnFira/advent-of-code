use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap};

extern crate regex;
use regex::Regex;

fn main() {
    let mut strings: Vec<String> = Vec::new();
    let mut sleep: HashMap<String, usize> = HashMap::new();
    let mut sleep_mins: HashMap<String, [usize; 60]> = HashMap::new();

    let file = File::open("src/input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        strings.push(line.unwrap());
    }

    let re = Regex::new(r"^\[1518-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)\] ([a-zA-Z]+) ([a-zA-Z0-9#]+)").unwrap();

    let mut last_id: String = "".to_string();
    let mut last_min: usize = 0;

    for line in &strings {
        let arr = re.captures(line).unwrap();

        let action: String = arr[5].to_string();

        if action == "Guard" {
            last_id = arr[6].to_string();
        }
        else if action == "falls" {
            last_min = arr[4].parse().unwrap();
        }
        else if action == "wakes" {
            let this_min: usize = arr[4].parse().unwrap();

            let count = sleep.entry(last_id.clone()).or_insert(0);
            *count += this_min - last_min;

            let arr = sleep_mins.entry(last_id.clone()).or_insert([0; 60]);
            for min in last_min..this_min {
                (*arr)[min] += 1;
            }
        }
    }

    let mut max_key: String = "".to_string();
    let mut max_value: usize = 0;

    let mut guard_min_max = 0;
    let mut guard_min_max_id = "".to_string();

    for (k, v) in sleep {
        // Need to learn variable moving cause this is terrible
        let a = k.clone();

        if v > max_value {
            max_key = k;
            max_value = v;
        }

        let guard_arr = sleep_mins.get(&a).unwrap();

        for i in 0..60 {
            if guard_arr[i] > guard_min_max {
                guard_min_max = guard_arr[i];
                guard_min_max_id = a.clone();
            }
        }   
    }

    let arr = sleep_mins.get(&max_key).unwrap();
    let mut max_min = 0;
    let mut max_min_index = 0;

    for i in 0..60 {
        if arr[i] > max_min {
            max_min = arr[i];
            max_min_index = i;
        }
    }

    le
    println!("Guard with id {} slept for {} mins. They slept the most in minute {}. (Checksum: {})", max_key, max_value, max_min_index, (max_key * max_min_index));
    println!("The guard with id {} slept the most times during min {}", guard_min_max_id, guard_min_max);
}