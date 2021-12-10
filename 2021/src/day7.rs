#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    // 16,1,2,0,4,2,7,1,2,14
    input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    (*input.iter().min().unwrap()..*input.iter().max().unwrap())
        .into_iter()
        .map(|curr| input.iter().map(|x| (curr - x).abs()).sum())
        .min()
        .unwrap()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    (*input.iter().min().unwrap()..*input.iter().max().unwrap())
        .into_iter()
        .map(|curr| {
            input
                .iter()
                .map(|x| {
                    let dist = (curr - x).abs();
                    dist * (dist + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}
