use std::{collections::HashSet, hash::Hash};

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Instruction {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
    on: bool,
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    // on x=-11..33,y=-6..40,z=-16..37
    // on x=-44..10,y=-24..30,z=-24..22
    // on x=-34..15,y=-21..27,z=-33..11
    // on x=-42..12,y=-43..9,z=1..48
    // off x=-31..21,y=-11..42,z=-4..49
    // on x=0..44,y=-13..37,z=-30..14
    // off x=-41..12,y=-32..17,z=-7..43
    // on x=-21..27,y=-16..30,z=-33..15
    // on x=-28..24,y=-12..42,z=-6..45
    // on x=-15..30,y=-32..14,z=-48..5

    let re = Regex::new(r"(\w) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let on = &caps[1] == "n";
            let x_min = caps[2].parse::<i32>().unwrap();
            let x_max = caps[3].parse::<i32>().unwrap();
            let y_min = caps[4].parse::<i32>().unwrap();
            let y_max = caps[5].parse::<i32>().unwrap();
            let z_min = caps[6].parse::<i32>().unwrap();
            let z_max = caps[7].parse::<i32>().unwrap();

            Instruction {
                x_min,
                x_max,
                y_min,
                y_max,
                z_min,
                z_max,
                on,
            }
        })
        .collect()
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &Vec<Instruction>) -> i32 {
    // dbg!(input);
    let mut on_point_count = 0;
    for x in -50..50 {
        for y in -50..50 {
            for z in -50..50 {
                // See if this point is on or off
                let mut on = false;
                for instruction in input {
                    if x >= instruction.x_min
                        && x <= instruction.x_max
                        && y >= instruction.y_min
                        && y <= instruction.y_max
                        && z >= instruction.z_min
                        && z <= instruction.z_max
                    {
                        if instruction.on {
                            on = true;
                        } else {
                            on = false;
                        }
                    }
                }

                if on {
                    on_point_count += 1;
                }
            }
        }
    }
    on_point_count
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &Vec<Instruction>) -> i32 {
    // This time, calculate the total number of points that are on at the end of
    // the instruction list

    let mut on_point_count = 0;

    let mut collision_map: Vec<HashSet<Instruction>> = Vec::new();

    // Go through each instruction and find every other one it collides with,
    // then add all of those instructions to the collision map

    let mut to_process_queue: Vec<Instruction> = input.clone();

    let mut added_instructions: Vec<Instruction> = Vec::new();

    for instruction in input {
        // If this current cube overlaps with something in added_instructions,
        // then break both down into 8 smaller cubes. If the new cube is on,
        // then keep the intersecting cube. Otherwise, discard it.

        for check_placed in added_instructions.clone() {
            if instruction.x_min <= check_placed.x_max
                && instruction.x_max >= check_placed.x_min
                && instruction.y_min <= check_placed.y_max
                && instruction.y_max >= check_placed.y_min
                && instruction.z_min <= check_placed.z_max
                && instruction.z_max >= check_placed.z_min
            {
                // They intersected, so break this instruction down into 8

                let mut new_instructions: Vec<Instruction> = Vec::new();

                let 
            
        }
    }

    dbg!(&collision_map);

    // for instruction in input {
    //     if instruction.on {
    //         on_point_count += (instruction.x_max - instruction.x_min)
    //             * (instruction.y_max - instruction.y_min)
    //             * (instruction.z_max - instruction.z_min);
    //     }
    // }

    3
}
