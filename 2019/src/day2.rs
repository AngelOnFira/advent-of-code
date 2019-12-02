#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut number = input
        .split(",")
        .map(|input| input.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    number[1] = 12;
    number[2] = 2;

    // If I knew more unsafe Rust this would work
    //
    // for chunk in number[..].chunks(4) {
    //     let opcode = chunk[0];
    //     let read_loc1 = chunk[1] as usize;
    //     let read_loc2 = chunk[2] as usize;
    //     let out_loc = chunk[3] as usize;

    //     println!("{}, {}, {}, {}", opcode, read_loc1, read_loc2, out_loc);
    //     unsafe {
    //         match opcode {
    //             1 => number[out_loc] = number[read_loc1] + number[read_loc2],
    //             2 => number[out_loc] = number[read_loc1] * number[read_loc2],
    //             99 => return number[0],
    //             _ => unreachable!()
    //         }
    //     }
    // }

    let mut index = 0;

    while index + 4 < number.len() {
        let opcode = number[index];
        let read_loc1 = number[index + 1] as usize;
        let read_loc2 = number[index + 2] as usize;
        let out_loc = number[index + 3] as usize;

        match opcode {
            1 => number[out_loc] = number[read_loc1] + number[read_loc2],
            2 => number[out_loc] = number[read_loc1] * number[read_loc2],
            99 => return number[0],
            _ => unreachable!(),
        }

        index += 4;
    }

    number[0]
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;

    #[test]
    fn sample1() {
        assert_eq!(part1("1,0,0,0,99\n"), 2);
    }

    #[test]
    fn sample2() {
        assert_eq!(part1("1,1,1,4,99,5,6,0,99"), 30);
    }

    #[test]
    fn sample3() {
        assert_eq!(part1("1,9,10,3,2,3,11,0,99,30,40,50"), 3500);
    }
}
