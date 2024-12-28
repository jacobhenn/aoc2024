use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 8;

// parse!(s => "Card " (_n) ": " (have) " | " @last (winning));

fn solve<Part: AocPart>(input: &str) -> usize {
    let grid = Grid::from_lines(input.trim());

    let antennas: HashMap<GridPos, char> = grid
        .iter_zm_with_pos()
        .filter(|(pos, c)| **c != '.')
        .map(|(pos, c)| (pos, *c))
        .collect();

    let mut antinodes: HashSet<GridPos> = HashSet::new();

    for (pos0, freq0) in &antennas {
        for (pos1, freq1) in &antennas {
            if freq0 != freq1 || pos0 == pos1 {
                continue;
            }

            log!("found a pair of {freq0}s at {pos0} and {pos1}");

            if Part::is_one() {
                let anode0 = *pos0 + (*pos0 - *pos1);
                let anode1 = *pos1 + (*pos1 - *pos0);

                log!("-> anode0: {anode0}");
                log!("-> anode1: {anode1}");

                antinodes.insert(anode0);
                antinodes.insert(anode1);
            } else {
                for n in 0.. {
                    if !grid.contains_pos(*pos0 + (*pos0 - *pos1) * n) {
                        break;
                    }

                    antinodes.insert(*pos0 + (*pos0 - *pos1) * n);
                }
                for n in 0.. {
                    if !grid.contains_pos(*pos1 + (*pos1 - *pos0) * n) {
                        break;
                    }

                    antinodes.insert(*pos1 + (*pos1 - *pos0) * n);
                }
            }
        }
    }

    antinodes
        .iter()
        .filter(|pos| grid.contains_pos(**pos))
        .count()
}

aoc_tests! {
    inputs {
        e0 = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
    }

    part::One {
        ea0: e0 => 14,
        ra: @input => 293,
    }

    part::Two {
        eb0: e0 => 34,
        rb: @input => 934,
    }
}
