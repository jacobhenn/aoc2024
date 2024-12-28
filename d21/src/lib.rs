#![feature(let_chains)]

use std::str;

use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 21;

pub fn grid_pathfind(gap: GridPos, start: GridPos, target: GridPos) -> Vec<String> {
    if start == gap {
        panic!("start == gap == {start}");
    }

    let mut paths = Vec::new();

    for axes in [[0, 1], [1, 0]] {
        if start[axes[1]] == gap[axes[1]] && target[axes[0]] == gap[axes[0]] {
            continue;
        }

        let mut path = String::new();

        for axis in axes {
            let dir_along_axis = LineDirection::of_num(&(target[axis] - start[axis]));
            let dir = RookDirection::new(axis, dir_along_axis);
            let amt = (target[axis] - start[axis]).abs() as usize;
            for _ in 0..amt {
                path.push(dir.to_ascii_arrow());
            }
        }

        paths.push(path);
    }

    paths
}

#[derive(Clone)]
pub enum StrTree {
    Branch { l: Rc<StrTree>, r: Rc<StrTree>, len: usize },
    Leaf(String),
    Nil,
}

impl StrTree {
    fn concat(l: StrTree, r: StrTree) -> StrTree {
        let len = l.len() + r.len();
        StrTree::Branch { l: Rc::new(l), r: Rc::new(r), len }
    }

    fn len(&self) -> usize {
        match self {
            StrTree::Branch { len, .. } => *len,
            StrTree::Leaf(s) => s.len(),
            StrTree::Nil => 0,
        }
    }
}

pub fn solve_recursive_impl<'a>(
    kp_lookup_at_level: &impl Fn(usize) -> &'a HashMap<char, GridPos>,
    level: usize,
    start: GridPos,
    target: char,
    memo: &mut HashMap<(usize, GridPos, char), StrTree>,
) -> StrTree {
    let _g = log::enter();

    if let Some(res) = memo.get(&(level, start, target)) {
        return res.clone();
    }

    let kp_lookup = kp_lookup_at_level(level);
    let gap = kp_lookup[&' '];

    let target_pos = kp_lookup[&target];

    let mut possible_paths = grid_pathfind(gap, start, target_pos);

    if level == 0 {
        let mut path = possible_paths.pop().unwrap();
        path.push('A');

        let res = StrTree::Leaf(path);
        memo.insert((level, start, target), res.clone());
        return res;
    }

    let mut possible_ress = Vec::new();
    for mut path in possible_paths {
        path.push('A');

        let _g = log::enter();

        let mut current_pos = kp_lookup_at_level(level - 1)[&'A'];
        let mut current_expansion = StrTree::Nil;

        for c in path.chars() {
            let _g = log::enter();

            let c_expanded =
                solve_recursive_impl(kp_lookup_at_level, level - 1, current_pos, c, memo);
            current_expansion = StrTree::concat(current_expansion, c_expanded);
            current_pos = kp_lookup_at_level(level - 1)[&c];
        }

        possible_ress.push(current_expansion);
    }

    let res = possible_ress.into_iter().min_by_key(|s| s.len()).unwrap();
    memo.insert((level, start, target), res.clone());
    res
}

pub fn solve_recursive<const NUM_ROBOTS: usize>(input: &str) -> usize {
    log!("solving recursively");
    let _g = log::enter();

    let num_keypad = Grid::from_lines("789\n456\n123\n 0A");
    let dir_keypad = Grid::from_lines(" ^A\n<v>");
    let num_kp_lookup =
        num_keypad.iter_zm_with_pos().map(|(pos, &c)| (c, pos)).collect::<HashMap<_, _>>();
    let dir_kp_lookup =
        dir_keypad.iter_zm_with_pos().map(|(pos, &c)| (c, pos)).collect::<HashMap<_, _>>();

    let kp_lookup_at_level =
        |level| if level == NUM_ROBOTS { &num_kp_lookup } else { &dir_kp_lookup };

    let mut total_complexity = 0;

    let mut memo = HashMap::new();

    for line in input.trim().lines() {
        log!("typing {line}");
        let _g = log::enter();

        let mut current_pos = num_kp_lookup[&'A'];
        let mut current_expansion = StrTree::Nil;

        for c in line.chars() {
            let c_expanded =
                solve_recursive_impl(&kp_lookup_at_level, NUM_ROBOTS, current_pos, c, &mut memo);
            current_expansion = StrTree::concat(current_expansion, c_expanded);
            current_pos = num_kp_lookup[&c];
        }

        total_complexity += current_expansion.len() * ints::<usize>(line).next().unwrap();
    }

    total_complexity
}

pub fn solve<Part: AocPart>(input: &str) -> usize {
    if Part::is_one() {
        solve_recursive::<2>(input)
    } else {
        solve_recursive::<25>(input)
    }
}

aoc_tests! {
    inputs {
        e0 = "029A
980A
179A
456A
379A",

        e0_0 = "029A",
        e0_1 = "980A",
        e0_2 = "179A",
        e0_3 = "456A",
        e0_4 = "379A",

        era0 = "671A",
        era1 = "279A",
        era2 = "083A",
        era3 = "974A",
        era4 = "386A",
    }

    part::One {
        ea0: e0 => 126384,
        ea0_0: e0_0 => 68 * 29,
        ea0_1: e0_1 => 60 * 980,
        ea0_2: e0_2 => 68 * 179,
        ea0_3: e0_3 => 64 * 456,
        ea0_4: e0_4 => 64 * 379,
        ra_0: era0 => 74 * 671,
        ra_1: era1 => 72 * 279,
        ra_2: era2 => 66 * 83,
        ra_3: era3 => 72 * 974,
        ra_4: era4 => 68 * 386,
        ra: @input => 171596,
    }

    part::Two {
        rb: @input => 209268004868246,
    }
}
