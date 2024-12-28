#![feature(array_windows)]

use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 22;

const PRUNE: i128 = 16777216;

pub fn solve<Part: AocPart>(input: &str) -> i128 {
    let nums = ints::<i128>(input);

    let mut total = 0;

    let mut seqs = Vec::new();

    for mut num in nums {
        let mut seq = Vec::new();

        seq.push(num);
        for _ in 0..2000 {
            // for _ in 0..9 {
            num ^= num * 64;
            num %= PRUNE;
            num ^= num / 32;
            num %= PRUNE;
            num ^= num * 2048;
            num %= PRUNE;
            seq.push(num);
        }

        seqs.push(seq);

        total += num;
    }

    if Part::is_one() {
        return total;
    }

    let mut diff_lookups: Vec<HashMap<&[i128], i128>> = Vec::new();

    for seq in seqs {
        let mut diff_lookup: HashMap<&[i128], i128> = HashMap::new();

        for (a, b, c, d, e) in seq.iter().map(|n| n % 10).tuple_windows() {
            let window = [a, b, c, d, e];
            let diffs = window
                .array_windows::<2>()
                .map(|[a, b]| b - a)
                .collect_vec();

            // log!("diffs: {diffs:?} for payout {e}");

            if !diff_lookup.contains_key(&diffs[..]) {
                diff_lookup.insert(diffs.leak(), window[4]);
            }
        }

        diff_lookups.push(diff_lookup);
    }

    let all_diffs: HashSet<&&[i128]> = diff_lookups
        .iter()
        .map(|diffs| diffs.keys())
        .flatten()
        .collect();

    let mut max_total = 0;

    for &diff in all_diffs {
        let total: i128 = diff_lookups
            .iter()
            .map(|diffs| diffs.get(diff).unwrap_or(&0))
            .sum();

        max_total = cmp::max(total, max_total);

        // if total == max_total {
        //     log!("found new max at diff {diff:?}");
        // }
    }

    max_total
}

aoc_tests! {
    inputs {
        e0 = "1
10
100
2024",

        e1 = "1 2 3 2024",
    }

    part::One {
        ea0: e0 => 0,
        ra: @input => 0,
    }

    part::Two {
        eb1: e1 => 23,
        eb2: "123" => 6,
        rb: @input => 0,
    }
}
