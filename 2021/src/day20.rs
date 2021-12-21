use std::collections::HashMap;

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> (String, HashMap<(i32, i32), bool>) {
    let out_string = input.lines().next().unwrap().to_string();

    let mut map = HashMap::new();

    let lines = input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    for (y, line) in lines[2..].iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert((x as i32, y as i32), true);
            }
        }
    }

    (out_string, map)
}

fn iterate(input: &(String, HashMap<(i32, i32), bool>), num: i32) -> i32 {
    let mut old_map = input.1.clone();
    let lookup_string = input.0.clone();
    for i in 0..num {
        // Find the lowest and greatest x and y values
        let mut lowest_x = 0;
        let mut lowest_y = 0;
        let mut greatest_x = 0;
        let mut greatest_y = 0;

        for (x, y) in old_map.keys() {
            let x = x.clone();
            let y = y.clone();

            if x < lowest_x {
                lowest_x = x;
            }
            if x > greatest_x {
                greatest_x = x;
            }
            if y < lowest_y {
                lowest_y = y;
            }
            if y > greatest_y {
                greatest_y = y;
            }
        }

        let mut new_map = HashMap::new();

        // Create a 3x3 kernel and get the binary value of the ordered bits
        for y in lowest_y - 1..=greatest_y + 1 {
            for x in lowest_x - 1..=greatest_x + 1 {
                let mut binary_string = "".to_string();
                for y_offset in -1 as i32..=1 {
                    for x_offset in -1 as i32..=1 {
                        let point = (x + x_offset as i32, y + y_offset as i32);

                        if x + x_offset < lowest_x
                            || x + x_offset > greatest_x
                            || y + y_offset < lowest_y
                            || y + y_offset > greatest_y
                        {
                            if i % 2 == 0 {
                                binary_string.push('0');
                            } else {
                                binary_string.push('1');
                            }
                        } else {
                            match old_map.get(&point) {
                                Some(true) => binary_string.push('1'),
                                _ => binary_string.push('0'),
                            }
                        }
                    }
                }
                let binary_value = i32::from_str_radix(&binary_string, 2).unwrap();
                new_map.insert(
                    (x, y),
                    lookup_string.chars().nth(binary_value as usize).unwrap() == '#',
                );
            }
        }

        old_map = new_map.clone();

        // // Draw the map
        // for y in lowest_y - 1..=greatest_y + 1 {
        //     for x in lowest_x - 1..=greatest_x + 1 {
        //         match old_map.get(&(x, y)) {
        //             Some(true) => print!("#"),
        //             _ => print!(" "),
        //         }
        //     }
        //     println!();
        // }
        // println!();
        // println!();
    }
    old_map.values().filter(|x| **x).count() as i32
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &(String, HashMap<(i32, i32), bool>)) -> i32 {
    iterate(input, 2)
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &(String, HashMap<(i32, i32), bool>)) -> i32 {
    iterate(input, 50)
}
