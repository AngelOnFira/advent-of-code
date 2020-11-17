use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day12)]
fn parse_input_day1(input: &str) -> Vec<Vec<i32>> {
    let re = Regex::new(r"<x=(?P<x>.*), y=(?P<y>.*), z=(?P<z>.*)>").unwrap();

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
    println!("{:?}", input);
    let mut moons = Vec::new();
    let mut state_map = HashMap::new();

    for moon in input {
        moons.push(vec![moon[0], moon[1], moon[2]]);
    }

    let mut velocities: Vec<Vec<i32>> = Vec::new();

    for _ in 0..(moons.len()) {
        velocities.push(vec![0, 0, 0]);
    }

    let mut j = 0;

    loop {
        if j % 1_000_000 == 0 {
            println!("{}", j)
        }
        for (moon_idx, moon) in moons.clone().iter().enumerate() {
            for moon2 in moons.clone() {
                if *moon != moon2 {
                    for i in 0..3 {
                        if moon[i] < moon2[i] {
                            velocities[moon_idx][i] += 1;
                        } else if moon[i] > moon2[i] {
                            velocities[moon_idx][i] -= 1;
                        }
                    }
                }
            }
        }

        // println!("After {} steps", j + 1);
        for moon_index in 0..moons.len() {
            for i in 0..3 {
                moons[moon_index][i] += velocities[moon_index][i];
            }

            // println!(
            //     "pos=<x= {}, y= {}, z= {}>, vel=<x= {}, y= {}, z= {}>",
            //     moons[moon_index][0],
            //     moons[moon_index][1],
            //     moons[moon_index][2],
            //     velocities[moon_index][0],
            //     velocities[moon_index][1],
            //     velocities[moon_index][2],
            // );
        }
        // println!(
        //     "Total energy is {}",
        //     moons
        //         .iter()
        //         .zip(velocities.iter())
        //         .map(|(pos, vel)| pos.iter().map(|x| x.abs()).sum::<i32>()
        //             * vel.iter().map(|x| x.abs()).sum::<i32>())
        //         .sum::<i32>()
        // );

        if let Some(old) = state_map.insert(moons.clone(), j) {
            return j - old;
        }
        j += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::parse_input_day1 as gen1;
    use super::solve_part1 as part1;
    // use super::solve_part2 as part2;

    #[test]
    fn sample911() {
        println!(
            "{:?}",
            gen1("<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>")
        );
    }

    #[test]
    fn sample1211() {
        assert_eq!(
            12,
            part1(&vec![
                vec![-1, 0, 2],
                vec![2, -10, -7],
                vec![4, -8, 8],
                vec![3, 5, -1]
            ])
        );
    }

    #[test]
    fn sample1212() {
        assert_eq!(
            12,
            part1(&vec![
                vec![-8, -10, 0],
                vec![5, 5, 10],
                vec![2, -7, 3],
                vec![9, -8, -3],
            ])
        );
    }
}
