#![feature(array_chunks)]
#![feature(let_chains)]

use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 18;

struct MemGraph<'bytes>(&'bytes HashSet<GridPos>, usize);

impl<'bytes> Graph for MemGraph<'bytes> {
    type Distance = usize;

    type Node = GridPos;

    fn neighbors<'a>(
        &'a self,
        center: &'a Self::Node,
    ) -> impl Iterator<Item = (Self::Distance, Self::Node)> + 'a {
        RookDirection::iter()
            .map(|dir| *center + dir)
            .filter(|neighbor| {
                (0..=(self.1 as isize)).contains(&neighbor[0])
                    && (0..=(self.1 as isize)).contains(&neighbor[1])
            })
            .filter(|neighbor| !self.0.contains(neighbor))
            .map(|neighbor| (1, neighbor))
    }
}

pub fn solve<Part: AocPart>(input: &str, grid_size: usize, bytes_to_take: usize) -> String {
    let corrupted_bytes: Vec<GridPos> = input
        .lines()
        .map(|line| {
            unlist!(ints::<isize>(line), col, row);
            v![col, row]
        })
        .collect();

    if Part::is_one() {
        let considered_bytes = corrupted_bytes.into_iter().take(bytes_to_take).collect();
        let graph = MemGraph(&considered_bytes, grid_size);

        let (end, spanning_tree) = graph.shortest_paths_dijkstra(
            v![0, 0],
            |v| *v == v![grid_size as isize, grid_size as isize],
            usize::cmp,
        );

        return spanning_tree[&end.unwrap()].0.to_string();
    }

    let mut considered_bytes = HashSet::new();

    for &byte in &corrupted_bytes {
        considered_bytes.insert(byte);

        let graph = MemGraph(&considered_bytes, grid_size);

        let (end, _spanning_tree) = graph.shortest_paths_dijkstra(
            v![0, 0],
            |v| *v == v![grid_size as isize, grid_size as isize],
            usize::cmp,
        );

        if !end.is_some() {
            return byte.components.iter().map(|c| c.to_string()).join(",");
        }
    }

    unreachable!()
}

aoc_tests! {
    inputs {
        e0 = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
    }

    part::One {
        ea0: e0, 6, 12 => "22",
        ra: @input 70, 1024 => "438",
    }

    part::Two {
        eb0: e0, 6, 0 => "6,1",
        rb: @input 70, 0 => "26,22",
    }
}
