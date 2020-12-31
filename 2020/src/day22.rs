use std::collections::HashSet;
use std::collections::VecDeque;

#[aoc_generator(day22)]
fn parse_input_day22(input: &str) -> Vec<Vec<u8>> {
    let mut player_decks: Vec<Vec<u8>> = Vec::new();
    let mut curr_deck: Vec<u8> = Vec::new();
    for line in input.lines() {
        if line == "" {
            player_decks.push(curr_deck.clone());
            curr_deck = Vec::new();
            continue;
        }
        if let Ok(x) = line.parse::<u8>() {
            curr_deck.push(x);
        }
    }
    player_decks.push(curr_deck.clone());

    player_decks
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &Vec<Vec<u8>>) -> u8 {
    let size = input.iter().map(|x| x.len()).sum();
    let mut deck1: VecDeque<u8> = input[0].clone().into_iter().collect();
    deck1.reserve(size);
    let mut deck2: VecDeque<u8> = input[1].clone().into_iter().collect();
    deck2.reserve(size);

    while deck1.len() > 0 && deck2.len() > 0 {
        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();

        if c1 > c2 {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
    }

    score(deck1) + score(deck2)
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &Vec<Vec<u8>>) -> u8 {
    let size = input.iter().map(|x| x.len()).sum();
    let mut deck1: VecDeque<u8> = input[0].clone().into_iter().collect();
    deck1.reserve(size);
    let mut deck2: VecDeque<u8> = input[1].clone().into_iter().collect();
    deck2.reserve(size);

    score(play(deck1, deck2).1)
}

pub fn play(mut deck1: VecDeque<u8>, mut deck2: VecDeque<u8>) -> (u8, VecDeque<u8>) {
    let mut seen = HashSet::new();
    while deck1.len() > 0 && deck2.len() > 0 {
        if seen.contains(&deck1) || seen.contains(&deck2) {
            return (1, deck1.clone());
        }

        seen.insert(deck1.clone());
        seen.insert(deck2.clone());
        let mut winner = 0;

        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();

        if c1 <= deck1.len() as u8 && c2 <= deck2.len() as u8 {
            winner = play(
                deck1.iter().take(c1 as usize).cloned().collect(),
                deck2.iter().take(c2 as usize).cloned().collect(),
            )
            .0;
        }

        if (c1 > c2 && winner == 0) || winner == 1 {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else if (c2 > c1 && winner == 0) || winner == 2 {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
    }

    if deck1.len() == 0 {
        return (2, deck2.clone());
    } else {
        return (1, deck1.clone());
    }
}

pub fn score(deck: VecDeque<u8>) -> u8 {
    deck.iter()
        .enumerate()
        .fold(0, |acc, (i, card)| acc + (deck.len() - i) as u8 * card)
}
