use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    c: Color,
    id: (i64, i64),
}

impl Tile {
    fn new() -> Self {
        Tile {
            c: Color::White,
            id: (0, 0),
        }
    }

    fn invert(&mut self) {
        self.c = match self.c {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    fn get_neighbour_id(&self, n: &str) -> (i64, i64) {
        let diff = match n {
            "e" => (2, 0),
            "w" => (-2, 0),
            "se" => (1, 2),
            "sw" => (-1, 2),
            "ne" => (1, -2),
            "nw" => (-1, -2),
            _ => unreachable!(),
        };
        (self.id.0 + diff.0, self.id.1 + diff.1)
    }
}

fn part1(lines: Vec<String>) -> (usize, HashMap<(i64, i64), Tile>) {
    let mut tiles = HashMap::new();
    let ref_tile = Tile::new();
    tiles.insert((0, 0), ref_tile);
    for line in &lines {
        if line.is_empty() {
            continue;
        }
        let mut i = 0;
        let mut curr_id = (0, 0);
        while i < line.len() {
            let curr = *tiles.get(&curr_id).unwrap();
            let c = line.chars().nth(i).unwrap();
            let n = match c {
                'e' => "e",
                'w' => "w",
                's' => {
                    i += 1;
                    match line.chars().nth(i).unwrap() {
                        'e' => "se",
                        'w' => "sw",
                        _ => unreachable!(),
                    }
                }
                'n' => {
                    i += 1;
                    match line.chars().nth(i).unwrap() {
                        'e' => "ne",
                        'w' => "nw",
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            };
            let next_id = curr.get_neighbour_id(n);
            tiles.entry(next_id).or_insert(Tile {
                id: next_id,
                ..Tile::new()
            });
            curr_id = next_id;
            i += 1;
        }
        // println!(
        //     "{:?} inverted from {:#?}",
        //     curr_id,
        //     tiles.get(&curr_id).unwrap().c
        // );
        tiles.get_mut(&curr_id).unwrap().invert();
    }

    // dbg!(&tiles);

    (
        tiles.iter().fold(0usize, |acc, (_, v)| match v.c {
            Color::White => acc,
            Color::Black => acc + 1,
        }),
        tiles,
    )
}

fn part2(tiles: &mut HashMap<(i64, i64), Tile>) -> usize {
    for day in 0..100 {
        let none_tile_ids = |tile: &Tile| {
            ["e", "w", "se", "sw", "ne", "nw"]
                .iter()
                .filter_map(|n| {
                    let n_id = tile.get_neighbour_id(n);
                    if tiles.contains_key(&n_id) {
                        None
                    } else {
                        Some(n_id)
                    }
                })
                .collect::<Vec<_>>()
        };
        let to_insert: Vec<(i64, i64)> = tiles
            .iter()
            .map(|(_, tile)| none_tile_ids(tile))
            .flatten()
            .collect();
        for &id in &to_insert {
            tiles.insert(id, Tile { id, ..Tile::new() });
        }
        let black_count = |tile: &Tile| {
            ["e", "w", "se", "sw", "ne", "nw"]
                .iter()
                .filter_map(|n| {
                    let n_id = tile.get_neighbour_id(n);
                    tiles.get(&n_id)
                })
                .fold(0usize, |acc, tile| {
                    if let Color::Black = tile.c {
                        acc + 1
                    } else {
                        acc
                    }
                })
        };
        let to_invert: Vec<_> = tiles
            .iter()
            .filter_map(|(&id, tile)| match tile.c {
                Color::White => {
                    if black_count(tile) == 2 {
                        Some(id)
                    } else {
                        None
                    }
                }
                Color::Black => {
                    let bc = black_count(tile);
                    if (bc == 0) || (bc > 2) {
                        Some(id)
                    } else {
                        None
                    }
                }
            })
            .collect();

        for tile in &to_invert {
            tiles.get_mut(&tile).unwrap().invert();
        }

        // println!(
        //     "Day {}: {}",
        //     day + 1,
        //     tiles.iter().fold(0usize, |acc, (_, v)| match v.c {
        //         Color::White => acc,
        //         Color::Black => acc + 1,
        //     })
        // )
    }

    tiles.iter().fold(0usize, |acc, (_, v)| match v.c {
        Color::White => acc,
        Color::Black => acc + 1,
    })
}

fn main() {
    let lines: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let (num_black, mut tiles) = part1(lines);
    println!("Part1: {}", num_black);
    println!("Part2: {}", part2(&mut tiles));
}

#[test]
fn test_part1() {
    let lines = "sesenwnenenewseeswwswswwnenewsewsw\n\
    neeenesenwnwwswnenewnwwsewnenwseswesw\n\
    seswneswswsenwwnwse\n\
    nwnwneseeswswnenewneswwnewseswneseene\n\
    swweswneswnenwsewnwneneseenw\n\
    eesenwseswswnenwswnwnwsewwnwsene\n\
    sewnenenenesenwsewnenwwwse\n\
    wenwwweseeeweswwwnwwe\n\
    wsweesenenewnwwnwsenewsenwwsesesenwne\n\
    neeswseenwwswnwswswnw\n\
    nenwswwsewswnenenewsenwsenwnesesenew\n\
    enewnwewneswsewnwswenweswnenwsenwsw\n\
    sweneswneswneneenwnewenewwneswswnese\n\
    swwesenesewenwneswnwwneseswwne\n\
    enesenwswwswneneswsenwnewswseenwsese\n\
    wnwnesenesenenwwnenwsewesewsesesew\n\
    nenewswnwewswnenesenwnesewesw\n\
    eneswnwswnwsenenwnwnwwseeswneewsenese\n\
    neswnwewnwnwseenwseesewsenwsweewe\n\
    wseweeenwnesenwwwswnew\n\
    "
    .split('\n')
    .map(|line| line.to_string())
    .collect();

    assert_eq!(10, part1(lines).0);
}

#[test]
fn test_part2() {
    let lines = "sesenwnenenewseeswwswswwnenewsewsw\n\
    neeenesenwnwwswnenewnwwsewnenwseswesw\n\
    seswneswswsenwwnwse\n\
    nwnwneseeswswnenewneswwnewseswneseene\n\
    swweswneswnenwsewnwneneseenw\n\
    eesenwseswswnenwswnwnwsewwnwsene\n\
    sewnenenenesenwsewnenwwwse\n\
    wenwwweseeeweswwwnwwe\n\
    wsweesenenewnwwnwsenewsenwwsesesenwne\n\
    neeswseenwwswnwswswnw\n\
    nenwswwsewswnenenewsenwsenwnesesenew\n\
    enewnwewneswsewnwswenweswnenwsenwsw\n\
    sweneswneswneneenwnewenewwneswswnese\n\
    swwesenesewenwneswnwwneseswwne\n\
    enesenwswwswneneswsenwnewswseenwsese\n\
    wnwnesenesenenwwnenwsewesewsesesew\n\
    nenewswnwewswnenesenwnesewesw\n\
    eneswnwswnwsenenwnwnwwseeswneewsenese\n\
    neswnwewnwnwseenwseesewsenwsweewe\n\
    wseweeenwnesenwwwswnew\n\
    "
    .split('\n')
    .map(|line| line.to_string())
    .collect();

    assert_eq!(2208, part2(&mut part1(lines).1));
}
