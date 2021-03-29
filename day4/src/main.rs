use std::collections::HashMap;
use std::io::{stdin, BufRead, BufReader};

fn validate_byr(value: &str) -> bool {
    if value.len() != 4 {
        false
    } else {
        if let Ok(year) = value.parse::<usize>() {
            (year >= 1920) && (year <= 2002)
        } else {
            false
        }
    }
}

fn validate_iyr(value: &str) -> bool {
    if value.len() != 4 {
        false
    } else {
        if let Ok(year) = value.parse::<usize>() {
            (year >= 2010) && (year <= 2020)
        } else {
            false
        }
    }
}

fn validate_eyr(value: &str) -> bool {
    if value.len() != 4 {
        false
    } else {
        if let Ok(year) = value.parse::<usize>() {
            (year >= 2020) && (year <= 2030)
        } else {
            false
        }
    }
}

fn validate_hgt(value: &str) -> bool {
    if let Ok(hgt) = value[..value.len() - 2].parse::<usize>() {
        match &value[value.len() - 2..] {
            "cm" => (hgt >= 150) && (hgt <= 193),
            "in" => (hgt >= 59) && (hgt <= 76),
            _ => false,
        }
    } else {
        false
    }
}

fn validate_hcl(value: &str) -> bool {
    if &value[..1] == "#" {
        if let Ok(_) = usize::from_str_radix(&value[1..], 16) {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn validate_ecl(value: &str) -> bool {
    match value {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn validate_pid(value: &str) -> bool {
    if value.len() == 9 {
        if let Ok(_) = value.parse::<usize>() {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn validate(kind: &str, value: &str) -> bool {
    match kind {
        "byr" => validate_byr(value),
        "iyr" => validate_iyr(value),
        "eyr" => validate_eyr(value),
        "hgt" => validate_hgt(value),
        "hcl" => validate_hcl(value),
        "ecl" => validate_ecl(value),
        "pid" => validate_pid(value),
        "cid" => true, // any cid is acceptable
        _ => false,
    }
}

fn main() -> std::io::Result<()> {
    let cin = BufReader::new(stdin());
    let mut lines = Vec::new();
    let mut rqd = HashMap::new();
    let mut valid_count = 0;

    rqd.insert("byr", true);
    rqd.insert("iyr", true);
    rqd.insert("eyr", true);
    rqd.insert("hgt", true);
    rqd.insert("hcl", true);
    rqd.insert("ecl", true);
    rqd.insert("pid", true);

    for l in cin.lines() {
        let line = l.unwrap();
        lines.push(line);
    }

    let mut i = 0;
    loop {
        if i >= lines.len() {
            break;
        }

        let mut rqd_l = rqd.clone();

        while !lines[i].is_empty() {
            // println!("Got non empty line {}", lines[i]);
            let kv_pairs = lines[i].split(' ');
            for kv_pair in kv_pairs {
                let kv: Vec<&str> = kv_pair.split(':').collect();

                assert!(kv.len() == 2);

                if validate(kv[0], kv[1]) {
                    println!("\t{} is a valid {}", kv[1], kv[0]);
                    rqd_l.remove(kv[0]);
                } else {
                    println!("\t{} is an invalid {}", kv[1], kv[0]);
                }
            }
            i += 1;
        }

        // Skip over empty line
        i += 1;

        if rqd_l.len() == 0 {
            println!("Valid");
            valid_count += 1;
        } else {
            println!("Invalid");
        }
    }

    println!("valid count: {}\n", valid_count);
    return Ok(());
}
