#![feature(array_chunks)]
#![feature(let_chains)]

use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 19;

fn count_possible_arrangements(design: &str, patterns: &[&str]) -> usize {
    let mut possible_pats: HashMap<&str, usize> = [("", 1)].into_iter().collect();

    for design_c in design.chars() {
        let mut new_possible_pats: HashMap<&str, usize> = HashMap::new();

        for (pat, &branches) in &possible_pats {
            match pat.chars().next() {
                Some(c) if c == design_c => {
                    *new_possible_pats.entry(&pat[1..]).or_insert(0) += branches;
                }
                Some(_) => (),
                None => {
                    for possible_pat in patterns
                        .iter()
                        .filter(|pat| pat.chars().next() == Some(design_c))
                    {
                        *new_possible_pats.entry(&possible_pat[1..]).or_insert(0) += branches;
                    }
                }
            }
        }

        if new_possible_pats.is_empty() {
            return 0;
        }

        possible_pats.clear();
        mem::swap(&mut possible_pats, &mut new_possible_pats);
    }

    possible_pats.get("").copied().unwrap_or(0)
}

pub fn solve<Part: AocPart>(input: &str) -> usize {
    let (patterns_s, designs_s) = input.trim().split_once("\n\n").unwrap();

    let patterns: Vec<&str> = patterns_s.split(", ").collect();

    let designs: Vec<&str> = designs_s.lines().collect();

    log!("patterns: {patterns:?}");
    log!("designs: {designs:?}");

    if Part::is_one() {
        designs
            .iter()
            .filter(|design| count_possible_arrangements(design, &patterns) > 0)
            .count()
    } else {
        designs
            .iter()
            .map(|design| count_possible_arrangements(design, &patterns))
            .sum()
    }
}

aoc_tests! {
    inputs {
        e0 = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
    }

    part::One {
        ea0: e0 => 6,
        ra: @input => 360,
    }

    part::Two {
        eb0: e0 => 16,
        rb: @input => 577474410989846,
    }
}
