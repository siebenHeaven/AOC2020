use std::{
    collections::{HashMap, VecDeque},
    io::{self, BufRead},
};

type TileData = Vec<Vec<char>>;

#[derive(Debug)]
struct Tile {
    data: TileData,
    flip: bool,
    rotate: u8,
}

impl Tile {
    fn new(data: TileData, flip: bool, rotate: u8) -> Self {
        Self { data, flip, rotate }
    }

    fn flip(&mut self) {
        self.flip = !self.flip;
    }

    fn rotate(&mut self) {
        self.rotate = (self.rotate + 1) % 4;
    }

    fn get(&self, x: usize, y: usize) -> char {
        let (newx, newy) = self.transform(x, y);

        self.data[newx][newy]
    }

    fn set(&mut self, x: usize, y: usize, val: char) {
        let (newx, newy) = self.transform(x, y);

        self.data[newx][newy] = val;
    }
    fn transform(&self, mut x: usize, mut y: usize) -> (usize, usize) {
        if self.flip {
            x = self.data.len() - x - 1;
        }

        for _ in 0..self.rotate {
            std::mem::swap(&mut x, &mut y);
            x = self.data.len() - x - 1;
        }

        (x, y)
    }
}

fn check_constraints(
    tiles: &HashMap<usize, Tile>,
    image: &Vec<Vec<usize>>,
    x: usize,
    y: usize,
) -> bool {
    let curr_tile = tiles.get(&image[x][y]).unwrap();

    if x > 0 {
        let left_tile = tiles.get(&image[x - 1][y]).unwrap();
        let n = left_tile.data.len();

        for i in 0..n {
            if left_tile.get(n - 1, i) != curr_tile.get(0, i) {
                return false;
            }
        }
    }

    if y > 0 {
        let up_tile = tiles.get(&image[x][y - 1]).unwrap();
        let n = up_tile.data.len();

        for i in 0..n {
            if up_tile.get(i, n - 1) != curr_tile.get(i, 0) {
                return false;
            }
        }
    }

    true
}

fn bfs(
    tiles: &mut HashMap<usize, Tile>,
    mut available_tiles: VecDeque<usize>,
    image: &mut Vec<Vec<usize>>,
    x: usize,
    y: usize,
    square_len: usize,
    d: usize,
) -> bool {
    if x >= square_len || y >= square_len {
        return true;
    }

    let num_tiles = available_tiles.len();
    let mut next_x = x + 1;
    let mut next_y = y;

    if next_x >= square_len {
        next_x = 0;
        next_y += 1;
    }

    for _ in 0..num_tiles {
        let current_tile = available_tiles.pop_front().unwrap();

        for _ in 0..d {
            // print!("\t");
        }
        // println!("Trying with tile {} for [{}][{}]", current_tile, x, y);
        for _ in 0..2 {
            for r in 0..4 {
                // println!("\t{}", 90 * r);
                // rotate 90,180,270
                image[x][y] = current_tile;
                if check_constraints(tiles, image, x, y) {
                    // println!("Inserted {} at [{}][{}]", current_tile, x, y);
                    if bfs(
                        tiles,
                        available_tiles.clone(),
                        image,
                        next_x,
                        next_y,
                        square_len,
                        d + 1,
                    ) {
                        return true;
                    }
                }
                image[x][y] = 0;

                // rotate and try again
                tiles.get_mut(&current_tile).unwrap().rotate();
            }

            // flip and try again
            tiles.get_mut(&current_tile).unwrap().flip();
            // println!("Flipping and trying");
        }
        available_tiles.push_back(current_tile);
    }

    false
}

fn part1_internal(
    lines: Vec<String>,
    tiles: &mut HashMap<usize, Tile>,
) -> (Vec<Vec<usize>>, usize) {
    let mut image: Vec<Vec<usize>> = Vec::new();
    let n = lines.len();
    let mut i = 0;
    while i < n {
        if lines[i].starts_with("Tile ") {
            let current_tile_id = lines[i]
                .strip_prefix("Tile ")
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let mut tile_data = TileData::new();

            while (i + 1 < n) && (!lines[i + 1].is_empty()) {
                i += 1;
                tile_data.push(lines[i].chars().collect());
            }

            println!("Inserting tile: {}", current_tile_id);
            tiles.insert(current_tile_id, Tile::new(tile_data, false, 0));
        }
        i += 1;
    }
    let square_len = f64::from(tiles.len() as i32).sqrt() as usize;
    for _ in 0..square_len {
        let mut temp = Vec::new();
        for _ in 0..square_len {
            temp.push(0);
        }
        image.push(temp);
    }
    let available_tiles: VecDeque<_> = tiles.keys().cloned().collect();
    if false == bfs(tiles, available_tiles, &mut image, 0, 0, square_len, 0) {
        println!("NOT FOUND");
    }
    (image, square_len)
}

fn part1(lines: Vec<String>) -> usize {
    let mut tiles = HashMap::<usize, Tile>::new();
    let (image, square_len) = part1_internal(lines, &mut tiles);
    println!(
        "{}, {}, {}, {}, {}",
        tiles.len(),
        image[0][0],
        image[square_len - 1][0],
        image[0][square_len - 1],
        image[square_len - 1][square_len - 1]
    );
    image[0][0]
        * image[square_len - 1][0]
        * image[0][square_len - 1]
        * image[square_len - 1][square_len - 1]
}

fn part2(lines: Vec<String>) -> usize {
    let mut tiles = HashMap::<usize, Tile>::new();
    let (image, square_len) = part1_internal(lines, &mut tiles);
    let mut final_tile_data = Vec::new();
    // remove borders, hence - 2
    let tile_len = tiles.get(&image[0][0]).unwrap().data.len() - 2;
    let final_tile_len = square_len * tile_len;

    for _ in 0..final_tile_len {
        let mut temp = Vec::new();
        for _ in 0..final_tile_len {
            temp.push(' ');
        }
        final_tile_data.push(temp);
    }

    for i in 0..square_len {
        for j in 0..square_len {
            let tile = tiles.get(&image[i][j]).unwrap();
            for x in 0..tile_len {
                for y in 0..tile_len {
                    final_tile_data[i * tile_len + x][j * tile_len + y] = tile.get(x + 1, y + 1);
                }
            }
        }
    }

    let final_tile = Tile::new(final_tile_data, false, 0);
    let monster = Vec::from([
        "                  # ".chars().collect::<Vec<_>>(),
        "#    ##    ##    ###".chars().collect::<Vec<_>>(),
        " #  #  #  #  #  #   ".chars().collect::<Vec<_>>(),
    ]);
    let monster_tile = Tile::new(monster, false, 0);

    get_water_roughness(final_tile, monster_tile)
}

fn get_water_roughness(mut final_tile: Tile, monster_tile: Tile) -> usize {
    let mut matched_tile = Tile::new(final_tile.data.clone(), final_tile.flip, final_tile.rotate);

    let monster_match = |a: char, b: char| if b == '#' { a == b } else { true };

    for _ in 0..2 {
        for _ in 0..4 {
            for i in 0..(final_tile.data.len() - monster_tile.data.len() + 1) {
                for j in 0..(final_tile.data[0].len() - monster_tile.data[0].len() + 1) {
                    let mut valid = true;
                    for x in 0..monster_tile.data.len() {
                        for y in 0..monster_tile.data[0].len() {
                            if !monster_match(final_tile.get(i + x, j + y), monster_tile.get(x, y))
                            {
                                valid = false;
                                break;
                            } else {
                                // println!(
                                //     "Match [{}] != [{}]",
                                //     final_tile.get(i + x, j + y),
                                //     monster_tile.get(x, y)
                                // );
                            }
                        }
                        if !valid {
                            break;
                        }
                    }
                    if valid {
                        println!("Valid");
                        for x in 0..monster_tile.data.len() {
                            for y in 0..monster_tile.data[0].len() {
                                if monster_tile.get(x, y) == '#' {
                                    matched_tile.set(i + x, j + y, '.');
                                }
                            }
                        }
                    }
                }
            }
            // rotate and try again
            final_tile.rotate();
            matched_tile.rotate();
        }

        // flip and try again
        final_tile.flip();
        matched_tile.flip();
        // println!("Flipping and trying");
    }

    // println!("{:#?}", matched_tile.data);
    matched_tile
        .data
        .into_iter()
        .flatten()
        .filter(|x| *x == '#')
        .fold(0, |acc, x| acc + 1)
}

fn main() {
    let lines = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();

    // println!("{}", part1(lines));
    println!("{}", part2(lines));
}
