use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 4;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn count_xmas_at(grid: &Grid<char>, at: GridPos) -> usize {
    if grid[at] != 'X' {
        return 0;
    }

    let mut count = 0;

    'dir: for direction in QueenDirection::iter() {
        println!("  checking direction {}", direction.unit_vector::<i32>());
        for i in 1..=3isize {
            let v = at + (direction.unit_vector::<isize>() * i);
            println!("    v: {v} -> {:?}", grid.get(v));
            if grid.get(v) != Some(&XMAS[i as usize]) {
                println!("    break");
                continue 'dir;
            }
        }

        println!("    found");
        count += 1;
    }

    count
}

fn is_x_mas_in_dir(grid: &Grid<char>, at: GridPos, dir: QueenDirection<2>) -> bool {
    (grid.get(at + dir.unit_vector::<isize>()) == Some(&'M')
        && grid.get(at - dir.unit_vector::<isize>()) == Some(&'S'))
        || (grid.get(at + dir.unit_vector::<isize>()) == Some(&'S')
            && grid.get(at - dir.unit_vector::<isize>()) == Some(&'M'))
}

fn is_x_mas(grid: &Grid<char>, at: GridPos) -> bool {
    if grid[at] != 'A' {
        return false;
    }

    is_x_mas_in_dir(grid, at, QueenDirection::MINUS_MINUS)
        && is_x_mas_in_dir(grid, at, QueenDirection::PLUS_MINUS)
}

pub fn solve<Part: AocPart>(input: &str) -> usize {
    let grid: Grid<char> = input.trim().lines().map(|line| line.chars()).collect();

    println!("{}", grid.render(|_, c| *c));

    if Part::is_two() {
        return grid
            .positions_zm()
            .filter(|pos| is_x_mas(&grid, *pos))
            .count();
    }

    let mut count = 0;

    for position in grid.positions_zm() {
        println!("position: {position}");
        let count_here = count_xmas_at(&grid, position);
        println!("count_here: {count_here}");
        count += count_here;
    }

    println!(
        "{}",
        grid.render(|pos, c| if count_xmas_at(&grid, pos) == 0 {
            '.'
        } else {
            *c
        })
    );

    count
}

aoc_tests! {
    inputs {
        e0 = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    }

    part::One {
        ea0: e0 => 18,
        ra: @input => 2613,
    }

    part::Two {
        eb0: e0 => 9,
        rb: @input => 1905,
    }
}
