use std::collections::HashMap;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let num = input.parse::<i32>().unwrap();
    let mut pos = (0, 0);

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut dir = 0;
    let mut dir_counter: i32 = 0;
    let mut dir_base: i32 = 0;

    for i in 0..(num - 1) {
        map.insert(pos, i + 1);

        match dir {
            0 => pos.0 += 1,
            1 => pos.1 -= 1,
            2 => pos.0 -= 1,
            3 => pos.1 += 1,
            _ => (),
        }

        if dir_counter == 0 {
            dir_base += 1;
            dir_counter = dir_base / 2 + 1;
            dir += 1;
            dir %= 4;
        }
        dir_counter -= 1;
    }

    pos.0.abs() + pos.1.abs()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let num = input.parse::<i32>().unwrap();
    let mut pos = (0, 0);

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut dir = 0;
    let mut dir_counter: i32 = 0;
    let mut dir_base: i32 = 0;

    let mut total = 1;
    loop {
        for y in -1..=1 {
            for x in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }
                total += map.get(&(pos.0 + x, pos.1 + y)).unwrap_or(&0);
            }
        }
        if total > num {
            return total;
        }
        map.insert(pos, total);

        match dir {
            0 => pos.0 += 1,
            1 => pos.1 -= 1,
            2 => pos.0 -= 1,
            3 => pos.1 += 1,
            _ => (),
        }

        if dir_counter == 0 {
            dir_base += 1;
            dir_counter = dir_base / 2 + 1;
            dir += 1;
            dir %= 4;
        }
        dir_counter -= 1;
        total = 0;
    }
}
