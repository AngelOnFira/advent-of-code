use std::{collections::HashMap, ops::Index};

use regex::Regex;

pub struct Instruction {}

// #[aoc_generator(day21)]
// pub fn input_generator(input: &str) -> Vec<Instruction> {}

#[aoc(day21, part1)]
pub fn solve_part1(_: &str) -> i32 {
    let mut players_pos = [2, 7];
    let mut players_scores = [0, 0];

    let mut curr_roll = 1;

    let mut die_roll = 0;

    'outer: loop {
        for player in 0..2 {
            let mut dist = 0;
            for _ in 0..3 {
                die_roll += 1;
                dist += curr_roll;
                curr_roll += 1;
                if curr_roll > 100 {
                    curr_roll = 1;
                }
            }

            players_pos[player] += dist;

            while players_pos[player] > 10 {
                players_pos[player] -= 10;
            }

            players_scores[player] += players_pos[player];

            if players_scores[player] >= 1000 {
                break 'outer;
            }
        }
    }

    die_roll * players_scores.iter().min().unwrap()
}

struct Player {
    pos: i16,
    score: i8,
}

#[aoc(day21, part2)]
pub fn solve_part2(_: &str) -> i128 {
    // A second compartment opens, this time labeled Dirac dice. Out of it falls
    // a single three-sided die.

    // As you experiment with the die, you feel a little strange. An
    // informational brochure in the compartment explains that this is a quantum
    // die: when you roll it, the universe splits into multiple copies, one copy
    // for each possible outcome of the die. In this case, rolling the die
    // always splits the universe into three copies: one where the outcome of
    // the roll was 1, one where it was 2, and one where it was 3.

    // The game is played the same as before, although to prevent things from
    // getting too far out of hand, the game now ends when either player's score
    // reaches at least 21.

    // Using the same starting positions as in the example above, player 1 wins
    // in 444356092776315 universes, while player 2 merely wins in
    // 341960390180808 universes.

    // Using your given starting positions, determine every possible outcome.
    // Find the player that wins in more universes; in how many universes does
    // that player win?

    let mut players_pos = [4, 8];
    // let mut players_pos = [2, 7];
    let mut players_wins = [0, 0];

    let score_to_reach = 21;

    // p1p, p2p, p1s, p2s, turn
    let mut old_lookup_map: HashMap<(i128, i128, i128, i128, i128), i128> = HashMap::new();

    for i in 0..3 {
        old_lookup_map.insert(
            (players_pos[0] + i, players_pos[0], players_pos[0], 0, 0),
            1,
        );
    }

    while old_lookup_map.len() > 0 {
        let mut new_lookup_map: HashMap<(i128, i128, i128, i128, i128), i128> = HashMap::new();
        for (state, val) in old_lookup_map.iter() {
            for i in 0..3 {
                let new_state;
                if state.4 == 0 {
                    let mut new_pos = state.1 + i;
                    while new_pos > 10 {
                        new_pos -= 10;
                    }
                    new_state = (state.0, new_pos, state.2, state.3 + new_pos, 1);
                } else {
                    let mut new_pos = state.0 + i;
                    while new_pos > 10 {
                        new_pos -= 10;
                    }
                    new_state = (new_pos, state.1, state.2 + new_pos, state.3, 0);
                }
                
                if new_state.2 >= score_to_reach {
                    players_wins[0] += val;
                } else {
                    *new_lookup_map.entry(new_state).or_insert(0) += val;
                }

                if new_state.3 >= score_to_reach {
                    players_wins[1] += val;
                } else {
                    *new_lookup_map.entry(new_state).or_insert(0) += val;
                }
            }
        }
        old_lookup_map = new_lookup_map;
    }

    *players_wins.iter().max().unwrap()
}
