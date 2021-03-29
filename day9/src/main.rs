use std::collections::VecDeque;
use std::io::{self, BufRead};

fn is_sum(q: &mut VecDeque<&usize>, v: usize) -> bool {
    for i in 0..q.len() {
        for j in 0..q.len() {
            if i == j {
                continue;
            } else {
                if **q.get(i).unwrap() + **q.get(j).unwrap() == v {
                    return true;
                }
            }
        }
    }
    false
}

fn check_contig_sum(xmas: &Vec<usize>, start: usize, target: usize) -> Option<usize> {
    let mut acc: usize = 0;
    for i in start..xmas.len() {
        if xmas[i] > target {
            return None;
        } else if acc == target {
            return Some(i - 1);
        } else {
            // intentionally empty
        }

        acc += xmas[i];
    }
    None
}

fn main() -> io::Result<()> {
    let pre_size: usize = 25;

    let xmas: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect();

    let pre: VecDeque<_> = xmas.iter().take(pre_size).collect();

    //println!("{:#?}", pre);

    let mut find_non_sum = xmas.iter().skip(pre_size).scan(pre, |mut q, x| {
        if !is_sum(q, *x) {
            println!("{} is not sum", x);
            Some(x)
        } else {
            q.pop_front();
            q.push_back(x);
            None
        }
    });

    let mut temp;
    loop {
        temp = find_non_sum.next();
        if temp.is_some() {
            break;
        }
    }

    if let Some(&non_sum) = temp {
        for i in 0..xmas.len() {
            if let Some(j) = check_contig_sum(&xmas, i, non_sum) {
                println!("{} + {} = {}", xmas[i], xmas[j], xmas[i] + xmas[j]);
                break;
            }
        }
    }

    Ok(())
}
