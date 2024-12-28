#![feature(let_chains)]

use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 15;

pub fn solve_b(input: &str) -> isize {
    let (grid_s, moves_s) = input.split_once("\n\n").unwrap();

    let pre_grid: Grid<char> = Grid::from_lines(grid_s);

    let mut grid: Grid<char> =
        Grid::from_dims_and_val([pre_grid.width() * 2, pre_grid.height()], 'X');

    for (mut pos, c) in pre_grid.iter_zm_with_pos() {
        pos[0] *= 2;

        match c {
            '#' => {
                grid[pos] = '#';
                grid[pos + RookDirection::PLUS_X] = '#';
            }
            '.' => {
                grid[pos] = '.';
                grid[pos + RookDirection::PLUS_X] = '.';
            }
            'O' => {
                grid[pos] = '[';
                grid[pos + RookDirection::PLUS_X] = ']';
            }
            '@' => {
                grid[pos] = '@';
                grid[pos + RookDirection::PLUS_X] = '.';
            }
            other => panic!("{other}"),
        }
    }

    let mut robot_pos = grid.find(|&c| c == '@').unwrap();

    let mut boxes: HashSet<GridPos> = grid
        .positions_zm()
        .filter(|&pos| grid[pos] == '[')
        .collect();

    fn try_locate_box(pos: GridPos, boxes: &HashSet<GridPos>) -> Option<GridPos> {
        if boxes.contains(&pos) {
            Some(pos)
        } else if boxes.contains(&(pos + RookDirection::MINUS_X)) {
            Some(pos + RookDirection::MINUS_X)
        } else {
            None
        }
    }

    'mvs: for mv in moves_s.chars() {
        let Some(dir) = RookDirection::from_ascii_arrow(mv) else {
            continue;
        };

        let looking_pos = robot_pos + dir;

        let mut shift_queue: VecDeque<GridPos> = VecDeque::new();
        let mut shift_set: HashSet<GridPos> = HashSet::new();
        if let Some(box_pos) = try_locate_box(looking_pos, &boxes) {
            shift_queue.push_back(box_pos);
        }

        while let Some(box_pos) = shift_queue.pop_front() {
            log!("box_pos: {box_pos}");
            log!("shift_queue: {shift_queue:?}");
            log!("shift_set: {shift_set:?}");
            log!("boxes: {boxes:?}");

            if let Some(next_pos) = try_locate_box(box_pos + dir, &boxes) {
                if next_pos != box_pos && !shift_queue.contains(&next_pos) {
                    shift_queue.push_back(next_pos);
                }
            } else if grid[box_pos + dir] == '#' {
                continue 'mvs;
            }

            if let Some(next_pos) = try_locate_box(box_pos + RookDirection::PLUS_X + dir, &boxes) {
                if next_pos != box_pos && !shift_queue.contains(&next_pos) {
                    shift_queue.push_back(next_pos);
                }
            } else if grid[box_pos + RookDirection::PLUS_X + dir] == '#' {
                continue 'mvs;
            }

            shift_set.insert(box_pos);
        }

        for old_pos in &shift_set {
            boxes.remove(old_pos);
        }

        for &old_pos in &shift_set {
            boxes.insert(old_pos + dir);
        }

        log!("new boxes: {boxes:?}");

        if grid[robot_pos + dir] != '#' && try_locate_box(robot_pos + dir, &boxes).is_none() {
            robot_pos += dir;
        }

        log!(
            "after move {mv}:\n{}",
            grid.render(|pos, &c| if c == '#' {
                '#'
            } else if pos == robot_pos {
                '@'
            } else if try_locate_box(pos, &boxes) == Some(pos) {
                '['
            } else if try_locate_box(pos, &boxes).is_some() {
                ']'
            } else {
                '.'
            })
        );
    }

    boxes.into_iter().map(|pos| pos[1] * 100 + pos[0]).sum()
}

pub fn solve<Part: AocPart>(input: &str) -> isize {
    if Part::is_two() {
        return solve_b(input);
    }

    let (grid_s, moves_s) = input.split_once("\n\n").unwrap();

    let grid: Grid<char> = Grid::from_lines(grid_s);

    let mut robot_pos = grid.find(|&c| c == '@').unwrap();

    let mut boxes: HashSet<GridPos> = grid
        .positions_zm()
        .filter(|&pos| grid[pos] == 'O')
        .collect();

    for mv in moves_s.chars() {
        let Some(dir) = RookDirection::from_ascii_arrow(mv) else {
            continue;
        };

        let mut looking_pos = robot_pos + dir;

        if boxes.contains(&looking_pos) {
            while boxes.contains(&looking_pos) {
                looking_pos += dir;
            }

            if grid[looking_pos] == '#' {
                continue;
            } else {
                boxes.remove(&(robot_pos + dir));
                boxes.insert(looking_pos);
            }
        }

        if grid[robot_pos + dir] != '#' && !boxes.contains(&(robot_pos + dir)) {
            robot_pos += dir;
        }

        log!(
            "after move {mv}:\n{}",
            grid.render(|pos, &c| if c == '#' {
                '#'
            } else if pos == robot_pos {
                '@'
            } else if boxes.contains(&pos) {
                'O'
            } else {
                '.'
            })
        );
    }

    boxes.into_iter().map(|pos| pos[1] * 100 + pos[0]).sum()
}

aoc_tests! {
    inputs {
        e0 = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",

        e1 = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",

        e2 = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^",
    }

    part::One {
        ea0: e0 => 2028,
        ea1: e1 => 10092,
        ra: @input => 1426855,
    }

    part::Two {
        eb1: e1 => 9021,
        rb: @input => 1404917,
    }
}
