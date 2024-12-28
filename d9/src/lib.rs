use core::str;

use aocutil::prelude::*;

pub const YEAR: usize = 2024;

pub const DAY: usize = 9;

// parse!(s => "Card " (_n) ": " (have) " | " @last (winning));

pub fn solve_b(input: &str) -> usize {
    // (File ID -> File span) in order of file span
    let mut files: Vec<Range<usize>> = Vec::with_capacity(input.len() / 2);
    let mut gaps: [VecDeque<Range<usize>>; 10] =
        array::from_fn(|_| VecDeque::with_capacity(input.len() / 20));

    let mut pos = 0;
    let mut max = 0;
    for (i, c) in input.trim().chars().enumerate() {
        let c = c.to_digit(10).unwrap() as usize;

        let span = Range::new(pos, pos + c - 1);
        assert!((0..=9).contains(&span.len()), "{}", span.len());

        if i % 2 == 0 {
            files.push(span);
            max = span.end;
        } else if span.len() != 0 {
            gaps[span.len()].push_back(span);
        }

        pos += c;
    }

    log!("gaps: {gaps:?}");

    for id in (0..files.len()).rev() {
        let _g = log::enter();
        log!("moving file {id}");

        let moving_file = &mut files[id];

        if let Some(gap) = gaps[moving_file.len()..]
            .iter_mut()
            .filter_map(|v| v.get(0))
            .min_by_key(|r| r.start)
        {
            let _g = log::enter();
            log!("found gap {gap:?}");

            if gap.start > moving_file.start {
                continue;
            }

            let new_span = Range::new(gap.start, gap.start + moving_file.len() - 1);
            log!("moving from {moving_file:?} to {new_span:?}");
            *moving_file = new_span;

            let mut gap = gaps[gap.len()].pop_front().unwrap();

            if gap.len() > moving_file.len() {
                gap.start += moving_file.len();
                let insert_idx = gaps[gap.len()]
                    .binary_search_by_key(&gap.start, |g| g.start)
                    .unwrap_err();
                gaps[gap.len()].insert(insert_idx, gap);
            }
        }
    }

    let mut checksum = 0;

    for (id, span) in files.iter().enumerate() {
        for i in span.start..=span.end {
            checksum += id * i;
        }
    }

    checksum
}

fn solve<Part: AocPart>(input: &str) -> usize {
    if Part::is_two() {
        return solve_b(input);
    }

    let mut disk: Vec<Option<usize>> = Vec::new();

    let mut id = 0;
    for (i, digit) in input.trim().chars().enumerate() {
        if i % 2 == 0 {
            for _ in 0..digit.to_digit(10).unwrap() {
                disk.push(Some(id));
            }

            id += 1;
        } else {
            for _ in 0..digit.to_digit(10).unwrap() {
                disk.push(None);
            }
        }
    }

    // compact

    let mut free = disk.iter().position(|c| c.is_none()).unwrap();

    loop {
        while disk.last().unwrap().is_none() {
            disk.pop();
        }
        let last = disk.pop().unwrap().unwrap();

        if free < disk.len() {
            disk[free] = Some(last);
        } else {
            disk.push(Some(last));
        }

        if let Some(new_free) = disk[free..].iter().position(|c| c.is_none()) {
            free = free + new_free;
        } else {
            break;
        }
    }

    let mut checksum = 0;

    for (i, c) in disk.iter().enumerate() {
        if let Some(c) = c {
            checksum += i * c;
        }
    }

    checksum
}

aoc_tests! {
    inputs {
        e0 = "2333133121414131402",
    }

    part::One {
        ea0: e0 => 1928,
        ra: @input => 6519155389266,
    }

    part::Two {
        eb0: e0 => 2858,
        rb: @input => 6547228115826,
    }
}
