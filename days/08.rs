use std::{
    collections::HashMap,
    fmt::{self, Write},
    str::FromStr,
};

use common::{get_input, ArrayCollect, PanicOnError};

struct Input {
    entries: Vec<Entry>,
}

#[derive(Clone, Copy)]
struct Signal(u16);
impl Signal {
    fn num_set(self) -> u32 {
        self.0.count_ones()
    }

    fn num_common_with(self, other: Signal) -> u32 {
        (self.0 & other.0).count_ones()
    }

    fn segments_set(self) -> impl Iterator<Item = u8> {
        let mut x = self.0;
        std::iter::from_fn(move || {
            if x == 0 {
                return None;
            }
            let out = x.trailing_zeros() as u8;
            x &= x - 1;
            Some(out)
        })
    }
}

impl fmt::Debug for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in self.segments_set() {
            f.write_char(char::from_u32((x + b'a').into()).ok_or(fmt::Error)?)?;
        }
        Ok(())
    }
}

impl FromStr for Signal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Signal(s.as_bytes().iter().try_fold(
            0,
            |acc, &c| match c - b'a' {
                bit @ 0..=7 => Ok(acc | 1 << (bit as u16)),
                _c => Err(()),
            },
        )?))
    }
}

#[derive(Debug)]
struct Entry {
    signal_patterns: [Signal; 10],
    output_value: [Signal; 4],
}

fn parse_signals<const N: usize>(s: &str) -> Option<[Signal; N]> {
    // todo: no panic
    s.split(' ')
        .map(|x| x.parse::<Signal>().unwrap())
        .array_collect()
}

impl FromStr for Entry {
    type Err = PanicOnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signal_patterns, output_value) = s.split_once(" | ").unwrap();
        let signal_patterns = parse_signals(signal_patterns).unwrap();
        let output_value = parse_signals(output_value).unwrap();

        Ok(Entry {
            signal_patterns,
            output_value,
        })
    }
}

fn part1(input: &Input) -> usize {
    input
        .entries
        .iter()
        .flat_map(|i| i.output_value.iter().copied())
        .map(Signal::num_set)
        .filter(|&num_set| match num_set {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count()
}

fn part2(input: &Input) -> i64 {
    let mut out = 0;
    for entry in &input.entries {
        let mut counts = HashMap::new();
        for unique_signal in entry.signal_patterns {
            counts
                .entry(unique_signal.num_set() as usize)
                .or_insert(Signal(0))
                .0 |= unique_signal.0;
        }
        let one: Signal = counts[&2];
        let four: Signal = counts[&4];

        let mut value = 0;
        for o in entry.output_value {
            let n = match (o.num_set(), o.num_common_with(one), o.num_common_with(four)) {
                (2, 2, 2) => 1,
                (5, 1, 2) => 2,
                (5, 2, 3) => 3,
                (4, 2, 4) => 4,
                (5, 1, 3) => 5,
                (6, 1, 3) => 6,
                (3, 2, 2) => 7,
                (7, 2, 4) => 8,
                (6, 2, 4) => 9,
                (6, 2, 3) => 0,
                (_, _, _) => panic!("invalid signal {:?}", o),
            };
            value = value * 10 + n;
        }
        out += value;
    }
    out
}

fn main() {
    let input = Input {
        entries: get_input!(parsed).collect(),
    };
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
