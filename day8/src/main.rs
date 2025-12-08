use std::{
    cmp::Reverse,
    collections::HashSet,
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

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        let pows = (dx.pow(2) + dy.pow(2) + dz.pow(2)) as f64;
        pows.sqrt()
    }
}

fn main() {
    let lines = get_lines();
    let points = lines
        .into_iter()
        .map(|l| {
            let mut itr = l.split(",");
            let x = itr.next().unwrap().parse().unwrap();
            let y = itr.next().unwrap().parse().unwrap();
            let z = itr.next().unwrap().parse().unwrap();
            Point::new(x, y, z)
        })
        .collect::<Vec<_>>();

    let mut seen = HashSet::new();
    let mut distances = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate() {
            if i == j {
                continue;
            }

            if seen.contains(&(i, j)) | seen.contains(&(j, i)) {
                continue;
            }

            distances.push((i, j, p1.distance(&p2)));
            seen.insert((i, j));
            seen.insert((j, i));
        }
    }
    distances.sort_by(|a, b| a.2.total_cmp(&b.2));

    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    let mut last_pair = None;

    for d in distances.iter()
    /*.take(1000)*/
    {
        let p1 = points[d.0];
        let p2 = points[d.1];

        let s0 = circuits.iter().position(|s| s.contains(&d.0));
        let s1 = circuits.iter().position(|s| s.contains(&d.1));

        match (s0, s1) {
            (None, None) => {
                let set = HashSet::from([d.0, d.1]);
                circuits.push(set);
                last_pair = Some((p1, p2));
            }
            (Some(idx), None) | (None, Some(idx)) => {
                circuits[idx].insert(d.0);
                circuits[idx].insert(d.1);
                last_pair = Some((p1, p2));
            }
            (Some(mut idx1), Some(mut idx2)) => {
                if idx1 == idx2 {
                    continue;
                }

                // make sure idx1 is larger
                if idx2 > idx1 {
                    std::mem::swap(&mut idx1, &mut idx2);
                }

                let removed = circuits.remove(idx1);
                circuits[idx2].extend(removed);

                if circuits[idx2].len() == points.len() {
                    last_pair = Some((p1, p2));
                }
            }
        }
    }

    circuits.sort_by_key(|s| Reverse(s.len()));

    let mut _ans = 1;
    for c in circuits.iter().take(3) {
        _ans *= c.len();
    }

    let last_pair = last_pair.unwrap();
    println!("{}", last_pair.0.x * last_pair.1.x);
}
