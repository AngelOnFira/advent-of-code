use std::collections::HashSet;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut houses: HashSet<(i32, i32)> = HashSet::new();

    houses.insert((x, y));

    for character in input.chars() {
        match character {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y -= 1,
            'v' => y += 1,
            _ => unreachable!(),
        };

        houses.insert((x, y));
    }
    houses.len() as i32
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut pos = vec![vec![0, 0], vec![0, 0]];
    let mut houses: HashSet<(i32, i32)> = HashSet::new();

    houses.insert((0, 0));

    for (i, character) in input.chars().enumerate() {
        match character {
            '>' => pos[i % 2][0] += 1,
            '<' => pos[i % 2][0] -= 1,
            '^' => pos[i % 2][1] -= 1,
            'v' => pos[i % 2][1] += 1,
            _ => unreachable!(),
        };

        houses.insert((pos[i % 2][0], pos[i % 2][1]));
    }
    houses.len() as i32
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
