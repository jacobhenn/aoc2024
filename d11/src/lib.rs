use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 11;

pub fn solve<Part: AocPart>(input: &str, iterations: usize) -> usize {
    let mut stones: HashMap<i128, usize> = input
        .split_ascii_whitespace()
        .map(|word| (word.parse::<i128>().unwrap(), 1))
        .collect();

    let mut new_stones = HashMap::new();

    for n in 0..iterations {
        for (&stone, &count) in &stones {
            if stone == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else if stone.to_string().len() % 2 == 0 {
                let s = stone.to_string();
                let mid = s.len() / 2;
                *new_stones
                    .entry(s[..mid].parse::<i128>().unwrap())
                    .or_insert(0) += count;
                *new_stones
                    .entry(s[mid..].parse::<i128>().unwrap())
                    .or_insert(0) += count;
            } else {
                *new_stones.entry(stone * 2024).or_insert(0) += count;
            }
        }

        mem::swap(&mut stones, &mut new_stones);
        new_stones.clear();

        println!("{n} {}", stones.len());
    }

    stones.values().sum()
}

aoc_tests! {
    inputs {
        e0 = "0 1 10 99 999",
        e1 = "125 17",
    }

    part::One {
        ea0: e0, 1 => 7,
        ea11: e1, 6 => 22,
        ea12: e1, 25 => 55312,
        ra: @input 25 => 200446,
    }

    part::Two {
        rb: @input 75 => 238317474993392,
    }
}
