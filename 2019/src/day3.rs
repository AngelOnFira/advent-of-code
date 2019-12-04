use std::cmp::{max, min};
use std::collections::HashMap;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut board = HashMap::new();
    let mut cursor_x = 0;
    let mut cursor_y = 0;
    let mut min_distance: i32 = -1;

    for line in input.lines() {
        cursor_x = 0;
        cursor_y = 0;
        for command in line.split(",") {
            match command.get(..1).unwrap() {
                "U" => add_to_hashmap(
                    &mut cursor_x,
                    &mut cursor_y,
                    0,
                    command.get(1..).unwrap().parse::<i32>().unwrap(),
                    &mut board,
                    &mut min_distance,
                ),
                "D" => add_to_hashmap(
                    &mut cursor_x,
                    &mut cursor_y,
                    0,
                    command.get(1..).unwrap().parse::<i32>().unwrap() * -1,
                    &mut board,
                    &mut min_distance,
                ),
                "L" => add_to_hashmap(
                    &mut cursor_x,
                    &mut cursor_y,
                    command.get(1..).unwrap().parse::<i32>().unwrap() * -1,
                    0,
                    &mut board,
                    &mut min_distance,
                ),
                "R" => add_to_hashmap(
                    &mut cursor_x,
                    &mut cursor_y,
                    command.get(1..).unwrap().parse::<i32>().unwrap(),
                    0,
                    &mut board,
                    &mut min_distance,
                ),
                _ => unreachable!(),
            }
        }
    }

    println!("{:?}", board.keys());

    min_distance
}

fn add_to_hashmap(
    cursor_x: &mut i32,
    cursor_y: &mut i32,
    delta_x: i32,
    delta_y: i32,
    board: &mut HashMap<(i32, i32), i32>,
    min_distance: &mut i32,
) {
    if *cursor_x != *cursor_x + delta_x {
        for x in min(*cursor_x, *cursor_x + delta_x)..=max(*cursor_x, *cursor_x + delta_x) {
            let key = (x, *cursor_y);
            if board.contains_key(&key) {
                println!("{} {}", x.abs(), (*cursor_y).abs());
                if x + *cursor_y < *min_distance || *min_distance == -1 {
                    *min_distance = x.abs() + (*cursor_y).abs();
                }
            } else {
                if !(x == 0 && *cursor_y == 0) {
                    board.insert(key, 1);
                }
            }
        }
    }
    if *cursor_y != *cursor_y + delta_y {
        for y in min(*cursor_y, *cursor_y + delta_y)..=max(*cursor_y, *cursor_y + delta_y) {
            let key = (*cursor_x, y);
            if board.contains_key(&key) {
                println!("{} {}", (*cursor_x).abs(), y.abs());
                if y + *cursor_x < *min_distance || *min_distance == -1 {
                    *min_distance = (*cursor_x).abs() + y.abs();
                }
            } else {
                if !(y == 0 && *cursor_x == 0) {
                    board.insert(key, 1);
                }
            }
        }
    }

    *cursor_x += delta_x;
    *cursor_y += delta_y;
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;

    #[test]
    fn sample311() {
        assert_eq!(
            part1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            159
        );
    }

    #[test]
    fn sample312() {
        assert_eq!(part1("R8,U5,L5,D3\nU7,R6,D4,L4"), 6);
    }

    #[test]
    fn sample313() {
        assert_eq!(
            part1(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }
}
