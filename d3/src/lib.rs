use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 3;

fn go(s: &str) -> i32 {
    let re = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    re.captures_iter(s)
        .map(|c| {
            c.extract::<2>()
                .1
                .iter()
                .map(|s| s.parse::<i32>().unwrap())
                .product::<i32>()
        })
        .sum()
}

pub fn solve<P: AocPart>(input: &str) -> i32 {
    match P::part() {
        Part::One => go(input),
        Part::Two => {
            let mut s = input;
            let mut res = 0;

            loop {
                let dont = s.find("don't()").unwrap_or(s.len());
                res += go(&s[..dont]);
                s = &s[dont..];
                let Some(doi) = s.find("do()") else {
                    break;
                };
                s = &s[doi..];
            }

            res
        }
    }
}

aoc_tests! {
    inputs {
        e0 = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        e1 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    }

    part::One {
        ea0: e0 => 161,
        ra: @input => 174336360,
    }

    part::Two {
        eb0: e1 => 48,
        rb: @input => 88802350,
    }
}
