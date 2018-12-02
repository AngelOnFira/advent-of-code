use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let checksum = find_checksum();
    println!("{}", checksum);
}

fn find_checksum() -> i32 {

    let mut twos = 0;
    let mut threes = 0;

    let file = File::open("src/input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        let mut chars: HashMap<char, i32> = HashMap::new();
        let mut this_two = 0;
        let mut this_three = 0;
        let this_line = String::from(line.unwrap());

        for this_char in this_line.chars() {
            if !chars.contains_key(&this_char) {
                chars.insert(this_char, 1);
            }
            else {
                let count = chars.entry(this_char).or_insert(0);
                *count += 1;
            }
        }

        for (_k, v) in chars {
            if v == 2 {
                this_two = 1;
            }
            if v == 3 {
                this_three = 1;
            }
        }

        twos += this_two;
        threes += this_three;
    }

    twos * threes
}