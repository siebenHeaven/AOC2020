use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut nums = HashMap::new();
    // 5,1,9,18,13,8,0
    let mut line = String::new();
    let num_lines = io::stdin().lock().read_line(&mut line);
    line = line.trim_end().to_string();
    assert_ne!(num_lines.unwrap(), 0);
    if let Some(mut prev) = line.split(',').enumerate().fold(None, |acc, (i, n)| {
        // println!("{:#?}", (acc, i, n));
        if let Some(x) = acc {
            nums.insert(x, i - 1);
        }
        Some(n.parse::<usize>().unwrap())
    }) {
        let mut count = nums.len();
        while count != (30000000 - 1) {
            if let Some(i) = nums.get_mut(&prev) {
                prev = count - *i;
                *i = count;
            } else {
                nums.insert(prev, count);
                prev = 0;
            }
            count += 1;
        }
        println!("2020th number spoken: {}", prev);
    }
}
