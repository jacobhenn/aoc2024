use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 5;

// parse!(s => "Card " (_n) ": " (have) " | " @last (winning));

pub fn sort(line: Vec<i32>, rules: &[(i32, i32)]) -> Vec<i32> {
    let _g = log::enter();

    let mut res = Vec::new();

    for page in line {
        let i = res
            .iter()
            .position(|n| {
                let _g = log::enter();
                log!("searching for {n}|{page}");
                !rules.iter().contains(&(*n, page))
            })
            .unwrap_or(res.len());

        log!("inserting {page} at {i}");

        res.insert(i, page);

        log!("new res: {res:?}");
    }

    res
}

pub fn is_correctly_ordered(line: &[i32], rules: &[(i32, i32)]) -> bool {
    let mut ordered = true;
    'pages: for (i, page) in line.iter().enumerate() {
        for rule @ (l, r) in rules {
            if page == l {
                for lpage in line.iter().take(i) {
                    if lpage == r {
                        ordered = false;
                        break 'pages;
                    }
                }
            }
        }
    }

    ordered
}

pub fn solve<Part: AocPart>(input: &str) -> i32 {
    log!("hello");

    let (ruless, pagess) = input.trim().split_once("\n\n").unwrap();

    let rules = ruless
        .lines()
        .map(|line| {
            let (l, r) = line.split_once('|').unwrap();
            (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap())
        })
        .collect_vec();

    let mut pages = pagess
        .lines()
        .map(|line| {
            line.split(',')
                .map(|ns| ns.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut res = 0;

    for line in &mut pages {
        if Part::is_one() {
            if is_correctly_ordered(&line, &rules) {
                res += line[line.len() / 2];
            }
        } else {
            if !is_correctly_ordered(&line, &rules) {
                log!("{line:?} is not correctly ordered");
                *line = sort(line.clone(), &rules);
                log!(" -> {line:?}");
                res += line[line.len() / 2];
            }
        }
    }

    res
}

aoc_tests! {
    inputs {
        e0 = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
",
    }

    part::One {
        ea0: e0 => 143,
        ra: @input => 5275,
    }

    part::Two {
        eb0: e0 => 123,
        rb: @input => 6191,
    }
}
