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
    let lines = get_lines()
        .into_iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let n = lines.len();
    let m = lines[0].len();

    let mut tl = vec![vec![0; m]; n];

    let s_pos = lines[0].iter().position(|c| *c == 'S').unwrap();
    tl[0][s_pos] = 1;

    for i in 0..n - 1 {
        for j in 0..m {
            if lines[i][j] == '^' {
                tl[i + 1][j - 1] += tl[i][j];
                tl[i + 1][j + 1] += tl[i][j];
            } else {
                tl[i + 1][j] += tl[i][j];
            }
        }
    }

    let timelines = tl.last().unwrap().iter().sum::<u64>();
    println!("{timelines}");
}
