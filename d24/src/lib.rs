use aocutil::prelude::*;

use fs::OpenOptions;
use std::io::Write;

pub const YEAR: usize = 2024;

pub const DAY: usize = 24;

pub const VIZ_DIR: &str = "/home/jacob/code/rust/aoc/aoc2024/d24/viz/aoc2024-d24-viz";

pub fn solve_b(gates: Vec<(&str, &str, &str, &str)>) -> usize {
    for &(lhs, op, rhs, out) in &gates {
        let mut lhs_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(format!("{VIZ_DIR}/{lhs}.md"))
            .unwrap();
        writeln!(lhs_file, "[[{lhs}{op}{rhs}]]").unwrap();

        let mut rhs_file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(format!("{VIZ_DIR}/{rhs}.md"))
            .unwrap();
        writeln!(rhs_file, "[[{lhs}{op}{rhs}]]").unwrap();

        fs::write(
            format!("{VIZ_DIR}/{lhs}{op}{rhs}.md"),
            format!("#{op}\n[[{out}]]"),
        )
        .unwrap();
    }

    todo!()
}

pub fn solve<Part: AocPart>(input: &str) -> usize {
    let (initial_values_s, gates_s) = input.split_once("\n\n").unwrap();

    let mut values: HashMap<&str, usize> = initial_values_s
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            (l, r.parse::<usize>().unwrap())
        })
        .collect();

    let gates: Vec<(&str, &str, &str, &str)> = gates_s
        .lines()
        .map(|line| {
            parse!(line => (lhs) " " (op) " " (rhs) " -> " @last (out));
            (lhs, op, rhs, out)
        })
        .collect();

    if Part::is_two() {
        return solve_b(gates);
    }

    loop {
        let mut did_something = false;

        for &(lhs, op, rhs, out) in &gates {
            if values.contains_key(out) {
                continue;
            }

            did_something = true;

            let Some(lhs_val) = values.get(lhs) else {
                continue;
            };
            let Some(rhs_val) = values.get(rhs) else {
                continue;
            };
            let out_val = match op {
                "AND" => lhs_val & rhs_val,
                "XOR" => lhs_val ^ rhs_val,
                "OR" => lhs_val | rhs_val,
                other => panic!("{other}"),
            };
            values.insert(out, out_val);
        }

        if !did_something {
            break;
        }
    }

    let digits = values
        .iter()
        .filter(|(k, v)| k.starts_with('z'))
        .sorted_by_key(|(k, v)| k[1..].parse::<usize>().unwrap())
        .map(|(k, v)| v)
        .collect_vec();

    digits.iter().zip(0..).map(|(&&l, r)| l * 2.pow(r)).sum()
}

aoc_tests! {
    inputs {
        e0 = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02",

        e1 = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj",

    }

    part::One {
        ea0: e0 => 4,
        ea1: e1 => 2024,
        ra: @input => 0,
    }

    part::Two {
        eb0: e0 => 0,
        rb: @input => 0,
    }
}
