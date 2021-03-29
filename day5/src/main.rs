use std::fs::File;
use std::io::{BufRead, BufReader};

fn round_closest(dividend: usize, divisor: usize) -> usize {
    return (dividend + (divisor / 2)) / divisor;
}

fn get_seat_id(encoded: &str) -> usize {
    assert!(encoded.len() == 10);

    let mut min_row = 0;
    let mut max_row = 127;

    for c in encoded[..7].chars() {
        let diff = round_closest(max_row - min_row, 2);
        match c {
            'F' => max_row -= diff,
            'B' => min_row += diff,
            _ => unreachable!(),
        }
    }

    let mut min_col = 0;
    let mut max_col = 8;

    for c in encoded[7..].chars() {
        let diff = round_closest(max_col - min_col, 2);
        match c {
            'L' => max_col -= diff,
            'R' => min_col += diff,
            _ => unreachable!(),
        }
    }

    min_row * 8 + min_col
}

fn main() -> std::io::Result<()> {
    let cin = BufReader::new(File::open(
        "/home/amahindre/windowshome/AOC2020/day5/input.txt",
    )?);
    let mut seat_ids = Vec::new();
    let mut max_seat_id = 0;

    for l in cin.lines() {
        let line = l.unwrap();
        if line.is_empty() {
            continue;
        }
        let seat_id = get_seat_id(&line);
        max_seat_id = usize::max(max_seat_id, seat_id);
        seat_ids.push(seat_id);
    }

    // Part 1
    println!("Max seatId: {}", max_seat_id);

    // Part 2
    seat_ids.sort_unstable();

    for i in 1..seat_ids.len() - 1 {
        if seat_ids[i - 1] != seat_ids[i] - 1 {
            println!("Missing seat_id: {}", seat_ids[i] - 1);
        }
    }

    return Ok(());
}
