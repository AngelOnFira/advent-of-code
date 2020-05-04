use std::collections::{HashSet, HashMap};

#[aoc(day24, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut map: Vec<Vec<bool>> = Vec::new();
    let mut biodiversity: HashSet<usize> = HashSet::new();

    for line in input.lines() {
        let mut line_vec = Vec::new();
        for symbol in line.chars() {
            match symbol {
                '#' => line_vec.push(true),
                '.' => line_vec.push(false),
                _ => unreachable!(),
            }
        }
        map.push(line_vec);
    }

    loop {
        let mut new_map: Vec<Vec<bool>> = Vec::new();
        let mut bio_total = 0;

        for y in 0..map.len() {
            let mut new_map_line = Vec::new();
            for x in 0..map[0].len() {
                let mut alive = 0;
                if x > 0 && map[y][x - 1] {
                    alive += 1;
                }
                if x < map[0].len() - 1 && map[y][x + 1] {
                    alive += 1;
                }
                if y > 0 && map[y - 1][x] {
                    alive += 1;
                }
                if y < map[0].len() - 1 && map[y + 1][x] {
                    alive += 1;
                }

                if map[y][x] && alive != 1 {
                    new_map_line.push(false);
                }
                else if !map[y][x] && (alive == 1 || alive == 2) {
                    new_map_line.push(true);
                }
                else {
                    new_map_line.push(map[y][x]);
                }

                if new_map_line.last().copied().unwrap() {
                    bio_total += 2usize.pow((y * map[0].len() + x) as u32);
                }
            }
            new_map.push(new_map_line);
        }
        if !biodiversity.insert(bio_total) {
            return bio_total as i32;
        }
        map = new_map;
    }
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut map: Vec<Vec<bool>> = Vec::new();
    // let mut levels= HashMap::new();

    let mut max_dim = 0;
    let mut min_dim = 0;

    // levels.insert(0, map);

    for line in input.lines() {
        let mut line_vec = Vec::new();
        for symbol in line.chars() {
            match symbol {
                '#' => line_vec.push(true),
                '.' => line_vec.push(false),
                _ => unreachable!(),
            }
        }
        map.push(line_vec);
    }

    loop {
        let mut new_map: Vec<Vec<bool>> = Vec::new();

        for y in 0..map.len() {
            let mut new_map_line = Vec::new();
            for x in 0..map[0].len() {
                let mut alive = 0;
                if x > 0 && map[y][x - 1] {
                    alive += 1;
                }
                if x < map[0].len() - 1 && map[y][x + 1] {
                    alive += 1;
                }
                if y > 0 && map[y - 1][x] {
                    alive += 1;
                }
                if y < map[0].len() - 1 && map[y + 1][x] {
                    alive += 1;
                }

                if map[y][x] && alive != 1 {
                    new_map_line.push(false);
                }
                else if !map[y][x] && (alive == 1 || alive == 2) {
                    new_map_line.push(true);
                }
                else {
                    new_map_line.push(map[y][x]);
                }
            }
            new_map.push(new_map_line);
        }
        map = new_map;
    }
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    // use super::solve_part2 as part2;

    #[test]
    fn sample311() {
        assert_eq!(part1("....#\n#..#.\n#..##\n..#..\n#...."), 159);
    }

    #[test]
    fn sample312() {
        assert_eq!(part1(".....\n.....\n.....\n#....\n.#..."), 159);
    }
}
