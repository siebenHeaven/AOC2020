use std::io::{self, BufRead};

#[derive(Debug)]
enum Direction {
    E,
    W,
    N,
    S,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'N' => Direction::N,
            'S' => Direction::S,
            'E' => Direction::E,
            'W' => Direction::W,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn get_offsets(&self) -> (i64, i64) {
        match self {
            Direction::E => (1, 0),
            Direction::W => (-1, 0),
            Direction::N => (0, 1),
            Direction::S => (0, -1),
        }
    }

    fn left(&self) -> Self {
        match self {
            Direction::E => Direction::N,
            Direction::W => Direction::S,
            Direction::N => Direction::W,
            Direction::S => Direction::E,
        }
    }

    fn right(&self) -> Self {
        match self {
            Direction::E => Direction::S,
            Direction::W => Direction::N,
            Direction::N => Direction::E,
            Direction::S => Direction::W,
        }
    }
}

fn main() {
    let instructions = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let temp = line.unwrap();
            (
                temp.chars().nth(0).unwrap(),
                temp[1..].parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    //    println!("{:#?}", instructions);

    {
        // Part 1
        let mut dir = Direction::E;
        let execute = |mut ship: (i64, i64), ins: &(char, i64)| {
            // println!("dir: {:#?}, ship: {:#?}", dir, acc);
            match ins.0 {
                'F' => {
                    let offsets = dir.get_offsets();
                    ship.0 += ins.1 * offsets.0;
                    ship.1 += ins.1 * offsets.1;
                    ship
                }
                'L' => {
                    assert!(ins.1 % 90 == 0);
                    for _ in 0..ins.1 / 90 {
                        dir = dir.left();
                    }
                    ship
                }
                'R' => {
                    assert!(ins.1 % 90 == 0);
                    for _ in 0..ins.1 / 90 {
                        dir = dir.right();
                    }
                    ship
                }
                _ => {
                    let temp_dir = Direction::from(ins.0);
                    let offsets = temp_dir.get_offsets();
                    ship.0 += ins.1 * offsets.0;
                    ship.1 += ins.1 * offsets.1;
                    ship
                }
            }
        };
        let final_pos = instructions.iter().fold((0i64, 0i64), execute);
        println!("Part 1: {:#?}", final_pos);
    }

    {
        // Part 2
        let mut way_point = (10, 1);
        let execute = |mut ship: (i64, i64), ins: &(char, i64)| {
            // println!("waypoint: {:#?}, ship: {:#?}", way_point, acc);
            match ins.0 {
                'F' => {
                    ship.0 += ins.1 * way_point.0;
                    ship.1 += ins.1 * way_point.1;
                    ship
                }
                'L' => {
                    assert!(ins.1 % 90 == 0);
                    for _ in 0..ins.1 / 90 {
                        let temp_x = -1 * way_point.1;
                        let temp_y = way_point.0;
                        way_point.0 = temp_x;
                        way_point.1 = temp_y;
                    }
                    ship
                }
                'R' => {
                    assert!(ins.1 % 90 == 0);
                    for _ in 0..ins.1 / 90 {
                        let temp_x = way_point.1;
                        let temp_y = -1 * way_point.0;
                        way_point.0 = temp_x;
                        way_point.1 = temp_y;
                    }
                    ship
                }
                _ => {
                    let temp_dir = Direction::from(ins.0);
                    let offsets = temp_dir.get_offsets();
                    way_point.0 += ins.1 * offsets.0;
                    way_point.1 += ins.1 * offsets.1;
                    ship
                }
            }
        };
        let final_pos = instructions.iter().fold((0i64, 0i64), execute);

        println!("Part 2: {:#?}", final_pos);
    }
}
