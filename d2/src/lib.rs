use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 2;

pub fn solve<Part: AocPart>(input: &str) -> usize {
    println!("lines: {}", input.lines().count());

    input
        .trim()
        .lines()
        .filter(|line| {
            let ns = ints::<i32>(line).collect_vec();

            let res = ns.iter().tuple_windows().all(|(a, b)| (a - b).abs() <= 3)
                && (ns.iter().tuple_windows().all(|(a, b)| a < b)
                    || ns.iter().tuple_windows().all(|(a, b)| a > b));

            if !res && Part::is_two() {
                for i in 0..ns.len() {
                    let it = ns.iter().take(i).chain(ns.iter().skip(i + 1));

                    if it.clone().tuple_windows().all(|(a, b)| (a - b).abs() <= 3)
                        && (it.clone().tuple_windows().all(|(a, b)| a < b)
                            || it.tuple_windows().all(|(a, b)| a > b))
                    {
                        return true;
                    }
                }
            }

            println!("{line}: {res}");

            res
        })
        .count()
}

aoc_tests! {
    inputs {
        e0 = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    }

    part::One {
        ea0: e0 => 2,
        ra: @input => 510,
    }

    part::Two {
        eb0: e0 => 4,
        rb: @input => 553,
    }
}
