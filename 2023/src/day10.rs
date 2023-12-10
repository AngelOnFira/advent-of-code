use std::collections::HashMap;

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> i64 {
    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), c);
        }
    }

    struct Dirs {
        north: bool,
        south: bool,
        east: bool,
        west: bool,
    }

    // In each position, find the directions that the pipe can go.
    let mut dirs: HashMap<(i32, i32), Dirs> = HashMap::new();
    for (pos, c) in &map {
        let mut d = Dirs {
            north: false,
            south: false,
            east: false,
            west: false,
        };
        match c {
            '|' => {
                d.north = true;
                d.south = true;
            }
            '-' => {
                d.east = true;
                d.west = true;
            }
            'L' => {
                d.north = true;
                d.east = true;
            }
            'J' => {
                d.north = true;
                d.west = true;
            }
            '7' => {
                d.south = true;
                d.west = true;
            }
            'F' => {
                d.south = true;
                d.east = true;
            }
            '.' => {}
            'S' => {}
            _ => panic!("Unknown char {}", c),
        }
        dirs.insert(*pos, d);
    }

    // Use a BFS to find the furthest we can get from the start before getting
    // to a point that has already been visited.
    type DistanceFromStart = i32;
    let mut visited: HashMap<(i32, i32), DistanceFromStart> = HashMap::new();
    let mut queue: Vec<((i32, i32), i32)> = Vec::new();

    // The starting place is the S in the grid
    let start: &(i32, i32) = map.iter().find(|(_, c)| **c == 'S').unwrap().0;

    // Add the neighbours of the start to the queue if they have a pipe that
    // points to the first. Add them at distance 1
    if let Some(d) = dirs.get(&(start.0 + 1, start.1)) {
        if d.west {
            queue.push(((start.0 + 1, start.1), 1));
        }
    }

    if let Some(d) = dirs.get(&(start.0 - 1, start.1)) {
        if d.east {
            queue.push(((start.0 - 1, start.1), 1));
        }
    }

    if let Some(d) = dirs.get(&(start.0, start.1 + 1)) {
        if d.north {
            queue.push(((start.0, start.1 + 1), 1));
        }
    }

    if let Some(d) = dirs.get(&(start.0, start.1 - 1)) {
        if d.south {
            queue.push(((start.0, start.1 - 1), 1));
        }
    }

    while !queue.is_empty() {
        let (pos, dist) = queue.remove(0);
        if visited.contains_key(&pos) {
            continue;
        }
        visited.insert(pos, dist + 1);

        let d = dirs.get(&pos).unwrap();
        if d.north {
            queue.push(((pos.0, pos.1 - 1), dist + 1));
        }
        if d.south {
            queue.push(((pos.0, pos.1 + 1), dist + 1));
        }
        if d.east {
            queue.push(((pos.0 + 1, pos.1), dist + 1));
        }
        if d.west {
            queue.push(((pos.0 - 1, pos.1), dist + 1));
        }
    }

    // Find the furthest distance from the start
    let mut max_dist = 0;
    for (_, dist) in visited {
        if dist > max_dist {
            max_dist = dist;
        }
    }
    max_dist as i64
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> i64 {
    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), c);
        }
    }

    struct Dirs {
        north: bool,
        south: bool,
        east: bool,
        west: bool,
    }

    // In each position, find the directions that the pipe can go.
    let mut dirs: HashMap<(i32, i32), Dirs> = HashMap::new();
    for (pos, c) in &map {
        let mut d = Dirs {
            north: false,
            south: false,
            east: false,
            west: false,
        };
        match c {
            '|' => {
                d.north = true;
                d.south = true;
            }
            '-' => {
                d.east = true;
                d.west = true;
            }
            'L' => {
                d.north = true;
                d.east = true;
            }
            'J' => {
                d.north = true;
                d.west = true;
            }
            '7' => {
                d.south = true;
                d.west = true;
            }
            'F' => {
                d.south = true;
                d.east = true;
            }
            '.' => {}
            'S' => {}
            _ => panic!("Unknown char {}", c),
        }
        dirs.insert(*pos, d);
    }

    // Use a BFS to find the furthest we can get from the start before getting
    // to a point that has already been visited.
    type DistanceFromStart = i32;
    let mut visited: HashMap<(i32, i32), DistanceFromStart> = HashMap::new();
    let mut queue: Vec<((i32, i32), i32)> = Vec::new();

    // The starting place is the S in the grid
    let start: &(i32, i32) = map.iter().find(|(_, c)| **c == 'S').unwrap().0;

    // Add the neighbours of the start to the queue if they have a pipe that
    // points to the first. Add them at distance 1
    if let Some(d) = dirs.get(&(start.0 + 1, start.1)) {
        if d.west {
            queue.push(((start.0 + 1, start.1), 1));
        }
    }

    if let Some(d) = dirs.get(&(start.0 - 1, start.1)) {
        if d.east {
            queue.push(((start.0 - 1, start.1), 1));
        }
    }

    if let Some(d) = dirs.get(&(start.0, start.1 + 1)) {
        if d.north {
            queue.push(((start.0, start.1 + 1), 1));
        }
    }

    if let Some(d) = dirs.get(&(start.0, start.1 - 1)) {
        if d.south {
            queue.push(((start.0, start.1 - 1), 1));
        }
    }

    while !queue.is_empty() {
        let (pos, dist) = queue.remove(0);
        if visited.contains_key(&pos) {
            continue;
        }
        visited.insert(pos, dist + 1);

        let d = dirs.get(&pos).unwrap();
        if d.north {
            queue.push(((pos.0, pos.1 - 1), dist + 1));
        }
        if d.south {
            queue.push(((pos.0, pos.1 + 1), dist + 1));
        }
        if d.east {
            queue.push(((pos.0 + 1, pos.1), dist + 1));
        }
        if d.west {
            queue.push(((pos.0 - 1, pos.1), dist + 1));
        }
    }

    // We're going to create a larger

    // Flood fill the map to find the number of tiles that are not reachable by
    // the outside world to inside the loop. If it needs to traverse any of the
    // spaces in visited, then it is not reachable.
    // Start at -100 and go to +200 on each axis

    let mut flood_fill: HashMap<(i32, i32), bool> = HashMap::new();
    // for x in -100..200 {
    //     for y in -100..200 {
    //         flood_fill.insert((x, y), false);
    //     }
    // }

    // Mark the start as reachable
    flood_fill.insert((-100, -100), true);

    // Start looking at the neighbours of the start until we've done the whole
    // map
    let mut queue: Vec<(i32, i32)> = Vec::new();
    queue.push((-100, -100));

    // dbg!(&visited);

    while !queue.is_empty() {
        let pos: (i32, i32) = queue.remove(0);
        // dbg!(&pos);
        // dbg!(&queue);
        flood_fill.insert(pos, true);
        // If we go out of the bounds of the map, then don't add it to the queue
        // Check if up down left and right are in the visited list. If not, we
        // can check them.
        for offset in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let neighbour = (pos.0 + offset.0, pos.1 + offset.1);
            // dbg!(&neighbour);
            // dbg!(!visited.contains_key(&neighbour));
            // dbg!(!flood_fill.contains_key(&neighbour));
            // dbg!(neighbour.0 >= -100);
            // dbg!(neighbour.0 <= 200);
            // dbg!(neighbour.1 >= -100);
            // dbg!(neighbour.1 <= 200);
            if !visited.contains_key(&neighbour)
                && !flood_fill.contains_key(&neighbour)
                && !queue.contains(&neighbour)
                && neighbour.0 >= -100
                && neighbour.0 <= 200
                && neighbour.1 >= -100
                && neighbour.1 <= 200
            {
                queue.push(neighbour);
            }
        }
    }

    // Print the floodfill map
    for y in -100..200 {
        for x in -100..200 {
            if flood_fill.contains_key(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    // // Print the original map
    for y in -100..200 {
        for x in -100..200 {
            if map.contains_key(&(x, y)) {
                print!("{}", map.get(&(x, y)).unwrap());
            } else {
                print!(".");
            }
        }
        println!();
    }

    // Print the visited map. If something was visited, draw its symbol on the
    // map
    for y in -100..200 {
        for x in -100..200 {
            if visited.contains_key(&(x, y)) {
                print!("{}", map.get(&(x, y)).unwrap());
            } else {
                print!(".");
            }
        }
        println!();
    }


    // Count the number of tiles in the floodfill that are not reachable
    let mut count = 0;
    for y in -100..200 {
        for x in -100..200 {
            if !flood_fill.contains_key(&(x, y)) {
                count += 1;
            }
        }
    }

    // Subtract the number of tiles in visited
    count -= visited.len();
    count as i64
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
