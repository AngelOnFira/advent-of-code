use itertools::iproduct;
// use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;

#[aoc_generator(day17)]
fn parse_input_day17(input: &str) -> HashSet<[i32; 4]> {
    let mut out_set = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character == '#' {
                out_set.insert([x as i32, y as i32, 0, 0]);
            }
        }
    }
    out_set
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &HashSet<[i32; 4]>) -> i32 {
    let mut last_grid = input.clone();

    for _ in 0..6 {
        // let mut new_grid = HashSet::new();
        let mut area = Vec::new();
        for i in 0..4 {
            let mut first = true;
            area.push(last_grid.iter().map(|&x| x[3 - i]).min().unwrap() - 1 as i32);
            area.push(last_grid.iter().map(|&x| x[3 - i]).max().unwrap() + 1 as i32);
        }

        let new_grid: HashSet<[i32; 4]> = (area[0]..=area[1])
            .into_par_iter()
            .map(|w| {
                let mut all_found = Vec::new();
                for (z, y, x) in iproduct!(area[2]..=area[3], area[4]..=area[5], area[6]..=area[7])
                {
                    // dbg!((x, y, z, w));

                    // for z in area[2]..=area[3] {
                    //     for y in area[4]..=area[5] {
                    //         for x in area[6]..=area[7] {
                    let mut this_cell;
                    if last_grid.contains(&[x, y, z, w]) {
                        this_cell = 1;
                    } else {
                        this_cell = 0;
                    }

                    let mut active = 0;
                    for (xk, yk, zk, wk) in iproduct!(-1..=1, -1..=1, -1..=1, -1..=1) {
                        // dbg!((xk, yk, zk, wk));
                        if xk == 0 && yk == 0 && zk == 0 && wk == 0 {
                            continue;
                        }
                        if last_grid.contains(&[x + xk, y + yk, z + zk, w + wk]) {
                            active += 1;
                        }
                    }

                    if this_cell == 1 {
                        if active == 2 || active == 3 {
                            all_found.push([x, y, z, w]);
                        }
                    } else if this_cell == 0 {
                        if active == 3 {
                            all_found.push([x, y, z, w]);
                        }
                    }
                    //         }
                    //     }
                    // }
                    // Some(None)
                }
                return all_found;
            })
            .flatten()
            .collect();
        last_grid = new_grid.clone();
    }
    last_grid.len() as i32
}
