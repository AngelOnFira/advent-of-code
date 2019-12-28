use specs::prelude::*;
use specs::DispatcherBuilder;
use specs::{Builder, World, WorldExt};
use specs::{Component, VecStorage};
use specs::{Join, System};
use std::error::Error;

use regex::Regex;

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}

struct CalculateGravity;

impl<'a> System<'a> for CalculateGravity {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x;
            pos.y += vel.y;
            pos.z += vel.z;
        }
    }
}

#[aoc_generator(day12)]
fn parse_input_day1(input: &str) -> Vec<Vec<i32>> {
    let re = Regex::new(r"<x=(?P<x>.*), y=(?P<y>.*), z=(?P<z>.*)").unwrap();
    println!("tesatest");

    input
        .lines()
        .map(|moon| {
            let data = re.captures(moon).unwrap();
            vec![
                data["x"].parse::<i32>().unwrap(),
                data["y"].parse::<i32>().unwrap(),
                data["z"].parse::<i32>().unwrap(),
            ]
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Vec<i32>]) -> i32 {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    let mut moons = Vec::new();

    for moon in input {
        moons.push(
            // world
            //     .create_entity()
            //     .with(Position {
            //         x: moon[0],
            //         y: moon[1],
            //         z: moon[2],
            //     })
            //     .build(),
            vec![moon[0], moon[1], moon[2]],
        );
    }

    // let mut dispatcher = DispatcherBuilder::new()
    //     .with(CalculateGravity, "hello_world", &[])
    //     .with(UpdatePos, "update_pos", &["hello_world"])
    //     .build();

    // let mut calculate_gravity = CalculateGravity;
    // for i in 0..1000 {
    //     calculate_gravity.run_now(&world);
    //     world.maintain();
    // }

    for j in 0..10 {
        let mut new_vels: Vec<&[i32]> = Vec::new();

        for moon in moons.clone() {
            let mut vel = [0; 3];
            for moon2 in moons.clone() {
                if moon != moon2 {
                    for i in 0..3 {
                        if moon[i] < moon2[i] {
                            vel[i] = 1;
                        } else {
                            vel[i] = -1;
                        }
                    }
                }
            }
        }

        println!("After {} steps", j);
        for moon_index in 0..moons.len() {
            println!(
                "pos=<x= {}, y= {}, z= {}>, vel=<x= {}, y= {}, z= {}>",
                moons[moon_index][0],
                moons[moon_index][1],
                moons[moon_index][2],
                new_vels[moon_index][0],
                new_vels[moon_index][1],
                new_vels[moon_index][2],
            );

            for i in 0..3 {
                moons[moon_index][i] += new_vels[moon_index][i];
            }
        }
    }

    12
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
