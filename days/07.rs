use std::{collections::BTreeMap, str::FromStr};

use common::{get_input, PanicOnError};
use itertools::Itertools;

struct Input {
    // BTreeMap because originally I cared about key order.
    // Still real fast to iterate though.
    counts: BTreeMap<i32, i32>,
}

impl FromStr for Input {
    type Err = PanicOnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // is there a good iterator-y way to do this?
        let mut counts = BTreeMap::new();
        for i in s.trim_end().split(',') {
            *counts.entry(i.parse()?).or_default() += 1;
        }
        Ok(Input { counts })
    }
}

fn calc_cost_to(items: &BTreeMap<i32, i32>, dest: i32) -> i32 {
    items
        .iter()
        .map(|(pos, count)| (pos - dest).abs() * count)
        .sum()
}

fn calc_pricey_cost_to(items: &BTreeMap<i32, i32>, dest: i32) -> i32 {
    items
        .iter()
        .map(|(pos, count)| {
            let distance = (pos - dest).abs();
            // Sum of the series [1, n]: n(n+1)/2
            (distance + 1) * distance / 2 * count
        })
        .sum()
}

fn part1(input: &Input) -> i32 {
    // there are more efficient ways to do this for sure
    let (min, max) = input.counts.keys().minmax().into_option().unwrap();
    (*min..=*max)
        .map(|pos| calc_cost_to(&input.counts, pos))
        .min()
        .unwrap()
}

fn part2(input: &Input) -> i32 {
    let (min, max) = input.counts.keys().minmax().into_option().unwrap();
    (*min..=*max)
        .map(|pos| calc_pricey_cost_to(&input.counts, pos))
        .min()
        .unwrap()
}

fn main() {
    let input = get_input!().parse().unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
