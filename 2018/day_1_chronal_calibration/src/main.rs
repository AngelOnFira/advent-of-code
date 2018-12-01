use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut frequency: i32 = 0;

    let file = File::open("src/input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        let change = String::from(line.unwrap());

        let mut mult = 1;
        if &change[0..1] == "-" {
            mult = -1;
        }

        frequency += mult * &change[1..].parse::<i32>().unwrap();
    }
    println!("{}", frequency);
}