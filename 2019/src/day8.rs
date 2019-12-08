#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut nums = input
        .chars()
        .map(|input| input.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut lowest = 0;

    nums[..].chunks(25 * 6).fold(-1, |min, layer| {
        let mut count: [i32; 3] = [0; 3];

        layer.into_iter().for_each(|num| {
            count[*num as usize] += 1;
        });

        if count[0] < min || min == -1 {
            lowest = count[1] * count[2];
            return count[0];
        }

        min
    });
    lowest
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut nums = input
        .chars()
        .map(|input| input.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    for y in 0..6 {
        for x in 0..25 {
            'outer: for layer in 0..100 {
                let pixel = nums[(layer * 6 * 25) + (y * 25 + x)];
                if pixel != 2 {
                    match pixel {
                        0 => print!(" "),
                        1 => print!("{}", pixel),
                        _ => unreachable!(),
                    };
                    break 'outer;
                }
            }
        }
        println!("");
    }

    0
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    //use super::solve_part2 as part2;

    #[test]
    fn sample811() {
        assert_eq!(part1("11222000000000111111"), 6);
    }
}
