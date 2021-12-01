use common::get_input;
use itertools::Itertools;

fn part1(iter: impl Iterator<Item = u32>) -> usize {
    iter.tuple_windows().filter(|(x, y)| x < y).count()
}

fn part2(iter: impl Iterator<Item = u32>) -> usize {
    part1(iter.tuple_windows().map(|(x, y, z)| x + y + z))
}

fn main() {
    println!("Part 1: {}", part1(get_input!(parsed)));
    println!("Part 2: {}", part2(get_input!(parsed)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA.into_iter()), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA.into_iter()), 5);
    }
}
