use std::collections::{HashMap, HashSet};

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut boards: Vec<HashSet<(i32, i32)>> = Vec::new();
    let mut cursor_x: i32;
    let mut cursor_y: i32;
    let mut min_distance: i32 = -1;
    let mut delta: (i32, i32);

    for line in input.lines() {
        cursor_x = 0;
        cursor_y = 0;
        let mut board = HashSet::new();
        for command in line.split(",") {
            match command.get(..1).unwrap() {
                "U" => delta = (0, 1),
                "D" => delta = (0, -1),
                "L" => delta = (-1, 0),
                "R" => delta = (1, 0),
                _ => unreachable!(),
            }
            for _ in 0..command.get(1..).unwrap().parse::<i32>().unwrap() {
                cursor_x += delta.0;
                cursor_y += delta.1;
                board.insert((cursor_x, cursor_y));
            }
        }
        boards.push(board);
    }

    for (cursor_x, cursor_y) in boards[0].intersection(&boards[1]) {
        if cursor_x.abs() + cursor_y.abs() < min_distance || min_distance == -1 {
            min_distance = cursor_x.abs() + cursor_y.abs();
        }
    }

    min_distance
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut steps_maps: Vec<HashMap<(i32, i32), i32>> = Vec::new();
    let mut boards: Vec<HashSet<(i32, i32)>> = Vec::new();
    let mut cursor_x: i32;
    let mut cursor_y: i32;
    let mut min_distance: i32 = -1;
    let mut delta: (i32, i32);

    for line in input.lines() {
        cursor_x = 0;
        cursor_y = 0;
        let mut steps = 0;
        let mut board = HashSet::new();
        let mut steps_map = HashMap::new();
        for command in line.split(",") {
            match command.get(..1).unwrap() {
                "U" => delta = (0, 1),
                "D" => delta = (0, -1),
                "L" => delta = (-1, 0),
                "R" => delta = (1, 0),
                _ => unreachable!(),
            }
            for _ in 0..command.get(1..).unwrap().parse::<i32>().unwrap() {
                steps += 1;
                cursor_x += delta.0;
                cursor_y += delta.1;
                board.insert((cursor_x, cursor_y));
                steps_map.insert((cursor_x, cursor_y), steps);
            }
        }
        boards.push(board);
        steps_maps.push(steps_map);
    }

    for (cursor_x, cursor_y) in boards[0].intersection(&boards[1]) {
        if steps_maps[0].get(&(*cursor_x, *cursor_y)).unwrap()
            + steps_maps[1].get(&(*cursor_x, *cursor_y)).unwrap()
            < min_distance
            || min_distance == -1
        {
            min_distance = steps_maps[0].get(&(*cursor_x, *cursor_y)).unwrap()
                + steps_maps[1].get(&(*cursor_x, *cursor_y)).unwrap();
        }
    }

    min_distance
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    use super::solve_part2 as part2;

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

    #[test]
    fn sample314() {
        assert_eq!(part1("R1,U4,R1,D4\nU5"), -1);
    }

    #[test]
    fn sample321() {
        assert_eq!(
            part2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
            610
        );
    }

    #[test]
    fn sample322() {
        assert_eq!(
            part2(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}
