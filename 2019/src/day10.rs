use std::cmp::{max, min};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let (count, _, _, _) = get_station(input);

    count
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let (count, max_x, max_y, map) = get_station(input);
    println!("{} {}", max_x, max_y);

    let mut stations: HashMap<i64, Vec<(f64, usize, usize)>> = HashMap::new();

    for (y, row) in map.iter().enumerate() {
        for (x, station) in row.iter().enumerate() {
            if station == &false {
                continue;
            }

            let dx = (max_x - x as i32) as f64;
            let dy = (max_y - y as i32) as f64;
            let distance = (((max_x - x as i32).pow(2) + (max_y - y as i32).pow(2)) as f64).sqrt();

            if distance.abs() < 0.01 {
                continue;
            }

            let station_direction = (dx.atan2(dy).to_degrees() * 1000.0) as i64;

            stations
                .entry(station_direction)
                .or_insert(Vec::new())
                .push((distance, x, y));
        }
    }

    let mut sorted_keys: Vec<i64> = stations.keys().cloned().into_iter().collect();
    sorted_keys.sort_by(|x, y| y.cmp(&x));

    let mut index = sorted_keys.iter().position(|&x| x == 0).unwrap();
    let mut station_counter = 0;
    let mut curr_station = (0.0, 0, 0);

    while station_counter < 200 {
        let angle = sorted_keys.get(index).unwrap();
        let curr_station_line_wrapped = stations.get_mut(angle);
        let curr_station_line = match curr_station_line_wrapped {
            Some(value) => value,
            None => continue,
        };

        if curr_station_line.len() == 0 {
            index += 1;
            index %= sorted_keys.len();
            continue;
        }

        curr_station_line.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());

        curr_station = curr_station_line.remove(0);

        index += 1;
        index %= sorted_keys.len();
        station_counter += 1;
    }

    (curr_station.1 * 100 + curr_station.2) as i32
}

fn get_station(input: &str) -> (i32, i32, i32, Vec<Vec<bool>>) {
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
    let mut max_x = 0;
    let mut max_y = 0;

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
                max_x = x;
                max_y = y;
            }
        }
    }

    (count, max_x, max_y, map)
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
    use super::solve_part2 as part2;

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

    #[test]
    fn sample1021() {
        assert_eq!(part2(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##"), 802);
    }
}
