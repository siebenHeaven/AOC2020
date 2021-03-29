use std::collections::HashMap;
use std::io::{self, BufRead};

fn count_paths(adapters: &[usize], results: &mut HashMap<usize, usize>, index: usize) -> usize {
    if index >= adapters.len() - 1 {
        1
    } else if let Some(res) = results.get(&index) {
        *res
    } else {
        let paths: usize = adapters
            .iter()
            .skip(index + 1)
            .take(3)
            .enumerate()
            .filter(|(_, a)| **a - adapters[index] <= 3)
            .map(|(i, _)| count_paths(adapters, results, index + i + 1))
            .sum::<usize>();

        results.insert(index, paths);

        paths
    }
}

fn main() -> io::Result<()> {
    let mut joltage_adapters = vec![];

    // Wall charger
    joltage_adapters.push(0);

    // Adapters in bag
    let mut bag_joltage_adapters: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect();

    joltage_adapters.append(&mut bag_joltage_adapters);
    joltage_adapters.sort_unstable();

    // Built-in adapter
    joltage_adapters.push(joltage_adapters[joltage_adapters.len() - 1] + 3);

    // println!("{:#?}", joltage_adapters);

    // Part 1
    let mut diffs = HashMap::new();
    for joltage_pair in joltage_adapters.as_slice().windows(2) {
        let j1 = joltage_pair[0];
        let j2 = joltage_pair[1];

        let diff = j2 - j1;

        if let Some(count) = diffs.get_mut(&diff) {
            *count += 1;
        } else {
            diffs.insert(diff, 1);
        }
    }

    // println!("{:#?}", diffs);
    if let Some(v1) = diffs.get(&1_usize) {
        if let Some(v3) = diffs.get(&3_usize) {
            println!("v1 * v3 = {}", v1 * v3);
        }
    }

    let mut mem = HashMap::new();
    let num_possible_ways = count_paths(&joltage_adapters, &mut mem, 0);

    println!("Num possible ways: {}", num_possible_ways);

    Ok(())
}
