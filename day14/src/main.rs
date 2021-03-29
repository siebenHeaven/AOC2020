use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn part1(lines: Vec<String>) -> u64 {
    let mut mem = HashMap::<u64, u64>::new();
    let bitmask = lines
        .iter()
        .fold((0xFFFFFFFFFu64, 0u64), |mut bitmask, line| {
            if let Some((key, val)) = line.splitn(2, '=').collect_tuple() {
                // println!("{:#?}", (key, val));
                if key.trim() == "mask" {
                    bitmask.0 = u64::from_str_radix(&val.trim().replace("X", "0"), 2).unwrap();
                    bitmask.1 = u64::from_str_radix(&val.trim().replace("X", "1"), 2).unwrap();
                } else {
                    let id = key
                        .trim()
                        .trim_start_matches("mem[")
                        .trim_end_matches("]")
                        .parse()
                        .unwrap();
                    let v = val.trim().parse::<u64>().unwrap() & bitmask.1 | bitmask.0;
                    if let Some(m) = mem.get_mut(&id) {
                        *m = v;
                    } else {
                        mem.insert(id, v);
                    }
                }
            };
            bitmask
        });

    println!("{:#?}\n{:#?}", mem, bitmask);
    mem.values().sum::<u64>()
}

fn generate_addresses(address: u64, bits: &Vec<usize>, n: usize, addresses: &mut Vec<u64>) {
    if n < bits.len() {
        generate_addresses(address & !(1u64 << bits[n]), bits, n + 1, addresses);
        generate_addresses(address | (1u64 << bits[n]), bits, n + 1, addresses);
    } else {
        addresses.push(address);
    }
}

fn part2(lines: Vec<String>) -> u64 {
    let mut mem = HashMap::<u64, u64>::new();
    let bitmask = lines
        .iter()
        .fold((0, Vec::<usize>::new()), |mut bitmask, line| {
            if let Some((key, val)) = line.splitn(2, '=').collect_tuple() {
                // println!("{:#?}", (key, val));
                if key.trim() == "mask" {
                    bitmask.0 = u64::from_str_radix(&val.trim().replace("X", "0"), 2).unwrap();
                    bitmask.1 = val
                        .trim()
                        .match_indices('X')
                        .map(|(x, _)| 35 - x)
                        .collect::<Vec<_>>();
                } else {
                    let address: u64 = key
                        .trim()
                        .trim_start_matches("mem[")
                        .trim_end_matches("]")
                        .parse()
                        .unwrap();
                    let v = val.trim().parse::<u64>().unwrap();
                    let mut addresses = vec![];
                    generate_addresses(address | bitmask.0, &bitmask.1, 0, &mut addresses);
                    for address in addresses {
                        println!("Writing: {} at {:b}", v, address);
                        if let Some(m) = mem.get_mut(&address) {
                            *m = v;
                        } else {
                            mem.insert(address, v);
                        }
                    }
                }
            };
            bitmask
        });

    println!("{:#?}", bitmask);
    mem.values().sum::<u64>()
}

fn main() -> io::Result<()> {
    let lines = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("{}", part1(lines.clone()));

    println!("{}", part2(lines));

    Ok(())
}

#[test]
fn test_part1() {
    let input_lines =
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0"
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
    assert_eq!(165, part1(input_lines));
}

#[test]
fn test_part2() {
    let input_lines =
        "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1"
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
    assert_eq!(208, part2(input_lines));
}
