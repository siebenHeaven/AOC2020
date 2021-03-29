use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn part1(lines: Vec<String>) -> usize {
    let mut alr_ingrset_map: HashMap<String, HashSet<String>> = HashMap::new();
    let mut ingr_count_map: HashMap<String, usize> = HashMap::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        // "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"
        let (ingrs, alrs) = line.splitn(2, " (contains ").collect_tuple().unwrap();
        let ingrset: HashSet<_> = ingrs
            .split(' ')
            .map(|s| {
                if let Some(count) = ingr_count_map.get_mut(s) {
                    *count += 1;
                } else {
                    ingr_count_map.insert(s.to_string(), 1);
                }
                s.to_string()
            })
            .collect();

        alrs.strip_suffix(')').unwrap().split(", ").for_each(|alr| {
            if let Some(prev_set) = alr_ingrset_map.get_mut(alr) {
                *prev_set = prev_set
                    .intersection(&ingrset)
                    .map(|s| s.to_string())
                    .collect();
            } else {
                alr_ingrset_map.insert(alr.to_string(), ingrset.clone());
            }
        })
    }

    println!("{:#?}", alr_ingrset_map);

    let ingrs_with_alr: HashSet<_> = alr_ingrset_map.iter().fold(HashSet::new(), |acc, (_, v)| {
        acc.union(v).map(|s| s.to_string()).collect()
    });

    ingr_count_map
        .iter()
        .filter(|(k, _)| !ingrs_with_alr.contains(k.as_str()))
        .fold(0, |acc, (_, v)| acc + v)
}

fn part2(lines: Vec<String>) -> usize {
    let mut alr_ingrset_map: HashMap<String, HashSet<String>> = HashMap::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        // "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"
        let (ingrs, alrs) = line.splitn(2, " (contains ").collect_tuple().unwrap();
        let ingrset: HashSet<_> = ingrs.split(' ').map(|s| s.to_string()).collect();

        alrs.strip_suffix(')').unwrap().split(", ").for_each(|alr| {
            if let Some(prev_set) = alr_ingrset_map.get_mut(alr) {
                *prev_set = prev_set
                    .intersection(&ingrset)
                    .map(|s| s.to_string())
                    .collect();
            } else {
                alr_ingrset_map.insert(alr.to_string(), ingrset.clone());
            }
        })
    }

    // println!("{:#?}", alr_ingrset_map);

    let ingrs_with_alr: HashSet<_> = alr_ingrset_map.iter().fold(HashSet::new(), |acc, (_, v)| {
        acc.union(v).map(|s| s.to_string()).collect()
    });

    let mut dangerous_ingr_vec = ingrs_with_alr.iter().collect::<Vec<_>>();
    dangerous_ingr_vec.sort();

    println!("cljf,frtfg,vvfjj,qmrps,hvnkk,qnvx,cpxmpc,qsjszn");

    0
}

fn main() {
    let lines: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    println!("part1: {}", part1(lines.clone()));
    println!("part2: {}", part2(lines));
}

#[test]
fn test_part1() {
    let lines = vec![
        "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
        "trh fvjkl sbzzf mxmxvkd (contains dairy)",
        "sqjhc fvjkl (contains soy)",
        "sqjhc mxmxvkd sbzzf (contains fish)",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();

    assert_eq!(5, part1(lines));
}

#[test]
fn test_part2() {
    let lines = vec![
        "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
        "trh fvjkl sbzzf mxmxvkd (contains dairy)",
        "sqjhc fvjkl (contains soy)",
        "sqjhc mxmxvkd sbzzf (contains fish)",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();

    assert_eq!(0, part2(lines));
}
