use itertools::iproduct;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;

pub fn main() {
    
}

fn day17() {
    let contents = fs::read_to_string("day17.txt")
        .expect("Can't find input file day17.txt");

    let mut last_grid = HashSet::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character == '#' {
                last_grid.insert([x as i32, y as i32, 0, 0]);
            }
        }
    }

    for _ in 0..6 {
        let mut area = Vec::new();
        for i in 0..4 {
            area.push(last_grid.iter().map(|&x| x[3 - i]).min().unwrap() - 1 as i32);
            area.push(last_grid.iter().map(|&x| x[3 - i]).max().unwrap() + 1 as i32);
        }

        let new_grid: HashSet<[i32; 4]> = (area[0]..=area[1])
            .into_par_iter()
            .map(|w| {
                (area[2]..=area[3])
                    .into_par_iter()
                    .map(|z| {
                        let mut all_found = Vec::new();
                        for (y, x) in iproduct!(area[4]..=area[5], area[6]..=area[7]) {
                            let this_cell;
                            if last_grid.contains(&[x, y, z, w]) {
                                this_cell = 1;
                            } else {
                                this_cell = 0;
                            }

                            let mut active = 0;
                            for (xk, yk, zk, wk) in iproduct!(-1..=1, -1..=1, -1..=1, -1..=1) {
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
                        }
                        all_found
                    })
                    .flatten()
                    .collect::<Vec<[i32; 4]>>()
            })
            .flatten()
            .collect();
        last_grid = new_grid.clone();
    }
    println!("The answer is: {}", last_grid.len());
}