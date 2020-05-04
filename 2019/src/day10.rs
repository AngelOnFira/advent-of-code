use std::cmp::{max, min};

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut map: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
        let mut this_line: Vec<bool> = Vec::new();

        for bit in line.chars() {
            match bit {
                '.' => this_line.push(false),
                '#' => this_line.push(true),
                _ => unreachable!(),
            }
        }

        map.push(this_line);
    }

    let mut count = 0;

    for y_usize in 0..map.len() {
        for x_usize in 0..map[0].len() {
            let x = x_usize as i32;
            let y = y_usize as i32;
            let mut this_count = 0;

            if map[y_usize][x_usize] {
                for y_check_usize in 0..map.len() {
                    'cast: for x_check_usize in 0..map[0].len() {
                        let x_check = x_check_usize as i32;
                        let y_check = y_check_usize as i32;

                        if map[y_check_usize][x_check_usize]
                            && !(x_usize == x_check_usize && y_usize == y_check_usize)
                        {
                            let diff_x = x_check - x;
                            let diff_y = y_check - y;

                            let gcd = gcd(diff_x, diff_y);

                            let mut test_x = x;
                            let mut test_y = y;

                            test_x += diff_x / gcd;
                            test_y += diff_y / gcd;

                            while !(test_x == x_check && test_y == y_check) {
                                if map[test_y as usize][test_x as usize] {
                                    continue 'cast;
                                }

                                test_x += diff_x / gcd;
                                test_y += diff_y / gcd;
                            }
                            this_count += 1;
                        }
                    }
                }
            }
            if this_count > count {
                count = this_count;
            }
        }
    }

    count
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        return max(a.abs(), b.abs());
    }

    let small = min(a.abs(), b.abs());
    for i in 0..=small {
        if a.abs() % (small - i) == 0 && b.abs() % (small - i) == 0 {
            return small - i;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::gcd;
    use super::solve_part1 as part1;
    //use super::solve_part2 as part2;

    #[test]
    fn sample1011() {
        assert_eq!(part1(".#..#\n.....\n#####\n....#\n...##"), 8);
    }

    #[test]
    fn sample1012() {
        assert_eq!(part1(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##"), 210);
    }

    #[test]
    fn sample1013() {
        assert_eq!(part1("##\n##\n##"), 2);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(134124232, 2), 2);
    }
}
