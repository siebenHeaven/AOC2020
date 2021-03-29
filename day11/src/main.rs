use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut grids = Vec::new();
    grids.push(
        io::stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<_>>(),
    );

    let height = grids[0].len();
    let width = grids[0][0].len();
    grids.push(grids[0].clone());

    let mut flag = 0;

    loop {
        for i in 0..height {
            for j in 0..width {
                match grids[flag].nth(j) {
                    "." => _,
                    "L" | "#" => {
                        let mut occ_count = 0;
                        for y in i-1..i+1 {
                            for x in j-1..j+1 {
                                if grids[y].as_bytes()[x] == "#" {
                                    occ_count += 1;
                                }
                            }
                        }
                        if row[j] == "L" && occ_count == 0 {
                            grids[]
                        }
                    }
                }
            }
        }
    }
    

    Ok(())
}
