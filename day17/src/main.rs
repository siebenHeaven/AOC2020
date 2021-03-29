use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone)]
enum State {
    Active,
    Inactive,
}

fn part1(lines: Vec<String>, num_cycles: usize) -> usize {
    let mut cubes: HashMap<(i64, i64, i64), State> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for x in 0..lines[y].len() {
            cubes.insert(
                (x as i64, y as i64, 0i64),
                if line.chars().nth(x).unwrap() == '.' {
                    State::Inactive
                } else {
                    State::Active
                },
            );
        }
    }

    let get_neighbours = |cube: (i64, i64, i64)| -> HashSet<_> {
        let mut ret = HashSet::new();
        for &x in &[-1, 0, 1] {
            for &y in &[-1, 0, 1] {
                for &z in &[-1, 0, 1] {
                    if !((x == 0) && (y == 0) && (z == 0)) {
                        let n = (cube.0 + x, cube.1 + y, cube.2 + z);
                        ret.insert(n);
                    }
                }
            }
        }
        ret
    };

    for cycle in 0..num_cycles {
        // println!(
        //     "State before cycle[{}] num_cubes[{}]:\n{:#?}",
        //     cycle,
        //     cubes.len(),
        //     cubes
        // );
        //        thread::sleep(time::Duration::from_secs(10));
        let to_add: HashSet<_> = cubes
            .keys()
            .map(|&cube| get_neighbours(cube))
            .flatten()
            .collect();

        for cube in to_add {
            cubes.entry(cube).or_insert(State::Inactive);
        }

        let get_active_neighbours_count = |cube: (i64, i64, i64)| -> usize {
            get_neighbours(cube)
                .iter()
                .filter(|&n| cubes.contains_key(n))
                .filter(|&n| matches!(cubes.get(n).unwrap(), State::Active))
                .count()
        };

        let cubes_to_invert: HashSet<_> = cubes
            .keys()
            .filter(|&cube| {
                let active_neighbours_count = get_active_neighbours_count(*cube);
                match cubes.get(cube).unwrap() {
                    State::Active => {
                        (active_neighbours_count != 2) && (active_neighbours_count != 3)
                    }
                    State::Inactive => active_neighbours_count == 3,
                }
            })
            .cloned()
            .collect();

        for cube in cubes_to_invert {
            cubes.entry(cube).and_modify(|state| {
                let new_state = match *state {
                    State::Active => State::Inactive,
                    State::Inactive => State::Active,
                };
                *state = new_state;
            });
        }

        // println!(
        //     "State after cycle[{}] num_cubes[{}]:\n{:#?}",
        //     cycle,
        //     cubes.len(),
        //     cubes
        // );
    }

    cubes
        .values()
        .filter(|&state| matches!(state, State::Active))
        .count()
}

fn part2(lines: Vec<String>, num_cycles: usize) -> usize {
    let mut cubes: HashMap<(i64, i64, i64, i64), State> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for x in 0..lines[y].len() {
            cubes.insert(
                (x as i64, y as i64, 0i64, 0i64),
                if line.chars().nth(x).unwrap() == '.' {
                    State::Inactive
                } else {
                    State::Active
                },
            );
        }
    }

    let get_neighbours = |cube: (i64, i64, i64, i64)| -> HashSet<_> {
        let mut ret = HashSet::new();
        for &x in &[-1, 0, 1] {
            for &y in &[-1, 0, 1] {
                for &z in &[-1, 0, 1] {
                    for &w in &[-1, 0, 1] {
                        if !((x == 0) && (y == 0) && (z == 0) && (w == 0)) {
                            let n = (cube.0 + x, cube.1 + y, cube.2 + z, cube.3 + w);
                            ret.insert(n);
                        }
                    }
                }
            }
        }
        ret
    };

    for cycle in 0..num_cycles {
        // println!(
        //     "State before cycle[{}] num_cubes[{}]:\n{:#?}",
        //     cycle,
        //     cubes.len(),
        //     cubes
        // );
        //        thread::sleep(time::Duration::from_secs(10));
        let to_add: HashSet<_> = cubes
            .keys()
            .map(|&cube| get_neighbours(cube))
            .flatten()
            .collect();

        for cube in to_add {
            cubes.entry(cube).or_insert(State::Inactive);
        }

        let get_active_neighbours_count = |cube: (i64, i64, i64, i64)| -> usize {
            get_neighbours(cube)
                .iter()
                .filter(|&n| cubes.contains_key(n))
                .filter(|&n| matches!(cubes.get(n).unwrap(), State::Active))
                .count()
        };

        let cubes_to_invert: HashSet<_> = cubes
            .keys()
            .filter(|&cube| {
                let active_neighbours_count = get_active_neighbours_count(*cube);
                match cubes.get(cube).unwrap() {
                    State::Active => {
                        (active_neighbours_count != 2) && (active_neighbours_count != 3)
                    }
                    State::Inactive => active_neighbours_count == 3,
                }
            })
            .cloned()
            .collect();

        for cube in cubes_to_invert {
            cubes.entry(cube).and_modify(|state| {
                let new_state = match *state {
                    State::Active => State::Inactive,
                    State::Inactive => State::Active,
                };
                *state = new_state;
            });
        }

        // println!(
        //     "State after cycle[{}] num_cubes[{}]:\n{:#?}",
        //     cycle,
        //     cubes.len(),
        //     cubes
        // );
    }

    cubes
        .values()
        .filter(|&state| matches!(state, State::Active))
        .count()
}

fn main() {
    let lines: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    println!("{}", part1(lines.clone(), 6));
    println!("{}", part2(lines, 6));
}

#[test]
fn test_part1() {
    let lines: Vec<String> = ".#.\n\
                ..#\n\
                ###"
    .split('\n')
    .map(|s| s.to_string())
    .collect();

    assert_eq!(11, part1(lines.clone(), 1));
    assert_eq!(21, part1(lines.clone(), 2));
    assert_eq!(38, part1(lines.clone(), 3));
    assert_eq!(112, part1(lines, 6));
}

#[test]
fn test_part2() {
    let lines: Vec<String> = ".#.\n\
                ..#\n\
                ###"
    .split('\n')
    .map(|s| s.to_string())
    .collect();

    assert_eq!(848, part2(lines, 6));
}
