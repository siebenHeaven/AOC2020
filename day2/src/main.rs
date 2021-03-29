use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() -> std::io::Result<()> {
    let cin = BufReader::new(stdin());
    let cout = &mut BufWriter::new(stdout());
    let mut valid_passwords = 0;
    let mut valid_passwords_2 = 0;

    for line in cin.lines() {
        let l_line = line.unwrap();
        if l_line.len() == 0 {
            continue;
        }

        // Parse each line that looks like this:
        // range char password
        // 0     1    2
        // 1-3   a:   abcd
        let words: Vec<&str> = l_line.split(' ').collect();
        assert!(words.len() == 3);

        // Get range
        let range: Vec<&str> = words[0].split('-').collect();
        assert!(range.len() == 2);
        let low = range[0].parse::<usize>().unwrap();
        let high = range[1].parse::<usize>().unwrap();

        // Get char
        assert!((words[1].len() == 2) && (words[1].chars().nth(1).unwrap() == ':'));
        let c = words[1].chars().nth(0).unwrap();

        // Get password
        let password = words[2];
        let mut count = 0;

        // Part 1
        for x in password.chars() {
            if c == x {
                count += 1;
            }
        }

        if (count >= low) && (count <= high) {
            valid_passwords += 1;
        }

        // Part 2
        let a = password.chars().nth(low - 1).unwrap() == c;
        let b = password.chars().nth(high - 1).unwrap() == c;

        if (a && !b) || (!a && b) {
            valid_passwords_2 += 1;
        }
    }

    write!(
        cout,
        "Valid passwords: {}, part 2: {}",
        valid_passwords, valid_passwords_2
    )?;

    return Ok(());
}
