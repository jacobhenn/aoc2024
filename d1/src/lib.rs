use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 1;

pub fn solve<P: AocPart>(input: &str) -> i32 {
    let lines = input
        .lines()
        .map(|line| ints::<i32>(line).collect_vec())
        .filter(|line| line.len() >= 2)
        .collect_vec();

    println!("{lines:?}");

    let mut lefts = lines.iter().map(|line| line[0]).collect_vec();
    let mut rights = lines.iter().map(|line| line[1]).collect_vec();

    match P::part() {
        Part::One => {
            lefts.sort();
            rights.sort();

            iter::zip(lefts, rights).map(|(l, r)| (l - r).abs()).sum()
        }
        Part::Two => lefts
            .iter()
            .map(|l| l * rights.iter().filter(|r| l == *r).count() as i32)
            .sum(),
    }
}

aoc_tests! {
    inputs {
        e0 = "3   4
4   3
2   5
1   3
3   9
3   3",
    }

    part::One {
        ea0: e0 => 11,
        ra: @input => 1651298,
    }

    part::Two {
        eb0: e0 => 31,
        rb: @input => 21306195,
    }
}
