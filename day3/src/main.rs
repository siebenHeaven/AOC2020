use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn get_encountered_trees(x_stride: usize, y_stride: usize, grid: &Vec<String>) -> usize {
    let mut trees_encountered = 0;
    let pattern_len=grid[0].len();
    let mut x = 0;
    let mut y = 0;

    loop {
        // println!("x: {}, y:{}", x, y);
        if grid[y].chars().nth(x).unwrap() == '#' {
            trees_encountered += 1;
        }
        x = (x + x_stride) % pattern_len;
        y += y_stride;
        if y >= grid.len() {
            break;
        }
    }

    return trees_encountered;
}

fn main() -> std::io::Result<()> {
    let cin = BufReader::new(stdin());
    let cout = &mut BufWriter::new(stdout());
    let mut grid = Vec::new();

    for l in cin.lines() {
        let line = l.unwrap();
        grid.push(line);
    }

    // Part 1
    write!(
        cout,
        "Trees encountered: {}\n",
        get_encountered_trees(3, 1, &grid)
    )?;

    // Part 2
    let mut acc_trees_encountered = 1;
    let strides = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for stride in &strides {
        acc_trees_encountered *= get_encountered_trees(stride.0, stride.1, &grid);
    }
    write!(
        cout,
        "Accumulation of trees encountered: {}\n",
        acc_trees_encountered
    )?;

    return Ok(());
}
