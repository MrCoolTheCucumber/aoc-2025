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
    let mut lines = get_lines()
        .into_iter()
        .map(|l| l.chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = 0;

    loop {
        let count = remove(&mut lines);
        ans += count;

        if count == 0 {
            break;
        }
    }

    println!("{ans}");
}

fn remove(lines: &mut Vec<Vec<bool>>) -> i32 {
    let m = lines.len() as i32;
    let n = lines[0].len() as i32;

    let dirs = [
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let mut ans = 0;
    let mut to_remove = Vec::new();

    for i in 0..m {
        for j in 0..n {
            if !lines[i as usize][j as usize] {
                continue;
            }

            let mut count = 0;
            for &(dx, dy) in &dirs {
                let x = i + dx;
                let y = j + dy;

                if x >= 0 && x < m && y >= 0 && y < n {
                    if lines[x as usize][y as usize] {
                        count += 1;
                    }
                }
            }

            if count < 4 {
                ans += 1;
                to_remove.push((i as usize, j as usize));
            }
        }
    }

    for (x, y) in to_remove {
        lines[x][y] = false;
    }

    ans
}
