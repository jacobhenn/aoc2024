use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 13;

pub fn solve<Part: AocPart>(input: &str) -> i64 {
    let mut tokens_used = 0;

    for machine in input.split("\n\n") {
        let lines = machine.lines().collect_vec();
        let a = v![
            ints::<i64>(lines[0]).nth(0).unwrap(),
            ints::<i64>(lines[0]).nth(1).unwrap()
        ];
        let b = v![
            ints::<i64>(lines[1]).nth(0).unwrap(),
            ints::<i64>(lines[1]).nth(1).unwrap()
        ];
        let mut prize = v![
            ints::<i64>(lines[2]).nth(0).unwrap(),
            ints::<i64>(lines[2]).nth(1).unwrap()
        ];

        if Part::is_two() {
            prize += v![10000000000000, 10000000000000];
        }

        log!("a: {a}, b: {b}, prize: {prize}");

        let det = a[0] * b[1] - b[0] * a[1];
        let deti = Ratio::new(1, det);

        log!("deti: {deti}");

        let res = v![
            Ratio::from_integer(prize[0] * b[1] - prize[1] * b[0]),
            Ratio::from_integer(-(prize[0] * a[1]) + prize[1] * a[0])
        ] * deti;

        log!("res: {res}");

        if res[0].is_integer() && res[1].is_integer() {
            log!("possible");
            tokens_used += res[0].numer() * 3 + res[1].numer();
        }
    }

    return tokens_used;
}

aoc_tests! {
    inputs {
        e0 = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
    }

    part::One {
        ea0: e0 => 480,
        ra: @input => 34393,
    }

    part::Two {
        rb: @input => 83551068361379,
    }
}
