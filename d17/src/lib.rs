#![feature(array_chunks)]

use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 17;

// adv X: A = A >> cX          irreversible
// bxl X: B ^= X               partially invertible
// bst X: B = cX % 8           irreversible
// jnz X: if A != 0 { jump X } ????
// bxc X: B ^= C               partially invertible
// out X: print (cX % 8)       invertible
// bdv X: B = A >> cX          irreversible
// cdv X: C = A >> cX          irreversible

/* ----- SCRATCHPAD: ------

my input:

    Register A: 61156655
    Register B: 0
    Register C: 0
    Program: 2,4,1,5,7,5,4,3,1,6,0,3,5,5,3,0

    translated program:

    2,4 => b = a % 8
    1,5 => b ^= 0b101
    7,5 => c = a >> b
    4,3 => b ^= c
    1,6 => b ^= 0b110
    0,3 => a >>= 3
    5,5 => output (b % 8)
    3,0 => if a != 0 { continue }

example quine:

    Register A: 117440
    Register B: 0
    Register C: 0
    Program: 0,3,5,4,3,0

    translated program:

    0,3 => a >>= 3
    5,4 => output (a % 8)
    3,0 => if a != 0 { continue }

*/

const INSTR_NAMES: &[&str] = &["adv", "bxl", "bst", "jnz", "bxc", "out", "bdv", "cdv"];

fn combo(arg: usize, regs: &[usize; 3]) -> usize {
    match arg {
        0..=3 => arg,
        4..=6 => regs[arg as usize - 4],
        7 => panic!("reserved"),
        other => panic!("other: {other}"),
    }
}

pub fn simul_r_fast<'a>(
    mut regs: [usize; 3],
    prog: &'a [usize],
) -> impl Iterator<Item = usize> + Clone + use<'a> {
    iter::from_fn(move || {
        if regs[0] == 0 {
            return None;
        }

        regs[1] = regs[0] % 8;
        regs[1] ^= 0b101;
        regs[2] = regs[0] >> regs[1];
        regs[1] ^= regs[2];
        regs[1] ^= 0b110;
        regs[0] >>= 3;
        return Some(regs[1] % 8);
    })
}

pub fn simul<'a>(
    mut regs: [usize; 3],
    prog: &'a [usize],
) -> impl Iterator<Item = usize> + Clone + use<'a> {
    let mut instr_ptr = 0;

    iter::from_fn(move || {
        while instr_ptr < prog.len() {
            let _g = log::enter();

            let opcode = prog[instr_ptr];
            let arg = prog[instr_ptr + 1];

            // log!("{} {arg}", INSTR_NAMES[opcode]);

            instr_ptr += 2;

            match opcode {
                0 => regs[0] = regs[0] >> combo(arg, &regs),
                1 => regs[1] = regs[1] ^ arg,
                2 => regs[1] = combo(arg, &regs) % 8,
                3 => {
                    if regs[0] != 0 {
                        instr_ptr = arg as usize;
                    }
                }
                4 => regs[1] = regs[1] ^ regs[2],
                5 => return Some(combo(arg, &regs) % 8),
                6 => regs[1] = regs[0] >> combo(arg, &regs),
                7 => regs[2] = regs[0] >> combo(arg, &regs),
                other => panic!("other instr: {other}"),
            }
        }

        None
    })
}

const BIT_WINDOW: u32 = 10;

pub fn solve<Part: AocPart>(input: &str) -> String {
    let mut ints = ints::<usize>(input);

    let regs: [usize; 3] = ints.fill_array().unwrap();
    let prog: Vec<usize> = ints.collect();
    log!("prog: {prog:?}");

    if Part::is_one() {
        return simul_r_fast(regs, &prog).into_iter().join(",");
    }

    let desired_output = &prog[..];
    // let desired_output = &[3, 5, 5, 3, 0][..];

    let mut quine_a = 0;

    let mut next_bits_to_test = vec![0; prog.len()];

    let mut n = 0;
    'outer: while n < prog.len() {
        log!("looking at desired_outputram part {n}");
        log!("quine_a: {quine_a:o}");
        let _g = log::enter();

        for test_bits in next_bits_to_test[n]..(2.pow(BIT_WINDOW) - 1) {
            next_bits_to_test[n] = test_bits + 1;

            // log!("testing bits {test_bits:b}");
            let mask = !(2.pow(BIT_WINDOW) - 1);
            let test_a = (quine_a & mask) ^ test_bits;
            // log!("test_a = {test_a:b}");
            let test_regs = [test_a, regs[1], regs[2]];
            let res = simul_r_fast(test_regs, &prog);
            // log!("res: {res:?}");
            if Iterator::eq(
                res.take(n + 1),
                desired_output[desired_output.len() - n - 1..].iter().copied(),
            ) {
                quine_a = test_a;
                log!(
                    "break! res: {} for 0o{quine_a:o} {quine_a}",
                    simul_r_fast(test_regs, &prog).join(",")
                );
                quine_a = quine_a << 3;
                n += 1;
                continue 'outer;
            }
        }

        log!("nothing found; backtracking");
        quine_a = quine_a >> 3;
        next_bits_to_test[n] = 0;
        n -= 1;
    }

    loop {
        quine_a = quine_a - 1;
        if Iterator::eq(
            simul_r_fast([quine_a, regs[1], regs[2]], &prog),
            desired_output.iter().copied(),
        ) {
            println!("{quine_a} 0o{quine_a:o}");
        }
    }

    log!("quine_a = {quine_a}, next_bits_to_test: {next_bits_to_test:?}");

    assert_eq!(desired_output, simul_r_fast([quine_a, regs[1], regs[2]], &prog).collect_vec());

    quine_a.to_string()
}

aoc_tests! {
    inputs {
        e0 = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",

        e1 = "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4",

        e2 = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",

        e3 = "Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
    }

    part::One {
        ea0: e0 => "4,6,3,5,6,3,5,2,1,0",
        ea1: e1 => "0,1,2",
        ea2: e2 => "4,2,5,6,7,7,7,7,3,1,0",
        ea3: e3 => "0,3,5,4,3,0",
        ra: @input => "7,3,5,7,5,7,4,3,0",
    }

    part::Two {
        eb3: e3 => "117440",
        rb: @input => "",
    }
}
