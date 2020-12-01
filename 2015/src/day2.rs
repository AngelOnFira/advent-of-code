use regex::Regex;

#[aoc_generator(day2)]
fn parse_input_day2(input: &str) -> Vec<Vec<i32>> {
    let re = Regex::new(r"(?P<x>.*)x(?P<y>.*)x(?P<z>.*)").unwrap();

    input
        .lines()
        .map(|dimension| {
            let data = re.captures(dimension).unwrap();
            vec![
                data["x"].parse::<i32>().unwrap(),
                data["y"].parse::<i32>().unwrap(),
                data["z"].parse::<i32>().unwrap(),
            ]
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Vec<i32>]) -> i32 {
    let mut wrapping_paper: i32 = 0;
    for box_size in input {
        let wrapping_paper_area: Vec<i32> = (0..3)
            .map(|x| box_size[x] * box_size[(x + 1) % 3] * 2)
            .collect();
        println!("{:?}", wrapping_paper_area);
        println!("{:?}", wrapping_paper_area.iter().min().unwrap());
        wrapping_paper += wrapping_paper_area.iter().sum::<i32>()
            + (wrapping_paper_area.iter().min().unwrap() / 2);
    }
    wrapping_paper
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Vec<i32>]) -> i32 {
    let mut ribbon_length: i32 = 0;
    for box_size in input {
        let mut sorted: Vec<i32> = box_size.clone();
        sorted.sort();

        ribbon_length += sorted.iter().product::<i32>() + sorted[0] * 2 + sorted[1] * 2;
    }
    ribbon_length
}

#[cfg(test)]
mod tests {
    // use super::solve_part1 as part1;
    // use super::solve_part2 as part2;
}
