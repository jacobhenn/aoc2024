use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 20;

struct RacetrackGraph<'graph> {
    grid: &'graph Grid<char>,
    shortcut: Option<(GridPos, GridPos)>,
}

impl<'graph> Graph for RacetrackGraph<'graph> {
    type Distance = usize;

    type Node = GridPos;

    fn neighbors<'a>(
        &'a self,
        center: &'a Self::Node,
    ) -> impl Iterator<Item = (Self::Distance, Self::Node)> + 'a {
        self.grid
            .rook_neighbors_with_positions(*center)
            .filter(|&(pos, &c)| c != '#')
            .map(|(pos, c)| (1, pos))
            .chain(
                self.shortcut
                    .filter(|(start, _end)| start == center)
                    .map(|(start, end)| (start.manhattan_distance(end) as usize, end)),
            )
    }
}

pub fn solve<ThisPart: AocPart>(input: &str, min_savings: usize) -> usize {
    let cut_max_time = match ThisPart::part() {
        Part::One => 2,
        Part::Two => 20,
    };

    let grid = Grid::from_lines(input.trim());

    let start_pos = grid.find(|c| *c == 'S').unwrap();
    let end_pos = grid.find(|c| *c == 'E').unwrap();

    let graph = RacetrackGraph {
        grid: &grid,
        shortcut: None,
    };

    let (_, forward_spanning_tree) =
        graph.shortest_paths_dijkstra(start_pos, |pos| *pos == end_pos, usize::cmp);
    let &(shortest_time_no_shortcut, _) = forward_spanning_tree.get(&end_pos).unwrap();
    log!("{shortest_time_no_shortcut} no cut");

    let (_, backward_spanning_tree) =
        graph.shortest_paths_dijkstra(end_pos, |pos| *pos == start_pos, usize::cmp);

    let mut shortcuts: HashMap<usize, usize> = HashMap::new();

    for cut_start in grid.positions_zm() {
        if cut_start[0] == 0 {
            println!("starting row {}", cut_start[1]);
        }
        for cut_end in grid.positions_zm() {
            if cut_start.manhattan_distance(cut_end) > cut_max_time {
                continue;
            }
            if grid[cut_start] == '#' || grid[cut_end] == '#' {
                continue;
            }
            let Some(&(start_to_cut_start, _)) = forward_spanning_tree.get(&cut_start) else {
                continue;
            };
            let start_to_cut_start = if cut_start == start_pos {
                0
            } else {
                start_to_cut_start
            };
            let Some(&(cut_end_to_end, _)) = backward_spanning_tree.get(&cut_end) else {
                continue;
            };
            let cut_end_to_end = if cut_end == end_pos {
                0
            } else {
                cut_end_to_end
            };
            let cut_time = cut_start.manhattan_distance(cut_end) as usize;

            let shortest_time_shortcut = start_to_cut_start + cut_time + cut_end_to_end;

            log!("{{start}} --{start_to_cut_start}-> {cut_start} --{cut_time}-> {cut_end} --{cut_end_to_end}-> {{end}}; total: {shortest_time_shortcut}");

            if shortest_time_no_shortcut.saturating_sub(shortest_time_shortcut) >= min_savings {
                *shortcuts
                    .entry(shortest_time_no_shortcut - shortest_time_shortcut)
                    .or_insert(0) += 1;
            }
        }
    }

    log!("shortcuts: {shortcuts:#?}");

    shortcuts.values().sum()
}

aoc_tests! {
    inputs {
        e0 = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
    }

    part::One {
        ea0: e0, 1 => 44,
        ra: @input 100 => 1332,
    }

    part::Two {
        eb0: e0, 50 => 285,
        rb: @input 100 => 987695,
    }
}
