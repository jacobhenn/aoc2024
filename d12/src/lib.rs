#![feature(let_chains)]

use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 12;

pub fn solve<Part: AocPart>(input: &str) -> usize {
    let plots = Grid::from_lines(input.trim());

    let mut idd_plots: Grid<usize> = Grid::from_dims_and_val(plots.dimensions(), 0);

    let mut i = 1;
    for pos in plots.positions_zm() {
        if idd_plots[pos] != 0 {
            continue;
        }

        idd_plots[pos] = i;

        let letter = plots[pos];

        let mut interior: HashSet<GridPos> = HashSet::new();
        let mut frontier: HashSet<GridPos> = HashSet::new();

        frontier.insert(pos);

        while !frontier.is_empty() {
            let mut new_frontier = HashSet::new();

            for f in frontier.iter().cloned().collect_vec() {
                for npos in plots.rook_neighbor_positions(f) {
                    if interior.contains(&npos) {
                        continue;
                    }

                    if plots[npos] == letter {
                        idd_plots[npos] = i;
                        new_frontier.insert(npos);
                    }
                }

                interior.insert(f);
            }

            mem::swap(&mut frontier, &mut new_frontier);
        }

        i += 1;
    }

    let s = idd_plots.render(|p, c| c.to_string().chars().next().unwrap());
    log!("{s}");

    let mut areas: HashMap<usize, usize> = HashMap::new();

    let mut perimeters: HashMap<usize, usize> = HashMap::new();

    let mut borders: HashMap<GridPos, usize> = HashMap::new();

    for (pos, &letter) in idd_plots.iter_zm_with_pos() {
        *areas.entry(letter).or_insert(0) += 1;

        for dir in RookDirection::iter() {
            let neighbor = idd_plots.get(pos + dir.unit_vector::<isize>());
            if neighbor.is_none_or(|c| *c != letter) {
                // log!("found {letter:?}-{neighbor:?} boundary at {pos} in dir {dir:?}");

                *perimeters.entry(letter).or_insert(0) += 1;

                borders.insert(pos, letter);
            }
        }
    }

    log!("areas: {areas:?}");
    log!("perimeters: {perimeters:?}");

    if Part::is_one() {
        return areas.iter().map(|(k, a)| a * perimeters[k]).sum();
    }

    // --------------------- PART TWO ----------------------------

    let mut side_counts: HashMap<usize, usize> = HashMap::new();

    let mut idd_plots1 = Grid::from_dims_and_val([plots.height(), plots.width()], 0);
    for (pos, &id) in idd_plots.iter_zm_with_pos() {
        idd_plots1[v![idd_plots.height() as isize - pos[1] - 1, pos[0]]] = id;
    }
    let mut idd_plots2 = Grid::from_dims_and_val(plots.dimensions(), 0);
    for (pos, &id) in idd_plots.iter_zm_with_pos() {
        idd_plots2[v![
            idd_plots.width() as isize - pos[0] - 1,
            idd_plots.height() as isize - pos[1] - 1
        ]] = id;
    }
    let mut idd_plots3 = Grid::from_dims_and_val([plots.height(), plots.width()], 0);
    for (pos, &id) in idd_plots.iter_zm_with_pos() {
        idd_plots3[v![pos[1], idd_plots.width() as isize - pos[0] - 1]] = id;
    }

    for (i, idd_plotsn) in [idd_plots, idd_plots1, idd_plots2, idd_plots3]
        .into_iter()
        .enumerate()
    {
        log!(
            "plots{i}:\n{}",
            idd_plotsn.render(|_, d| d.to_string().chars().next().unwrap())
        );

        let _g = log::enter();

        let mut current_id = 0;

        for (pos, &id) in idd_plotsn.iter_zm_with_pos() {
            if idd_plotsn.get(pos + v![0, -1]).is_none_or(|d| *d != id) {
                if id == current_id {
                    // do nothing
                } else if current_id == 0 {
                    // start run
                    current_id = id;
                } else {
                    // transition between two abutting runs
                    log!("trans ending {current_id} at {pos}");
                    *side_counts.entry(current_id).or_insert(0) += 1;
                    current_id = id;
                }
            } else {
                if current_id != 0 {
                    log!("new ending {current_id} at {pos}");
                    *side_counts.entry(current_id).or_insert(0) += 1;
                    current_id = 0;
                } else {
                    current_id = 0;
                }
            }
        }

        if current_id != 0 {
            log!("end ending {current_id}");
            *side_counts.entry(current_id).or_insert(0) += 1;
        }

        log!("plots{i}: {side_counts:?}");
    }

    log!("side_counts: {side_counts:?}");

    areas.iter().map(|(k, a)| a * side_counts[k]).sum()
}

aoc_tests! {
    inputs {
        e0 = "AAAA
BBCD
BBCC
EEEC",

        e1 = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",

        e2 = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",

        e3 = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",

        e4 = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
    }

    part::One {
        ea0: e0 => 140,
        ea1: e1 => 772,
        ea2: e2 => 1930,
        ra: @input => 1363484,
    }

    part::Two {
        eb0: e0 => 80,
        eb1: e1 => 436,
        eb2: e2 => 1206,
        eb3: e3 => 236,
        eb4: e4 => 368,
        rb: @input => 838988,
    }
}
