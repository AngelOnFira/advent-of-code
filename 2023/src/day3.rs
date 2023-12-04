use std::collections::HashMap;

use itertools::Itertools;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> i32 {
    // 467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..

    // We need to find all numbers that are adjacent to a symbol. We should do
    // this by making a symbol map first, the getting the position of all
    // numbers, and their dimensions. We'll compare this distance to the symbols
    // and see if we're close enough. Diagonal is adjacent too.

    // Get the map of anything that isn't a . or a number
    let symbol_hashmap = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| !c.is_numeric() && *c != '.')
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect::<HashMap<(i32, i32), char>>();

    // dbg!(&symbol_hashmap);

    // Next, extract all the numbers and their positions into a hashmap. We'll
    // be getting the sum of all numbers that are adjacent to a symbol after. A
    // number needs to combine with all of it's consective numbers, so 123 isn't
    // 1 and 2 and 3, it's 123.
    #[derive(Debug)]
    struct Number {
        position: (i32, i32),
        value: i32,
        length: i32,
    }

    let numbers: Vec<Number> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut curr_num = String::new();
            let mut numbers = Vec::new();
            for (x, c) in line.chars().enumerate() {
                if c.is_numeric() {
                    curr_num.push(c);
                } else if !curr_num.is_empty() {
                    let num = curr_num.parse::<i32>().unwrap();
                    let length = curr_num.len() as i32;
                    curr_num.clear();
                    numbers.push(Number {
                        position: (x as i32 - length, y as i32),
                        value: num,
                        length,
                    });
                }
            }
            if !curr_num.is_empty() {
                let num = curr_num.parse::<i32>().unwrap();
                let length = curr_num.len() as i32;
                curr_num.clear();
                numbers.push(Number {
                    position: (line.len() as i32 - length, y as i32),
                    value: num,
                    length,
                });
            }
            numbers
        })
        .collect::<Vec<Number>>();

    // Next, we need to find the sum of all numbers that are adjacent to a
    // symbol. We'll do this by getting the position of all numbers, and their
    // dimensions. We'll compare this distance to the symbols and see if we're
    // close enough. Diagonal is adjacent too.

    // dbg!(&numbers);

    numbers
        .iter()
        .filter(|number| {
            let (x, y) = number.position;
            let length = number.length;
            let mut adjacent = false;
            for (x2, y2) in (x - 1..=x + length).cartesian_product(y - 1..=y + 1) {
                // Check just if there is an entry in the symbol hashmap, we
                // don't care what it is
                if symbol_hashmap.contains_key(&(x2, y2)) {
                    adjacent = true;
                    break;
                }
            }
            adjacent
        })
        .map(|number| number.value)
        .sum::<i32>()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> i32 {
    // 467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..

    // We need to find all numbers that are adjacent to a symbol. We should do
    // this by making a symbol map first, the getting the position of all
    // numbers, and their dimensions. We'll compare this distance to the symbols
    // and see if we're close enough. Diagonal is adjacent too.

    // Get the map of anything that isn't a . or a number
    let symbol_hashmap = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| !c.is_numeric() && *c != '.')
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect::<HashMap<(i32, i32), char>>();

    // Next, extract all the numbers and their positions into a hashmap. We'll
    // be getting the sum of all numbers that are adjacent to a symbol after. A
    // number needs to combine with all of it's consective numbers, so 123 isn't
    // 1 and 2 and 3, it's 123.
    #[derive(Debug)]
    struct Number {
        position: (i32, i32),
        value: i32,
        length: i32,
    }

    let numbers: Vec<Number> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut curr_num = String::new();
            let mut numbers = Vec::new();
            for (x, c) in line.chars().enumerate() {
                if c.is_numeric() {
                    curr_num.push(c);
                } else if !curr_num.is_empty() {
                    let num = curr_num.parse::<i32>().unwrap();
                    let length = curr_num.len() as i32;
                    curr_num.clear();
                    numbers.push(Number {
                        position: (x as i32 - length, y as i32),
                        value: num,
                        length,
                    });
                }
            }
            if !curr_num.is_empty() {
                let num = curr_num.parse::<i32>().unwrap();
                let length = curr_num.len() as i32;
                curr_num.clear();
                numbers.push(Number {
                    position: (line.len() as i32 - length, y as i32),
                    value: num,
                    length,
                });
            }
            numbers
        })
        .collect::<Vec<Number>>();

    // This time, we need to find any * symbols that are adjacent to exactly 2
    // numbers, and multiply them together. Get the sum of all of those numbers.

    symbol_hashmap
        .iter()
        .filter(|(_, symbol)| **symbol == '*')
        .filter_map(|((x, y), _)| {
            let mut adjacent = 0;
            let mut product = 1;

            // Go through each number, and find any that are adjacent to this
            // symbol
            for number in &numbers {
                let (x2, y2) = number.position;
                let length = number.length;
                for (x3, y3) in (x2 - 1..=x2 + length).cartesian_product(y2 - 1..=y2 + 1) {
                    if (x3, y3) == (*x, *y) {
                        adjacent += 1;
                        product *= number.value;
                    }
                }
            }

            if adjacent == 2 {
                Some(product)
            } else {
                None
            }
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
