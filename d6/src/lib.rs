use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 6;

// parse!(s => "Card " (_n) ": " (have) " | " @last (winning));

#[derive(PartialEq, Eq, Copy, Clone)]
enum SimulationResult {
    Exited,
    Looped,
}

fn simulate_slow(
    mut guard_pos: GridPos,
    grid: &Grid<char>,
) -> (SimulationResult, HashMap<GridPos, HashSet<RookDirection>>) {
    let mut visited_positions: HashMap<GridPos, HashSet<RookDirection>> = HashMap::new();

    let mut guard_dir = RookDirection::MINUS_Y;

    let mut count = 0;

    loop {
        count += 1;
        // log!("guard at {guard_pos}");

        if !visited_positions
            .entry(guard_pos)
            .or_insert_with(HashSet::new)
            .insert(guard_dir)
        {
            log!("looped! {count}");
            return (SimulationResult::Looped, visited_positions);
        }

        match grid.get(guard_pos + guard_dir) {
            Some('#') => guard_dir = guard_dir.rotated(Rotation::IJ),
            Some('.') | Some('^') => guard_pos += guard_dir,
            Some(other) => panic!("other: {other}"),
            None => return (SimulationResult::Exited, visited_positions),
        }
    }
}

fn simulate_fast(
    mut guard_pos: GridPos,
    obstacles_by_axis: [HashMap<isize, Vec<GridPos>>; 2],
) -> (SimulationResult, HashMap<GridPos, HashSet<RookDirection>>) {
    // let _g = log::enter();

    let mut visited_positions: HashMap<GridPos, HashSet<RookDirection>> = HashMap::new();

    let mut guard_dir = RookDirection::MINUS_Y;

    let mut count = 0;

    loop {
        count += 1;

        // log!(
        //     "pos: {guard_pos}, dir: {guard_dir:?}, visited_positions[guard_pos]: {:?}",
        //     visited_positions.get(&guard_pos)
        // );

        let Some(&hit_obstacle) = obstacles
            .iter()
            .filter(|o| (**o - guard_pos).rook_direction_if_axis_aligned() == Some(guard_dir))
            .min_by_key(|o| o.manhattan_distance(guard_pos))
        else {
            log!("-> exited. count: {count}");
            return (SimulationResult::Exited, visited_positions);
        };

        guard_pos = hit_obstacle - guard_dir.unit_vector::<isize>();
        guard_dir = guard_dir.rotated(Rotation::IJ);

        if !visited_positions
            .entry(guard_pos)
            .or_insert_with(HashSet::new)
            .insert(guard_dir)
        {
            log!("-> looped! count: {count}");
            return (SimulationResult::Looped, visited_positions);
        }
    }
}

fn solve<Part: AocPart>(input: &str) -> usize {
    log!("hello");

    let mut grid = Grid::from_lines(input.trim());

    let guard_pos = grid
        .iter_zm_with_pos()
        .find(|(pos, char)| **char == '^')
        .unwrap()
        .0;

    let (_, visited_positions) = simulate_slow(guard_pos, &grid);

    if Part::is_one() {
        return visited_positions.len();
    } else {
        let mut obstacles_by_axis: [HashMap<isize, Vec<GridPos>>; 2] =
            array::from_fn(|_| HashMap::new());

        for (pos, &c) in grid.iter_zm_with_pos() {
            if c == '#' {
                for axis in 0..2 {
                    let v = obstacles_by_axis[axis]
                        .entry(pos[axis])
                        .or_insert_with(Vec::new);
                    let i = v
                        .binary_search_by_key(&pos[axis], |pos| pos[axis])
                        .unwrap_or_else(|i| i);
                    v.insert(i, pos);
                }
            }
        }

        let mut obstacles: HashSet<GridPos> = grid
            .positions_zm()
            .filter(|pos| grid[*pos] == '#')
            .collect();

        let mut count = 0;

        for (i, &pos) in visited_positions.keys().enumerate() {
            // log!("checking {pos} ({i}/{})", visited_positions.len());

            if pos == guard_pos {
                continue;
            }

            let prev = mem::replace(&mut grid[pos], '#');
            obstacles.insert(pos);
            if simulate_slow(guard_pos, &grid).0 == SimulationResult::Looped {
                count += 1;
            }
            obstacles.remove(&pos);
            grid[pos] = prev;
        }

        return count;
    }
}

// ....#.....
// .........#
// ..........
// ..#.......
// .......#..
// ..........
// .#..^.....
// ........#.
// #.........
// ......#...

aoc_tests! {
    inputs {
        e0 = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
    }

    part::One {
        ea0: e0 => 41,
        ra: @input => 4647,
    }

    part::Two {
        eb0: e0 => 6,
        rb: @input => 1723,
    }
}
