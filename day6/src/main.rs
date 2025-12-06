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
    pt_1();
    pt_2();
}

fn pt_1() {
    let lines = get_lines();
    let n = lines.len();

    let parsed = lines
        .clone()
        .into_iter()
        .take(n - 1)
        .map(|l| {
            l.trim()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut cols = vec![Vec::<u64>::new(); parsed[0].len()];

    for row in &parsed {
        for (i, &val) in row.iter().enumerate() {
            cols[i].push(val);
        }
    }

    let mut ans = 0;

    for (op, col) in lines.last().unwrap().split_whitespace().zip(cols) {
        match op {
            "*" => {
                let mut tot = 1;
                for c in col {
                    tot *= c;
                }
                ans += tot;
            }
            "+" => {
                let mut tot = 0;
                for c in col {
                    tot += c;
                }
                ans += tot;
            }
            _ => unreachable!(),
        }
    }

    println!("{ans}");
}

fn pt_2() {
    let lines = get_lines();
    let n = lines.len();

    let mut ops = Vec::new();
    let mut ops_raw = Vec::new();
    let mut parsed = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        // ops
        if idx == n - 1 {
            ops_raw = line.chars().collect();
            ops = line.split_whitespace().collect::<Vec<_>>();
            continue;
        }

        let chars = line.chars().collect::<Vec<_>>();
        parsed.push(chars);
    }

    let mut col_widths = Vec::new();
    let mut iter = ops_raw.iter().peekable();
    loop {
        let Some(_) = iter.next() else {
            break;
        };

        let mut len = 1;
        while let Some(' ') = iter.peek() {
            iter.next();
            len += 1;
        }
        col_widths.push(len as usize);
    }

    let cwn = col_widths.len();
    col_widths.iter_mut().take(cwn - 1).for_each(|w| *w -= 1);

    let width = parsed[0].len();
    let depth = parsed.len();
    let mut ops = ops.iter();
    let mut col_widths = col_widths.iter();

    let mut ans = 0;
    let mut col = Vec::new();
    let mut curr_col_idx = 0;
    let mut curr_col_width = *col_widths.next().unwrap();
    let mut skip_next = false;

    for i in 0..width {
        if skip_next {
            skip_next = false;
            curr_col_idx = 0;
            curr_col_width = *col_widths.next().unwrap_or(&0);
            continue;
        }

        // go down
        let mut num = String::new();
        for j in 0..depth {
            num.push(parsed[j][i]);
        }

        let num = num.trim().parse::<u64>().unwrap();
        col.push(num);

        // calc
        if curr_col_idx == curr_col_width - 1 {
            let op = *ops.next().unwrap();
            match op {
                "*" => {
                    let mut tot = 1;
                    for c in &col {
                        tot *= *c;
                    }
                    ans += tot;
                }
                "+" => {
                    let mut tot = 0;
                    for c in &col {
                        tot += *c;
                    }
                    ans += tot;
                }
                _ => unreachable!(),
            }

            col.clear();
            skip_next = true;
        }

        curr_col_idx += 1;
    }

    println!("{ans}");
}
