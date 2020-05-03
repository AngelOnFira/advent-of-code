use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Object<'a> {
    parent: &'a str,
    children: Vec<&'a str>,
}

#[aoc(day6, part1)]
pub fn solve_part1<'a>(input: &'a str) -> i32 {
    let mut child_map: HashMap<&str, Object> = HashMap::new();
    input.lines().for_each(|input| {
        let mut orbit_info = input.split(")").collect::<Vec<&str>>();
        let mut parent_name = orbit_info[0];
        let child_name = orbit_info[1];

        if !child_map.contains_key(parent_name) {
            child_map.insert(
                parent_name,
                Object {
                    parent: "unknown",
                    children: Vec::new(),
                },
            );
        }

        if !child_map.contains_key(child_name) {
            child_map.insert(
                child_name,
                Object {
                    parent: parent_name,
                    children: Vec::new(),
                },
            );
        } else {
            let this_child = child_map.get_mut(child_name).unwrap();
            this_child.parent = parent_name;
        }

        let this_parent = child_map.get_mut(parent_name).unwrap();
        this_parent.children.push(child_name);
    });

    println!("{}", child_map.len());
    println!("{:?}", child_map);

    let mut top = "";

    'outer: for key in child_map.keys() {
        if child_map.get(key).unwrap().parent == "unknown" {
            top = key;
            break 'outer;
        }
    }

    count_recurse(&child_map, top, 0)
}

fn count_recurse(child_map: &HashMap<&str, Object>, name: &str, depth: i32) -> i32 {
    let mut total = 0;
    for child in child_map.get(name).unwrap().children.clone() {
        total += count_recurse(child_map, child, depth + 1);
    }

    depth + total
}

#[aoc(day6, part2)]
pub fn solve_part2<'a>(input: &'a str) -> i32 {
    let mut child_map: HashMap<&str, Object> = HashMap::new();
    input.lines().for_each(|input| {
        let mut orbit_info = input.split(")").collect::<Vec<&str>>();
        let mut parent_name = orbit_info[0];
        let child_name = orbit_info[1];

        if !child_map.contains_key(parent_name) {
            child_map.insert(
                parent_name,
                Object {
                    parent: "unknown",
                    children: Vec::new(),
                },
            );
        }

        if !child_map.contains_key(child_name) {
            child_map.insert(
                child_name,
                Object {
                    parent: parent_name,
                    children: Vec::new(),
                },
            );
        } else {
            let this_child = child_map.get_mut(child_name).unwrap();
            this_child.parent = parent_name;
        }

        let this_parent = child_map.get_mut(parent_name).unwrap();
        this_parent.children.push(child_name);
    });

    println!("{}", child_map.len());
    println!("{:?}", child_map);

    let mut you_root = Vec::new();
    let mut san_root = Vec::new();

    let mut pointer = "YOU";

    while pointer != "unknown" {
        pointer = child_map.get(pointer).unwrap().parent;
        you_root.push(pointer);
    }

    pointer = "SAN";

    while pointer != "unknown" {
        pointer = child_map.get(pointer).unwrap().parent;
        san_root.push(pointer);
    }

    for i in 0..you_root.len() {
        for j in 0..san_root.len() {
            if you_root[i] == san_root[j] {
                return (i + j) as i32;
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    use super::solve_part2 as part2;

    #[test]
    fn sample611() {
        assert_eq!(
            part1("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"),
            42
        );
    }

    #[test]
    fn sample621() {
        assert_eq!(
            part2("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"),
            4
        );
    }
}
