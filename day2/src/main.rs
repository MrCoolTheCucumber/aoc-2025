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
    let mut lines = get_lines();
    let line = lines.remove(0);
    let ranges = line
        .split(",")
        .into_iter()
        .map(|l| {
            let s: Vec<_> = l.split("-").collect();
            (s[0].parse::<u64>().unwrap(), s[1].parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut invalid = 0;

    for r in ranges {
        for n in r.0..=r.1 {
            let c = n.to_string();
            // if c.len() % 2 != 0 {
            //     continue;
            // }

            // let spl = c.split_at(c.len() / 2);
            // if spl.0 == spl.1 {
            //     invalid += n;
            // }

            if repeating(&c) {
                invalid += n;
            }
        }
    }

    println!("{invalid}");
}

fn repeating(s: &str) -> bool {
    let chars = s.bytes().collect::<Vec<_>>();

    'bruh: for i in 1..=(chars.len() / 2) {
        let mut prev = None;
        for chunk in chars.chunks(i) {
            match prev {
                None => {
                    prev = Some(chunk);
                }
                Some(p) => {
                    if p != chunk {
                        continue 'bruh;
                    }

                    prev = Some(chunk);
                }
            }
        }

        return true;
    }

    false
}
