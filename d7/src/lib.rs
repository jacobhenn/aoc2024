use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 7;

// parse!(s => "Card " (_n) ": " (have) " | " @last (winning));

fn can_be_built<Part: AocPart>(target: i128, acc: i128, rest: &[i128]) -> bool {
    if rest.is_empty() {
        return acc == target;
    }

    can_be_built::<Part>(target, acc + rest[0], &rest[1..])
        || can_be_built::<Part>(target, acc * rest[0], &rest[1..])
        || (Part::is_two()
            && can_be_built::<Part>(
                target,
                (acc * 10.pow(rest[0].to_string().len() as u32)) + rest[0],
                &rest[1..],
            ))
}

fn solve<Part: AocPart>(input: &str) -> i128 {
    input
        .trim()
        .lines()
        .map(|line| {
            let (target, ns) = line.split_once(':').unwrap();
            let target = target.trim().parse::<i128>().unwrap();
            let ns = ns
                .trim()
                .split_ascii_whitespace()
                .map(|n| n.parse::<i128>().unwrap())
                .collect_vec();
            (target, ns)
        })
        .filter(|(target, ns)| can_be_built::<Part>(*target, ns[0], &ns[1..]))
        .map(|(target, ns)| target)
        .sum::<i128>()
}

aoc_tests! {
    inputs {
        e0 = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
",
    }

    part::One {
        ea0: e0 => 3749,
        ra: @input => 7885693428401,
    }

    part::Two {
        eb0: e0 => 11387,
        rb: @input => 348360680516005,
    }
}
