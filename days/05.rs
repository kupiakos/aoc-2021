use std::{cmp::Ordering, ops::RangeInclusive, str::FromStr};

use common::{get_input, Dots, PanicOnError};
use ndarray::{s, Array2};

#[derive(Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}
impl FromStr for Position {
    type Err = PanicOnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();
        let (x, y) = (x.parse()?, y.parse()?);
        Ok(Position { x, y })
    }
}

#[derive(Clone, Debug)]
struct Line {
    from: Position,
    to: Position,
}
enum LineOrientation {
    Horizontal,
    Vertical,
    Diagonal { rising: bool },
}

impl Line {
    fn normalize(mut self) -> Line {
        // left to right, then top to bottom
        let swap = match self.from.x.cmp(&self.to.x) {
            Ordering::Equal => self.from.y < self.to.y,
            Ordering::Less => false,
            Ordering::Greater => true,
        };
        if swap {
            std::mem::swap(&mut self.from, &mut self.to)
        };
        self
    }

    fn orientation(&self) -> LineOrientation {
        if self.from.x == self.to.x {
            LineOrientation::Vertical
        } else if self.from.y == self.to.y {
            LineOrientation::Horizontal
        } else {
            assert!(
                (self.from.x - self.to.x).abs() == (self.from.y - self.to.y).abs(),
                "line {:?} not 45 degrees",
                self
            );
            LineOrientation::Diagonal {
                rising: self.from.y > self.to.y,
            }
        }
    }
}
impl FromStr for Line {
    type Err = PanicOnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once(" -> ").unwrap();
        let (from, to) = (from.parse()?, to.parse()?);
        Ok(Line { from, to }.normalize())
    }
}

fn line_range(from: i32, to: i32) -> RangeInclusive<usize> {
    let (from, to) = (from as usize, to as usize);
    if from > to {
        to..=from
    } else {
        from..=to
    }
}

#[derive(Clone, Debug)]
struct Input {
    lines: Vec<Line>,
    max_size: usize,
}

fn part1(input: &Input) -> usize {
    // 1 MiB isn't that big
    let mut map: Array2<i32> = Array2::zeros((input.max_size, input.max_size));
    for line in &input.lines {
        match line.orientation() {
            LineOrientation::Horizontal => {
                let mut row = map.row_mut(line.from.y as usize);
                let mut line_content = row.slice_mut(s![line_range(line.from.x, line.to.x)]);
                line_content += 1;
            }
            LineOrientation::Vertical => {
                let mut col = map.column_mut(line.from.x as usize);
                let mut line_content = col.slice_mut(s![line_range(line.from.y, line.to.y)]);
                line_content += 1;
            }
            LineOrientation::Diagonal { .. } => (),
        }
    }
    map.into_iter().filter(|&x| x > 1).count()
}

fn part2(input: &Input) -> usize {
    let mut map: Array2<i32> = Array2::zeros((input.max_size, input.max_size));
    for line in &input.lines {
        match line.orientation() {
            LineOrientation::Horizontal => {
                let mut row = map.row_mut(line.from.y as usize);
                let mut line_content = row.slice_mut(s![line_range(line.from.x, line.to.x)]);
                line_content += 1;
            }
            LineOrientation::Vertical => {
                let mut col = map.column_mut(line.from.x as usize);
                let mut line_content = col.slice_mut(s![line_range(line.from.y, line.to.y)]);
                line_content += 1;
            }
            LineOrientation::Diagonal { rising } => {
                let mut y = line.from.y;
                for x in line.from.x..=line.to.x {
                    map[(y as usize, x as usize)] += 1;
                    y += if rising { -1 } else { 1 };
                }
            }
        }
    }
    println!("map:\n{}", Dots(&map));
    map.into_iter().filter(|&x| x > 1).count()
}

fn main() {
    let input = Input {
        lines: get_input!(parsed).collect(),
        max_size: 1000,
    };
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
