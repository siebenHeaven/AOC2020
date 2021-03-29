use core::fmt::Error;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Op {
    Sum,
    Mul,
}

impl FromStr for Op {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len(), 1, "{}", s);
        match s.chars().next().unwrap() {
            '+' => Ok(Op::Sum),
            '*' => Ok(Op::Mul),
            _ => Err(Error),
        }
    }
}

fn solve(in_line: &str) -> i64 {
    // println!("{}", in_line);
    // At this point, line will be a simple expression: 7 + 4 * 3
    let mut o_op: Option<Op> = None;
    let mut s: Vec<(Option<i64>, Option<Op>)> = Vec::new();
    let perform_op = |acc: Option<i64>, o_op: Option<Op>, num: i64| {
        if let Some(lhs) = acc {
            if let Some(op) = &o_op {
                match op {
                    Op::Sum => Some(lhs + num),
                    Op::Mul => Some(lhs * num),
                }
            } else {
                Some(num)
            }
        } else {
            Some(num)
        }
    };
    in_line
        .split(' ')
        .filter(|s| !s.is_empty())
        .fold(None, |acc: Option<i64>, term| {
            if let Ok(num) = term.parse::<i64>() {
                perform_op(acc, o_op, num)
            } else if "(" == term {
                s.push((acc, o_op));
                o_op = None;
                None
            } else if ")" == term {
                let (o_temp_acc, o_temp_op) = s.pop().unwrap();
                perform_op(o_temp_acc, o_temp_op, acc.unwrap())
            } else if let Ok(op) = Op::from_str(term) {
                o_op = Some(op);
                acc
            } else {
                unreachable!()
            }
        })
        .unwrap()
}

fn part1(lines: &[String]) -> i64 {
    let ans = lines.iter().fold(0, |acc, x| acc + solve(x));
    ans
}

fn paranthesize(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .map(|line| {
            let mut new_line = String::from("( ( ");
            for s in line.split(' ') {
                new_line = match s {
                    "+" => format!("{}{}", new_line, " ) + ("),
                    "*" => format!("{}{}", new_line, " ) ) * ( ("),
                    "(" => format!("{}{}", new_line, " ( ( ("),
                    ")" => format!("{}{}", new_line, " ) ) )"),
                    _ => format!("{} {}", new_line, s),
                }
            }
            format!("{}{}", new_line, " ) )")
        })
        .collect()
}

fn part2(lines: &[String]) -> i64 {
    let updated_lines = paranthesize(lines);
    // println!("{:#?}", updated_lines);
    let ans = updated_lines.iter().fold(0, |acc, x| acc + solve(x));
    ans
}

fn main() {
    let lines: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .collect();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
