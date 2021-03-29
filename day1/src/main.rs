use std::io::{BufWriter, stdin, stdout, BufReader, BufRead, Write};

fn main() -> std::io::Result<()> {
    let cin = BufReader::new(stdin());
    let cout = &mut BufWriter::new(stdout());
    let mut nums = Vec::new();

    for line in cin.lines() {
        nums.push(line.unwrap().parse::<usize>().unwrap());
    }

    for i in &nums {
        for j in &nums {
            for k in &nums {
                if i + j + k == 2020 {
                    write!(cout, "{}", (i * j * k))?;
                    return Ok(());
                }
            }
        }
    }

    return Ok(());
}
