use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    find_boxes();
}

fn find_boxes() {

    let mut strings: Vec<String> = Vec::new();

    let file = File::open("src/input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        strings.push(line.unwrap());
    }

    for line in &strings {
        for compare_line in &strings {
            let mut chars: HashMap<char, i32> = HashMap::new();

            for this_char in compare_line.chars() {
                if !chars.contains_key(&this_char) {
                    chars.insert(this_char, 1);
                }
                else {
                    let count = chars.entry(this_char).or_insert(0);
                    *count += 1;
                }
            }

            let mut extra: bool = true;
            let mut found: bool = true;
            for this_char in line.chars() {
                if !chars.contains_key(&this_char) || chars[&this_char] == 0 {
                    if extra {
                        extra = false;
                    }
                    else {
                        found = false;
                        break
                    }
                }
                else {
                    let count = chars.entry(this_char).or_insert(0);
                    *count -= 1;
                }
            }
            if found && !extra {
                println!("{} {}", line, compare_line);
            }
        }
    }
}