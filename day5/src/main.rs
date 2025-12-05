use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

fn get_lines() -> Vec<String> {
    let file = File::open("../input.txt").expect("no input file found");
    BufReader::new(file)
        .lines()
        .map(|l| l.expect("unable to read line"))
        .collect::<Vec<_>>()
}

fn main() {
    let lines = get_lines();

    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut fresh = 0;

    for line in lines {
        if line.contains("-") {
            let spl = line
                .split('-')
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            ranges.push(spl[0]..=spl[1]);
            continue;
        }

        if line.is_empty() {
            break;
        }
    }

    ranges = merge_ranges(ranges);

    for r in ranges {
        fresh += (r.end() - r.start()) + 1;
    }

    println!("{fresh}");
}

pub fn merge_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    if ranges.is_empty() {
        return Vec::new();
    }

    ranges.sort_by_key(|r| *r.start());

    let mut merged: Vec<RangeInclusive<u64>> = Vec::new();
    let mut curr = ranges[0].clone();

    for next in ranges.into_iter().skip(1) {
        if next.start() <= curr.end() {
            let end = *std::cmp::max(curr.end(), next.end());
            curr = *curr.start()..=end;
        } else {
            merged.push(curr);
            curr = next;
        }
    }

    merged.push(curr);
    merged
}
