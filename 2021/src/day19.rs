use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone)]
pub struct Scanner {
    beacons: Vec<Point>,
}

// Impl a push method for the Vec<Point>
impl Scanner {
    pub fn new(beacons: Vec<Point>) -> Scanner {
        Scanner { beacons }
    }

    pub fn push(&mut self, point: Point) {
        self.beacons.push(point);
    }

    pub fn rotate(&mut self, permutation: i32, offset: (i32, i32, i32)) -> HashSet<Point> {
        // Permutation is one of the 24 possible permutations of the beacons

        // Unfortunately, there's a second problem: the scanners also don't know
        // their rotation or facing direction. Due to magnetic alignment, each
        // scanner is rotated some integer number of 90-degree turns around all
        // of the x, y, and z axes. That is, one scanner might call a direction
        // positive x, while another scanner might call that direction negative
        // y. Or, two scanners might agree on which direction is positive x, but
        // one scanner might be upside-down from the perspective of the other
        // scanner. In total, each scanner could be in any of 24 different
        // orientations: facing positive or negative x, y, or z, and considering
        // any of four directions "up" from that facing.

        let negatives = vec![
            (1, 1, 1),
            (1, 1, -1),
            (1, -1, 1),
            (1, -1, -1),
            (-1, 1, 1),
            (-1, 1, -1),
            (-1, -1, 1),
            (-1, -1, -1),
        ];

        self.beacons
            .iter()
            .map(|point| {
                let mut new_point = point.clone();

                new_point.x = (point.x + offset.0) * negatives[(permutation % 8) as usize].0;
                new_point.y = (point.y + offset.1) * negatives[(permutation % 8) as usize].1;
                new_point.z = (point.z + offset.2) * negatives[(permutation % 8) as usize].2;

                let temp;
                if permutation / 3 == 0 {
                    // Swap x and z
                    temp = new_point.x;
                    new_point.x = new_point.z;
                    new_point.z = temp;
                } else if permutation / 3 == 1 {
                    // Swap y and z
                    temp = new_point.y;
                    new_point.y = new_point.z;
                    new_point.z = temp;
                } else if permutation / 3 == 2 {
                    // Swap x and y
                    temp = new_point.x;
                    new_point.x = new_point.y;
                    new_point.y = temp;
                }
                new_point
            })
            .collect()
    }

    pub fn get_set(&self) -> HashSet<Point> {
        self.beacons.iter().cloned().collect()
    }
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> HashMap<i32, Scanner> {
    // --- scanner 0 ---
    // 0,2
    // 4,1
    // 3,3

    // --- scanner 1 ---
    // -1,-1
    // -5,0
    // -2,1

    let mut curr_scanner_num = -1;
    let mut scanners = HashMap::new();

    for line in input.lines() {
        if line == "" {
            continue;
        } else if &line[0..2] == "--" {
            curr_scanner_num += 1;
            continue;
        } else {
            let points: Vec<i32> = line
                .split(",")
                .map(|point| point.parse::<i32>().unwrap())
                .collect();
            scanners
                .entry(curr_scanner_num)
                .or_insert(Scanner { beacons: vec![] })
                .push(Point {
                    x: points[0],
                    y: points[1],
                    z: points[2],
                });
        }
    }

    scanners
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &HashMap<i32, Scanner>) -> i32 {
    let mut found_beacons: HashSet<Point> = HashSet::new();

    let baseline_scanner = input.get(&0).unwrap();
    let baseline_set: HashSet<Point> = baseline_scanner.get_set();

    // Go through each scanner
    for scanner_num in 0..*input.keys().max().unwrap() + 1 {
        let mut scanner = input.get(&scanner_num).unwrap().clone();

        // for perm in 0..24 {
        //     let rotated_beacons = scanner.rotate(perm, (x, y, z));

        //     let overlap_count = baseline_set.intersection(&rotated_beacons).count();

        //     if overlap_count >= 12 {
        //         found_beacons.extend(rotated_beacons);
        //         // break 'outer;
        //     }
        // }

        // Find the permutation with the largest number of shared beacons
        // let mut max_overlap_count = 0;
        // let mut max_overlap_set = HashSet::new();

        // Add the beacons to the found_beacons set
        // found_beacons.extend(max_overlap_set);
    }
    found_beacons.len() as i32
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &HashMap<i32, Scanner>) -> i32 {
    3
}
