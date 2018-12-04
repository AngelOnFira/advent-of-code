use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap};

extern crate regex;
use regex::Regex;

fn main() {
    let mut strings: Vec<String> = Vec::new();
    let mut fabric: HashMap<String, i32> = HashMap::new();

    let file = File::open("src/input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        strings.push(line.unwrap());
    }

    let re = Regex::new(r"^#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)$").unwrap();

    for line in &strings {
        let arr = re.captures(line).unwrap();

        //let id: String = arr[1].to_string();
        let x_start: i32 = arr[2].parse().unwrap();
        let y_start: i32 = arr[3].parse().unwrap();
        let x_end: i32 = arr[4].parse().unwrap();
        let y_end: i32 = arr[5].parse().unwrap();

        for x in x_start..(x_start + x_end) {
            for y in y_start..(y_start + y_end) {
                let location = format!("{},{}", x, y);

                let count = fabric.entry(location).or_insert(0);
                *count += 1;
            }
        }

    }
    let mut count = 0;
    for (_k, v) in &fabric {
        if v >= &2 {
            count += 1;
        }
    }
    println!("{}", count);
}