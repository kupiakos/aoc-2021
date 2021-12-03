// This almost certainly would've been easier with string manipulation
// instead of the bit twiddling I did.

use common::get_input;

fn read_bins<'a>(iter: impl Iterator<Item = &'a str>) -> Vec<u32> {
    iter.filter(|x| !x.trim().is_empty())
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect()
}

// todo: refactor part1 to use part2 fn's?
fn part1(iter: impl Iterator<Item = u32>) -> u64 {
    let values: Vec<u32> = iter.collect();
    let len = values.len() as u32;
    // counts of set bits

    let mut counts = [0u32; 32];
    let mut max_width = 0;
    for mut value in values {
        while value > 0 {
            // let nonzero = NonZeroU32::new(value).unwrap();
            // lowest set bit
            let bit = value.trailing_zeros();
            counts[bit as usize] += 1;
            // clear that bit
            value &= value - 1;
            max_width = bit.max(max_width);
        }
    }

    let mut gamma: u64 = 0;
    let mut epsilon: u64 = 0;
    for count in counts.into_iter().take((max_width + 1) as usize).rev() {
        gamma <<= 1;
        epsilon <<= 1;
        if count * 2 > len {
            gamma |= 1;
        } else if count * 2 < len {
            epsilon |= 1;
        } else {
            panic!("no bit is most common")
        }
    }
    assert_eq!(epsilon, (!gamma) & ((1 << (max_width + 1)) - 1));
    gamma * epsilon
}

fn max_width(iter: impl Iterator<Item = u32>) -> Option<u32> {
    iter.map(|x| 32 - x.leading_zeros()).max()
}

// part 2 uses a different way to count set bits
fn bit_is_mostly_set(iter: impl Iterator<Item = u32>, bit: u32) -> bool {
    let mut set = 0;
    let mut len = 0;
    for i in iter {
        set += (i & (1 << bit) != 0) as u32;
        len += 1;
    }
    set * 2 >= len
}


fn find_rating(mut values: Vec<u32>, max_width: u32, flip: bool) -> Option<u32> {
    let mut mask = 0;
    for bit in (0..max_width).into_iter().rev() {
        let set = bit_is_mostly_set(values.iter().copied(), bit) ^ flip;
        mask |= (set as u32) << bit;
        values.retain(|&x| (x >> bit) << bit == mask);
        if values.len() == 1 {
            return Some(values[0]);
        }
    }
    None
}

fn part2(iter: impl Iterator<Item = u32>) -> u64 {
    let values: Vec<u32> = iter.collect();
    let max_width = max_width(values.iter().copied()).unwrap();
    let oxygen = find_rating(values.clone(), max_width, false).unwrap() as u64;
    let co2 = find_rating(values.clone(), max_width, true).unwrap() as u64;
    oxygen * co2
}

fn main() {
    let data = read_bins(get_input!(lines));
    println!("Part 1: {}", part1(data.iter().copied()));
    println!("Part 2: {}", part2(data.into_iter()));
}
