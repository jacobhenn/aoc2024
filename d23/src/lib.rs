use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 23;

pub fn solve_b(input: &str) -> String {
    let mut adj: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.trim().lines() {
        let (l, r) = line.split_once("-").unwrap();

        adj.entry(l).or_insert_with(HashSet::new).insert(r);
        adj.entry(r).or_insert_with(HashSet::new).insert(l);
    }

    let all_computers: HashSet<&str> = adj
        .keys()
        .chain(adj.values().flatten())
        .map(|s| *s)
        .collect();

    let mut max_size = 0;
    let mut max_group = HashSet::new();

    for &center in &all_computers {
        // try to grow a LAN party from this computer

        let mut party: HashSet<&str> = [center].into_iter().collect();

        for &computer in &all_computers {
            if party
                .iter()
                .all(|party_computer| adj[party_computer].contains(computer))
            {
                party.insert(computer);
            }
        }

        max_size = cmp::max(max_size, party.len());
        if max_size == party.len() {
            max_group = party;
        }
    }

    max_group.iter().sorted().join(",")
}

pub fn solve<Part: AocPart>(input: &str) -> String {
    if Part::is_two() {
        return solve_b(input);
    }

    let mut adj: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.trim().lines() {
        let (l, r) = line.split_once("-").unwrap();

        adj.entry(l).or_insert_with(HashSet::new).insert(r);
        adj.entry(r).or_insert_with(HashSet::new).insert(l);
    }

    let all_computers: HashSet<&str> = adj
        .keys()
        .chain(adj.values().flatten())
        .map(|s| *s)
        .collect();

    let mut pairs: HashSet<[&str; 2]> = HashSet::new();

    for &computer in all_computers.iter().filter(|s| s.starts_with('t')) {
        for &neighbor in &adj[computer] {
            let mut pair = [computer, neighbor];
            pair.sort();
            pairs.insert(pair);
        }
    }

    let mut triples: HashSet<[&str; 3]> = HashSet::new();

    for &computer in &all_computers {
        for &pair in &pairs {
            if computer == pair[0] || computer == pair[1] {
                continue;
            }

            if adj[computer].contains(pair[0]) && adj[computer].contains(pair[1]) {
                let mut triple = [pair[0], pair[1], computer];
                triple.sort();
                triples.insert(triple);
            }
        }
    }

    log!("triples: {triples:?}");

    triples.len().to_string()
}

aoc_tests! {
    inputs {
        e0 = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn",
    }

    part::One {
        ea0: e0 => "7",
        ra: @input => "1348",
    }

    part::Two {
        eb0: e0 => "co,de,ka,ta",
        rb: @input => "am,bv,ea,gh,is,iy,ml,nj,nl,no,om,tj,yv",
    }
}
