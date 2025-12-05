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
    let inputs = lines
        .into_iter()
        .map(|l| {
            let spl = l.split_at(1);
            let val = spl.1.parse::<i32>().unwrap();

            if spl.0 == "R" {
                val
            } else {
                -val
            }
        })
        .collect::<Vec<_>>();

    let mut curr = 50;
    let mut pass2 = 0;

    for i in inputs {
        let delta = i.signum();
        let mut diff = i.abs();

        while diff > 0 {
            curr += delta;
            diff -= 1;

            if curr == -1 {
                curr = 99;
            }

            if curr == 100 {
                curr = 0;
            }

            if curr == 0 {
                pass2 += 1;
            }
        }
    }

    println!("{pass2}");
}
