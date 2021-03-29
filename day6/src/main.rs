use std::io::{stdin, BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let cin = BufReader::new(stdin());
    let mut lines = Vec::new();
    let mut count = 0;

    for l in cin.lines() {
        let line = l.unwrap();
        lines.push(line);
    }

    let mut i = 0;
    loop {
        if i >= lines.len() {
            break;
        }

        let mut group_bitmap: usize = 0xFFFFFFFF;
        // Group
        while !lines[i].is_empty() {
            // Person
            let mut person_bitmap: usize = 0;

            for c in lines[i].as_bytes() {
                person_bitmap |= 1 << (c - b'a');
            }

            group_bitmap &= person_bitmap;

            i += 1;
        }

        println!("Group 1 has {}", group_bitmap.count_ones());
        count += group_bitmap.count_ones();
        // Skip over empty line
        i += 1;
    }

    println!("count: {}\n", count);
    return Ok(());
}
