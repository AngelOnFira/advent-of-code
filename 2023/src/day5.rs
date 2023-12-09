use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> i64 {
    // seeds: 79 14 55 13

    // seed-to-soil map:
    // 50 98 2
    // 52 50 48

    // soil-to-fertilizer map:
    // 0 15 37
    // 37 52 2
    // 39 0 15

    // fertilizer-to-water map:
    // 49 53 8
    // 0 11 42
    // 42 0 7
    // 57 7 4

    // water-to-light map:
    // 88 18 7
    // 18 25 70

    // light-to-temperature map:
    // 45 77 23
    // 81 45 19
    // 68 64 13

    // temperature-to-humidity map:
    // 0 69 1
    // 1 0 69

    // Read the first line of the input, which is the seed numbers. Then, read
    // sections after that, which are the maps. Extract the type (from and to),
    // then all the numbers
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split("seeds: ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    #[derive(Clone)]
    struct Map {
        from: String,
        to: String,
        numbers: Vec<Change>,
    }

    //     Consider again the example seed-to-soil map:

    // 50 98 2
    // 52 50 48
    // The first line has a destination range start of 50, a source range start of
    // 98, and a range length of 2. This line means that the source range starts at
    // 98 and contains two values: 98 and 99. The destination range is the same
    // length, but it starts at 50, so its two values are 50 and 51. With this
    // information, you know that seed number 98 corresponds to soil number 50 and
    // that seed number 99 corresponds to soil number 51.
    #[derive(Clone)]
    struct Change {
        dest_range_start: i64,
        source_range_start: i64,
        range_length: i64,
    }

    let mut seed_to_soil = Vec::new();

    // Go through each line, and add the changes to the hashmap
    let mut curr_map: Option<Map> = None;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        // If the first char is not a number, then it's a new map
        if !line.chars().next().unwrap().is_numeric() {
            // First, if there is an existing map, add it to the hashmap
            if let Some(map) = curr_map {
                seed_to_soil.push(map.clone());
            }

            let (from, to) = line.split("-to-").collect_tuple().unwrap();
            let to = to.strip_suffix(" map:").unwrap();
            curr_map = Some(Map {
                from: from.to_owned(),
                to: to.to_owned(),
                numbers: vec![],
            });
            continue;
        }

        // Otherwise, it's 3 numbers
        let (dest_range_start, source_range_start, range_length) = line
            .split(" ")
            .map(|n| n.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();

        curr_map.as_mut().unwrap().numbers.push(Change {
            dest_range_start,
            source_range_start,
            range_length,
        });
    }
    // Push the last map
    if let Some(map) = curr_map {
        seed_to_soil.push(map.clone());
    }

    // Go through each seed, and do the maps all the way through until the end.
    // We want to find the result of all the seeds that is the lowest at the
    // end.
    seeds
        .iter()
        .map(|seed| {
            let mut seed = *seed;

            for map in &seed_to_soil {
                println!("seed: {}", seed);
                // Find the number in the map that the seed is in
                let mut from = None;
                for change in &map.numbers {
                    if seed >= change.source_range_start
                        && seed < change.source_range_start + change.range_length
                    {
                        from = Some(change);
                        break;
                    }
                }
                // If from is none, then we didn't find a map. In that case, the
                // number stays the same.
                if from.is_none() {
                    continue;
                }

                let change = from.unwrap();

                // Find the index of the seed in the from range
                let index = seed - change.source_range_start;

                // Add the index to the to range
                seed = change.dest_range_start + index;
            }

            println!("");

            seed
        })
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> i64 {
    // seeds: 79 14 55 13

    // seed-to-soil map:
    // 50 98 2
    // 52 50 48

    // soil-to-fertilizer map:
    // 0 15 37
    // 37 52 2
    // 39 0 15

    // fertilizer-to-water map:
    // 49 53 8
    // 0 11 42
    // 42 0 7
    // 57 7 4

    // water-to-light map:
    // 88 18 7
    // 18 25 70

    // light-to-temperature map:
    // 45 77 23
    // 81 45 19
    // 68 64 13

    // temperature-to-humidity map:
    // 0 69 1
    // 1 0 69

    // Read the first line of the input, which is the seed numbers. Then, read
    // sections after that, which are the maps. Extract the type (from and to),
    // then all the numbers
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split("seeds: ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    // Change this into ranges so that it's a vec of ranges where the first
    // range is element 1 to element 1 + (size of element 2)
    let seeds: Vec<(i64, i64)> = seeds
        .into_iter()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let start = chunk.next().unwrap();
            let end = chunk.next().unwrap();

            // Create 100 ranges instead of 1 big one. This will cover
            // start..start+end in 100 ranges
            (0..100)
                .into_iter()
                .map(|i| (start + (end / 100 * i), start + (end / 100 * (i + 1))))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    #[derive(Clone)]
    struct Map {
        from: String,
        to: String,
        numbers: Vec<Change>,
    }

    //     Consider again the example seed-to-soil map:

    // 50 98 2
    // 52 50 48
    // The first line has a destination range start of 50, a source range start of
    // 98, and a range length of 2. This line means that the source range starts at
    // 98 and contains two values: 98 and 99. The destination range is the same
    // length, but it starts at 50, so its two values are 50 and 51. With this
    // information, you know that seed number 98 corresponds to soil number 50 and
    // that seed number 99 corresponds to soil number 51.
    #[derive(Clone)]
    struct Change {
        dest_range_start: i64,
        source_range_start: i64,
        range_length: i64,
    }

    let mut seed_to_soil = Vec::new();

    // Go through each line, and add the changes to the hashmap
    let mut curr_map: Option<Map> = None;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        // If the first char is not a number, then it's a new map
        if !line.chars().next().unwrap().is_numeric() {
            // First, if there is an existing map, add it to the hashmap
            if let Some(map) = curr_map {
                seed_to_soil.push(map.clone());
            }

            let (from, to) = line.split("-to-").collect_tuple().unwrap();
            let to = to.strip_suffix(" map:").unwrap();
            curr_map = Some(Map {
                from: from.to_owned(),
                to: to.to_owned(),
                numbers: vec![],
            });
            continue;
        }

        // Otherwise, it's 3 numbers
        let (dest_range_start, source_range_start, range_length) = line
            .split(" ")
            .map(|n| n.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();

        curr_map.as_mut().unwrap().numbers.push(Change {
            dest_range_start,
            source_range_start,
            range_length,
        });
    }
    // Push the last map
    if let Some(map) = curr_map {
        seed_to_soil.push(map.clone());
    }

    // Go through each seed, and do the maps all the way through until the end.
    // We want to find the result of all the seeds that is the lowest at the
    // end.
    seeds
        .into_par_iter()
        // .into_iter()
        .map(|m| {
            (m.0..=m.1)
                .map(|ref seed| {
                    let mut seed = *seed;

                    for map in &seed_to_soil {
                        // Find the number in the map that the seed is in
                        let mut from = None;
                        for change in &map.numbers {
                            if seed >= change.source_range_start
                                && seed < change.source_range_start + change.range_length
                            {
                                from = Some(change);
                                break;
                            }
                        }
                        // If from is none, then we didn't find a map. In that case, the
                        // number stays the same.
                        if from.is_none() {
                            continue;
                        }

                        let change = from.unwrap();

                        // Find the index of the seed in the from range
                        let index = seed - change.source_range_start;

                        // Add the index to the to range
                        seed = change.dest_range_start + index;
                    }

                    seed
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
