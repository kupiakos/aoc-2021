use std::{collections::VecDeque, str::FromStr};

use common::{get_input, PanicOnError};

struct Input {
    counts: [i64; 9],
}

impl FromStr for Input {
    type Err = PanicOnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut counts = [0; 9];
        for i in s.split(',') {
            counts[i.parse::<usize>()?] += 1;
        }
        Ok(Input { counts })
    }
}

fn run_sim(state: &mut VecDeque<i64>, days: usize) {
    assert_eq!(state.len(), 9);
    for _ in 0..days {
        let atzero = state.pop_front().expect("empty state");
        state[6] += atzero;
        state.push_back(atzero);
    }
}

fn part1(input: &Input) -> i64 {
    let mut state: VecDeque<i64> = input.counts.iter().copied().collect();
    state.resize(9, 0);
    run_sim(&mut state, 80);
    state.into_iter().sum()
}

fn part2(input: &Input) -> i64 {
    let mut state: VecDeque<i64> = input.counts.iter().copied().collect();
    state.resize(9, 0);
    run_sim(&mut state, 256);
    state.into_iter().sum()
}

fn main() {
    let input = get_input!().parse().unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
