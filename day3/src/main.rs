use std::{
    fs::File,
    io::{BufRead, BufReader},
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
    let mut max_jolt = 0;

    for l in lines {
        let nums = l
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        let max = largest_subseq(&nums);
        max_jolt += max;
    }

    println!("{max_jolt}");
}

fn largest_subseq(digits: &[u32]) -> u64 {
    let mut res = String::new();
    let mut cur_idx = 0;
    let n = digits.len();

    for i in 0..12 {
        let limit = n - 11 - i;

        let wd = &digits[cur_idx..limit];
        let max = wd.iter().max().unwrap();
        let pos = wd.iter().position(|d| d == max).unwrap();

        res.push_str(&max.to_string());
        cur_idx += pos + 1;
    }

    res.parse::<u64>().unwrap()
}
