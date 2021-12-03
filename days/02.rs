use std::str::FromStr;

use common::get_input;

fn part1(iter: impl Iterator<Item = Movement>) -> i32 {
    let mut pos = 0;
    let mut depth = 0;
    use Movement::*;
    for movement in iter {
        match movement {
            Forward(a) => pos += a,
            Down(a) => depth += a,
            Up(a) => depth -= a,
        }
    }
    pos * depth
}

fn part2(iter: impl Iterator<Item = Movement>) -> i32 {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    use Movement::*;
    for movement in iter {
        match movement {
            Forward(a) => {
                pos += a;
                depth += aim * a;
            },
            Down(a) => aim += a,
            Up(a) => aim -= a,
        }
    }
    pos * depth
}

enum Movement {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split(" ");
        let direction = i.next().unwrap();
        let amount: i32 = i.next().unwrap().parse().unwrap();
        Ok(match direction {
            "forward" => Movement::Forward(amount),
            "down" => Movement::Down(amount),
            "up" => Movement::Up(amount),
            x => panic!("invalid direction {}", x),
        })
    }
}

fn main() {
    println!("Part 1: {}", part1(get_input!(parsed)));
    println!("Part 2: {}", part2(get_input!(parsed)));
}
