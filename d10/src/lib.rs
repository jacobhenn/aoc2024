use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 10;

// parse!(s => "Card " (_n) ": " (have) " | " @last (winning));

fn count_trails<Part: AocPart>(
    grid: &Grid<u32>,
    current: GridPos,
    reached: &mut HashSet<GridPos>,
) -> usize {
    let _g = log::enter();
    log!("count_trails(current: {current})");

    if grid[current] == 9 && (Part::is_two() || reached.insert(current)) {
        return 1;
    }

    let mut result = 0;

    for neighbor_pos in grid.rook_neighbor_positions(current) {
        if grid[neighbor_pos].saturating_sub(grid[current]) == 1 {
            log!("stepping from {} to {}", grid[current], grid[neighbor_pos]);
            let x = count_trails::<Part>(grid, neighbor_pos, reached);
            log!("finds {x} trails");
            result += x;
        }
    }

    result
}

fn solve<Part: AocPart>(input: &str) -> usize {
    let grid: Grid<u32> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();

    let mut res = 0;

    for (pos, &height) in grid.iter_zm_with_pos() {
        if height == 0 {
            // trailhead
            log!("trailhead at {pos}");
            let score = count_trails::<Part>(&grid, pos, &mut HashSet::new());
            log!("-> score: {score}");
            res += score;
        }
    }

    res
}

aoc_tests! {
    inputs {
        e0 = "0123
1234
8765
9876",

        e1 = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    }

    part::One {
        ea0: e0 => 1,
        ea1: e1 => 36,
        ra: @input => 517,
    }

    part::Two {
        eb1: e1 => 81,
        rb: @input => 1116,
    }
}
