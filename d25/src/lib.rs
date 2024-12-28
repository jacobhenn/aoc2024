use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 25;

pub fn solve<Part: AocPart>(input: &str) -> usize {
    let shapes = input
        .trim()
        .split("\n\n")
        .map(|par| Grid::from_lines(par))
        .collect::<Vec<_>>();

    let locks = shapes
        .iter()
        .filter(|shape| (0..5).map(|x| v![x, 6]).all(|v| shape[v] == '#'))
        .collect_vec();

    let keys = shapes
        .iter()
        .filter(|shape| (0..5).map(|x| v![x, 0]).all(|v| shape[v] == '#'))
        .collect_vec();

    println!("--- locks");
    for shape in &locks {
        println!("{}", shape.render(|pos, c| *c));
    }

    println!("--- keys");
    for shape in &keys {
        println!("{}", shape.render(|pos, c| *c));
    }

    let mut total = 0;

    for lock in locks {
        for key in keys.iter() {
            if lock
                .iter_zm_with_pos()
                .all(|(pos, c)| !((*c == '#') && (key[pos] == '#')))
            {
                total += 1;
            }
        }
    }

    total
}

aoc_tests! {
    inputs {
        e0 = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####",
    }

    part::One {
        ea0: e0 => 0,
        ra: @input => 0,
    }

    part::Two {
        eb0: e0 => 0,
        rb: @input => 0,
    }
}
