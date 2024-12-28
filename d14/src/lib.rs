use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 14;

const TREE: &str = "███████████████████████████████
█                             █
█                             █
█                             █
█                             █
█              █              █
█             ███             █
█            █████            █
█           ███████           █
█          █████████          █
█            █████            █
█           ███████           █
█          █████████          █
█         ███████████         █
█        █████████████        █
█          █████████          █
█         ███████████         █
█        █████████████        █
█       ███████████████       █
█      █████████████████      █
█        █████████████        █
█       ███████████████       █
█      █████████████████      █
█     ███████████████████     █
█    █████████████████████    █
█             ███             █
█             ███             █
█             ███             █
█                             █
█                             █
█                             █
█                             █
███████████████████████████████";

pub fn solve_b(input: &str, width: i32, height: i32) -> usize {
    let mut robots: HashMap<Vector<i32>, Vec<Vector<i32>>> = HashMap::new();

    for line in input.trim().lines() {
        unlist!(ints(line), p0, p1, v0, v1);
        robots
            .entry(v![p0, p1])
            .or_insert_with(Vec::new)
            .push(v![v0, v1]);
    }

    let mut new_robots: HashMap<Vector<i32>, Vec<Vector<i32>>> = HashMap::new();

    let mut min_score = i32::MAX;

    for n in 0.. {
        for (&pos, vels) in &robots {
            for &vel in vels {
                let new_pos_unwrapped = pos + vel;
                let new_pos_wrapped = v![
                    new_pos_unwrapped[0].rem_euclid(width),
                    new_pos_unwrapped[1].rem_euclid(height)
                ];
                new_robots
                    .entry(new_pos_wrapped)
                    .or_insert_with(Vec::new)
                    .push(vel);
            }
        }

        // check for christmas tree

        let centroid = new_robots.keys().copied().sum::<Vector<i32>>() / new_robots.len() as i32;

        let mut symmetry_score = 0;

        // for &left_robot in new_robots.keys().filter(|p| p[0] < centroid[0]) {
        //     let reflection = left_robot + v![(centroid[0] - left_robot[0]) * 2, 0];
        //     let min_dist = new_robots
        //         .keys()
        //         .filter(|p| p[1] > centroid[1])
        //         .map(|p| p.distance::<metric::SquaredEuclidean, _, _>(reflection))
        //         .min()
        //         .unwrap();
        //     symmetry_score += min_dist * min_dist;
        // }501501

        // 1244 (v)
        // 1308 (h)
        // 1347 (v)
        // 1409 (h)
        // 1450 (v)
        // 1510 (h)
        // 27..57

        // (n - 8) % 103 == 0 || (n - 96) % 101 == 0
        // (28..57).all(|col| new_robots.contains_key(&v![col, 52]))

        if (28..57).all(|col| new_robots.contains_key(&v![col, 52])) {
            println!("n + 1 = {}", n + 1);

            for row in 0..width {
                print!("{row:03}");
                for col in 0..height {
                    if new_robots.contains_key(&v![col, row]) {
                        print!("█");
                    } else {
                        print!(" ")
                    }
                }
                println!();
            }

            min_score = symmetry_score;
            panic!();
        }

        robots.clear();
        mem::swap(&mut robots, &mut new_robots);
    }

    todo!()
}

pub fn solve<Part: AocPart>(input: &str, width: i32, height: i32) -> usize {
    if Part::is_two() {
        return solve_b(input, width, height);
    }

    let mut robots: HashMap<Vector<i32>, Vec<Vector<i32>>> = HashMap::new();

    for line in input.trim().lines() {
        unlist!(ints(line), p0, p1, v0, v1);
        robots
            .entry(v![p0, p1])
            .or_insert_with(Vec::new)
            .push(v![v0, v1]);
    }

    let mut new_robots: HashMap<Vector<i32>, Vec<Vector<i32>>> = HashMap::new();

    for (pos, vels) in robots {
        for vel in vels {
            let new_pos_unwrapped = pos + vel * 100;
            let new_pos_wrapped = v![
                new_pos_unwrapped[0].rem_euclid(width),
                new_pos_unwrapped[1].rem_euclid(height)
            ];
            new_robots
                .entry(new_pos_wrapped)
                .or_insert_with(Vec::new)
                .push(vel);
        }
    }

    log!("{new_robots:?}");

    let mut quadrant_counts: HashMap<(i32, i32), usize> = HashMap::new();

    for (pos, vels) in new_robots {
        if pos[0] == width / 2 || pos[1] == height / 2 {
            continue;
        }

        *quadrant_counts
            .entry((
                (pos[0] - width / 2).signum(),
                (pos[1] - height / 2).signum(),
            ))
            .or_insert(0) += vels.len();
    }

    log!("{quadrant_counts:?}");

    quadrant_counts.values().product()
}

aoc_tests! {
    inputs {
        e0 = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
    }

    part::One {
        ea0: e0, 11, 7 => 12,
        ra: @input 101, 103 => 34393,
    }

    part::Two {
        rb: @input 101, 103 => 83551068361379,
    }
}
