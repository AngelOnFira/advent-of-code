#[aoc_generator(day11)]
fn parse_input_day11(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> i32 {
    let mut current_map: Vec<Vec<char>> = input.clone();
    let mut next_map: Vec<Vec<char>> = Vec::new();

    let mut last: i32 = 0;
    let mut curr: i32 = 0;
    while last != curr || last == 0 {
        next_map = Vec::new();
        for (y, row) in current_map.clone().iter().enumerate() {
            let mut new_row: Vec<char> = Vec::new();
            for (x, seat) in row.iter().enumerate() {
                let mut empty = 0;
                let mut occupied = 0;
                for yk in 0..3 {
                    for xk in 0..3 {
                        if xk == yk {
                            continue;
                        }
                        if let Some(row) = current_map.get(y + yk - 1) {
                            if let Some(seat) = row.get(x + xk - 1) {
                                match seat {
                                    'L' => empty += 1,
                                    '#' => occupied += 1,
                                    _ => (),
                                }
                            }
                        }
                    }
                }
                let curr_seat = current_map[y][x];

                if curr_seat == 'L' {
                    if occupied == 0 {
                        new_row.push('#');
                    } else {
                        new_row.push('L');
                    }
                } else if curr_seat == '#' {
                    if occupied >= 4 {
                        new_row.push('L');
                    } else {
                        new_row.push('#');
                    }
                } else {
                    new_row.push('.');
                }
            }
            next_map.push(new_row);
        }
        last = curr;
        curr = next_map.iter().fold(0, |acc, line| {
            line.iter().filter(|&&seat| seat == '#').count()
        }) as i32;
        for line in next_map.clone() {
            println!("");
            for seat in line {
                print!("{}", seat);
            }
        }

        println!("");
        println!("");

        current_map = next_map;
    }
    curr
}
