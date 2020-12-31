use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;

// #[aoc(day23, part1)]
// pub fn solve_part1(input: &str) -> i32 {
//     // let mut cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
//     let mut cups = vec![2, 1, 9, 3, 4, 7, 8, 6, 5];

//     for i in 10..=1_000_000 {
//         cups.push(i);
//     }

//     let mut lookup: HashMap<usize, usize> = HashMap::new();

//     let mut last;
//     for cup_chunk in cups.chunks(2) {
//         lookup.insert(cup_chunk[0], cup_chunk[1]);
//         last = cup_chunk[1];
//     }
//     lookup.insert(last, cups[1]);

//     let mut current = cups[0];

//     for glob in 0..10_000_000 {
//         if glob % 1_000_000 == 0 {
//             println!("{}", glob);
//         }
//         // dbg!(cups.clone());
//         // println!("{:?}", cups.clone());
//         // println!("{:?}", current);

//         let mut take: Vec<usize> = Vec::new();
//         // let mut removal: Vec<i32 = Vec::new();
//         let loc = cups.iter().position(|&x| x == current).unwrap();
//         let future = cups[(loc + 4) % cups.len()];

//         for _ in 0..3 {
//             take.push(*lookup.get(&current).unwrap());
//             lookup.insert(current, *lookup.get(&current).unwrap());
//         }

//         let mut num = current;
//         loop {
//             num -= 1;

//             if num == 0 {
//                 num = 1_000_000;
//             }
//             if !take.contains(&num) {
//                 let new_loc = cups.iter().position(|&x| x == num).unwrap();

//                 for (index, element) in take.iter().enumerate() {
//                     let inloc = (new_loc + 1 + index);
//                     cups.insert(inloc, *element);
//                 }

//                 current = cups[(cups.iter().position(|&x| x == future).unwrap()) % cups.len()];
//                 break;
//             }
//         }
//     }
//     for i in 0..8 {
//         print!(
//             "{}",
//             cups[(cups.iter().position(|&x| x == 1).unwrap() + 1 + i) % cups.len()]
//         );
//     }
//     let pos = cups.iter().position(|&x| x == 1).unwrap();
//     cups[pos + 1] * cups[pos + 2]
// }

#[aoc(day23, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    // let mut cups = vec![2, 1, 9, 3, 4, 7, 8, 6, 5];

    // for i in 10..=1_000_000 {
    //     cups.push(i);
    // }

    let mut lookup: HashMap<usize, usize> = HashMap::new();

    let mut last = 0;
    lookup = cups.windows(2).map(|x| (x[0], x[1])).collect();
    lookup.insert(cups[cups.len() - 1], cups[0]);

    let mut current = cups[0];

    for glob in 0..3 {
        let mut oof = 1;
        for _ in 0..9 {
            let new = lookup.get(&oof).unwrap().clone();
            print!("{}", new);
            oof = new;
        }
        println!("");
        // dbg!(lookup.clone());
        // for glob in 0..10_000_000 {
        if glob % 1_000_000 == 0 {
            println!("{}", glob);
        }

        let mut take: Vec<usize> = Vec::new();
        // let loc = cups.iter().position(|&x| x == current).unwrap();
        for _ in 0..3 {
            let next = *lookup.get(&current).unwrap();
            take.push(next);
            lookup.insert(current, *lookup.get(&next).unwrap());
        }
        dbg!(take.clone(), current);
        let next_current = *lookup.get(&current).unwrap();

        let mut num = current;
        loop {
            num -= 1;

            if num == 0 {
                num = 9;
                // num = 1_000_000;
            }
            if !take.contains(&num) {
                let mut last = num;
                let after = lookup.get(&num).unwrap().clone();
                for element in take.iter() {
                    lookup.insert(last, *element);
                    last = *element;
                }
                lookup.insert(last, after);

                current = next_current;
                break;
            }
        }
    }
    let first = lookup.get(&1).unwrap();
    let second = lookup.get(&first).unwrap();

    (first * second) as i32
}
