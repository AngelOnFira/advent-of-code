const DECK_SIZE: usize = 119315717514047;

#[aoc(day22, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut deck: [i32; DECK_SIZE] = [0; DECK_SIZE];

    for i in 0..DECK_SIZE {
        deck[i] = i as i32;
    }

    input.lines().for_each(|input| {
        let command: Vec<&str> = input.rsplitn(2, ' ').collect();
        println!("{:?}", command);

        match command[1] {
            "cut" => deck = cut(&deck, command[0].parse::<i32>().unwrap()),
            "deal with increment" => {
                deck = deal_with_increment(&deck, command[0].parse::<i32>().unwrap())
            }
            "deal into new" => deck = deal_new_stack(&deck),
            _ => unreachable!(),
        };

        // println!("{:?}", deck);
    });
    for i in 0..DECK_SIZE {
        if deck[i] == 2019 {
            return i as i32;
        }
    }

    12
}

pub fn cut(deck: &[i32], index: i32) -> [i32; DECK_SIZE] {
    let mut new_deck: [i32; DECK_SIZE] = [0; DECK_SIZE];
    let mut cut_index: i32 = index;

    if cut_index < 0 {
        cut_index += DECK_SIZE as i32;
    }

    for i in 0..DECK_SIZE {
        new_deck[i] = deck[(i + cut_index as usize) % DECK_SIZE];
    }

    new_deck
}

pub fn deal_with_increment(deck: &[i32], step: i32) -> [i32; DECK_SIZE] {
    let mut new_deck: [i32; DECK_SIZE] = [0; DECK_SIZE];

    for i in 0..DECK_SIZE {
        new_deck[(i * step as usize) % DECK_SIZE] = deck[i];
    }

    new_deck
}

pub fn deal_new_stack(deck: &[i32]) -> [i32; DECK_SIZE] {
    let mut new_deck: [i32; DECK_SIZE] = [0; DECK_SIZE];

    for i in 0..DECK_SIZE {
        new_deck[i] = deck[DECK_SIZE - i - 1];
    }

    new_deck
}

#[cfg(test)]
mod tests {
    use super::solve_part1 as part1;
    //use super::solve_part2 as part2;

    #[test]
    fn sample2211() {
        assert_eq!(
            part1("deal into new stack\ncut -2\ndeal with increment 7\ncut 8\ncut -4\ndeal with increment 7\ncut 3\ndeal with increment 9\ndeal with increment 3\ncut -1"),
            2
        );
    }

    #[test]
    fn sample2212() {
        assert_eq!(
            part1("cut 6\ndeal with increment 7\ndeal into new stack"),
            2
        );
    }

    #[test]
    fn sample2213() {
        assert_eq!(
            part1("deal with increment 7\ndeal with increment 9\ncut -2"),
            2
        );
    }
}
