
pub struct Instruction {}

#[aoc_generator(day17)]
pub fn input_generator(_: &str) -> i32 {
    //target area: x=85..145, y=-163..-108
    1
}

#[aoc(day17, part1)]
pub fn solve_part1(_: &i32) -> i32 {
    let x1 = 85;
    let x2 = 145;
    let y1 = -163;
    let y2 = -108;

    let mut max_y = 0;

    for x in -100..1000 {
        for y in -100..1000 {
            // dbg!((x, y));
            let mut curr_pos = (0, 0);
            let mut curr_vel = (x, y);
            let mut this_max_y = 0;
            for _ in 0..1000 {
                curr_pos.0 += curr_vel.0;
                curr_pos.1 += curr_vel.1;

                // Due to drag, the probe's x velocity changes by 1 toward the
                // value 0; that is, it decreases by 1 if it is greater than 0,
                // increases by 1 if it is less than 0, or does not change if it
                // is already 0.

                if curr_vel.0 < 0 {
                    curr_vel.0 += 1;
                } else if curr_vel.0 > 0 {
                    curr_vel.0 -= 1;
                }

                curr_vel.1 -= 1;

                if curr_pos.1 > this_max_y {
                    // dbg!(curr_pos);
                    this_max_y = curr_pos.1;
                }

                // Check if curr_pos is in the target area
                if curr_pos.0 >= x1 && curr_pos.0 <= x2 && curr_pos.1 >= y1 && curr_pos.1 <= y2 {
                    // dbg!(this_max_y);
                    if this_max_y > max_y {
                        max_y = this_max_y;
                        break;
                    }
                }
            }
        }
    }

    max_y
}

#[aoc(day17, part2)]
pub fn solve_part2(_: &i32) -> i32 {
    let x1 = 85;
    let x2 = 145;
    let y1 = -163;
    let y2 = -108;

    let mut inside_count = 0;

    for x in 0..1000 {
        for y in -1000..1000 {
            let mut curr_pos = (0, 0);
            let mut curr_vel = (x, y);
            loop {
                curr_pos.0 += curr_vel.0;
                curr_pos.1 += curr_vel.1;

                if curr_vel.0 < 0 {
                    curr_vel.0 += 1;
                } else if curr_vel.0 > 0 {
                    curr_vel.0 -= 1;
                }

                curr_vel.1 -= 1;

                // Check if curr_pos is in the target area
                if curr_pos.0 >= x1 && curr_pos.0 <= x2 && curr_pos.1 >= y1 && curr_pos.1 <= y2 {
                    inside_count += 1;
                    break;
                }

                // check if we've passed the target area
                if curr_pos.0 > x2 || curr_pos.1 > y2 {
                    break;
                }
            }
        }
    }

    inside_count
}
