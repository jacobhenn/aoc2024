#![feature(let_chains)]

use aocutil::prelude::*;
use yansi::Paint;

pub const YEAR: usize = 2024;

pub const DAY: usize = 16;

struct ReindeerGraph<'grid> {
    grid: &'grid Grid<char>,
    backwards: bool,
}

impl<'grid> Graph for ReindeerGraph<'grid> {
    type Distance = usize;

    type Node = Particle<isize>;

    fn neighbors<'a>(
        &'a self,
        &center: &'a Self::Node,
    ) -> impl Iterator<Item = (Self::Distance, Self::Node)> + 'a {
        let turn_cost = if self.backwards && self.grid[center.position] == 'E' { 0 } else { 1000 };

        [
            (1, if self.backwards { Particle::unstepped } else { Particle::stepped }(center)),
            (turn_cost, center.map_velocity(|vel| vel.rotated(Rotation::IJ))),
            (turn_cost, center.map_velocity(|vel| vel.rotated(Rotation::JI))),
        ]
        .into_iter()
        .filter(|&(_, p)| self.grid.get(p.position).is_some_and(|&c| c != '#'))
    }
}

pub fn solve<Part: AocPart>(input: &str) -> usize {
    let grid = Grid::from_lines(input);

    let start_pos = grid.find(|&c| c == 'S').unwrap();
    let end_pos = grid.find(|&c| c == 'E').unwrap();

    let mut graph = ReindeerGraph { grid: &grid, backwards: false };

    let (_, forward_spanning_tree) =
        graph.shortest_paths_dijkstra(start_pos.with_velocity(v![1, 0]), |_| false, usize::cmp);

    let &forward_best_score = RookDirection::iter()
        .map(|dir| forward_spanning_tree.get(&end_pos.with_velocity(dir.unit_vector())))
        .flatten()
        .map(|(score, _)| score)
        .min()
        .expect("end should be reachable");

    if Part::is_one() {
        return forward_best_score;
    }

    graph.backwards = true;

    let (_, backward_spanning_tree) =
        graph.shortest_paths_dijkstra(end_pos.with_velocity(v![1, 0]), |_| false, usize::cmp);

    grid.positions_zm()
        .filter(|&pos| {
            RookDirection::iter().any(|dir| {
                let p = dir.unit_vector().with_position(pos);
                [forward_spanning_tree.get(&p), backward_spanning_tree.get(&p)]
                    .into_iter()
                    .flatten()
                    .map(|(score, _)| score)
                    .sum::<usize>()
                    == forward_best_score
            })
        })
        .count()
}

aoc_tests! {
    inputs {
        e0 = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",

        e1 = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
    }

    part::One {
        ea0: e0 => 7036,
        ea1: e1 => 11048,
        ra: @input => 98484,
    }

    part::Two {
        eb0: e0 => 45,
        eb1: e1 => 64,
        rb: @input => 0,
    }
}
