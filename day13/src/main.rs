use rayon::prelude::*;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let timestamp: i64 = lines[0].parse().unwrap();
    let ids_vec = lines[1]
        .split(',')
        .map(|id| {
            if id == "x" {
                None
            } else {
                Some(id.parse::<i64>().unwrap())
            }
        })
        .enumerate()
        .filter(|(_, id)| *id != None)
        .collect::<Vec<_>>();
    let ids = ids_vec.as_slice();
    let max_id = ids
        .iter()
        .max_by(|x, y| x.1.unwrap().cmp(&y.1.unwrap()))
        .unwrap();

    // Part 1
    let mut min_diff = i64::MAX;
    let mut min_id = i64::MAX;
    for (_, some_id) in ids {
        if let Some(id) = some_id {
            let diff = if timestamp % id == 0 {
                0
            } else {
                (((timestamp / id) + 1) * id) - timestamp
            };
            if diff < min_diff {
                min_diff = diff;
                min_id = *id;
            }
        }
    }

    println!("{}", min_id * min_diff);

    let mut multiples;
    let mut n: i64 = 1;
    let mut old_n = n - 1;
    loop {
        println!("Trying: {}", n);
        multiples = (old_n..n)
            .into_par_iter()
            .map(|i| i * max_id.1.unwrap())
            .filter(|x| {
                let mut res = true;
                for (i, some_id) in ids {
                    if let Some(id) = some_id {
                        res = res && (((*x + (*i as i64) - (max_id.0 as i64)) % *id) == 0);
                    }
                }
                res
            })
            .collect::<Vec<_>>();
        if multiples.len() != 0 {
            break;
        } else {
            old_n = n;
            n *= 2;
        }
    }

    println!("Ans: {}", multiples[0] - max_id.0 as i64);

    Ok(())
}
