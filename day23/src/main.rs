use std::io::{self, BufRead};

fn part1(lines: Vec<String>) -> String {
    assert_eq!(1, lines.len());
    let mut curr: Vec<_> = lines[0].chars().map(|c| c.to_digit(10).unwrap()).collect();
    const NUM_MOVES: usize = 10;
    let LEN: usize = curr.len();

    for i in 0..NUM_MOVES {
        let index = ((i % LEN) + 1) % LEN;
        let three_cups = curr.split_off(index + 1);
        let temp = three_cups.split_off(3);
        curr.append(temp);
    }

    curr.iter()
        .map(|x| std::char::from_digit(*x, 10).unwrap())
        .collect::<String>()
}

fn main() {
    let lines: Vec<_> = io::stdin()
        .lock()
        .lines()
        .filter_map(|line| {
            let temp = line.unwrap();
            if !temp.is_empty() {
                Some(temp)
            } else {
                None
            }
        })
        .collect();

    println!("Part1: {}", part1(lines.clone()));
    // println!("Part2: {}", part2(lines));
}

#[test]
fn test_part1() {
    let lines = "389125467\n\
    "
    .split('\n')
    .map(|line| line.to_string())
    .collect();

    assert_eq!("92658374", part1(lines));
}

// #[test]
// fn test_part2() {
//     let lines = "Player 1:\n\
//     9\n\
//     2\n\
//     6\n\
//     3\n\
//     1\n\
//     \n\
//     Player 2:\n\
//     5\n\
//     8\n\
//     4\n\
//     7\n\
//     10\n\
//     "
//     .split('\n')
//     .map(|line| line.to_string())
//     .collect();

//     assert_eq!(291, part2(lines));
// }
