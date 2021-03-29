use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum Rule {
    RuleIds(Vec<Vec<i64>>),
    Leaf(String),
}

impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('"') {
            // 5: "b"
            Ok(Rule::Leaf(s.trim_matches('"').to_string()))
        } else {
            // 1: 2 3 | 3 2
            let ids;
            let get_space_seperated = |line: &str| {
                line.split(" ")
                    .map(|word| word.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            };
            if s.contains(" | ") {
                ids = s
                    .split(" | ")
                    .map(|rule| get_space_seperated(rule))
                    .collect::<Vec<_>>();
            } else {
                ids = vec![get_space_seperated(s)];
            }
            Ok(Rule::RuleIds(ids))
        }
    }
}

fn solve_rule(
    rule_id: i64,
    rules: &HashMap<i64, Rule>,
    solved_rules: &mut HashMap<i64, String>,
) -> String {
    if let Some(ans) = solved_rules.get(&rule_id) {
        ans.to_string()
    } else {
        let ans;
        let mut get_str_from_rules = |mut acc, v: &Vec<_>| {
            for i in v {
                acc = format!("{}{}", acc, solve_rule(*i, rules, solved_rules))
            }
            format!("({})", acc)
        };
        if let Rule::RuleIds(r) = rules.get(&rule_id).unwrap() {
            ans = r.iter().fold("".to_string(), |acc, x| {
                if acc.is_empty() {
                    get_str_from_rules("".to_string(), x).to_string()
                } else {
                    format!(
                        "({}|{})",
                        acc,
                        get_str_from_rules("".to_string(), x).to_string()
                    )
                    .trim_matches('|')
                    .to_string()
                }
            })
        } else if let Rule::Leaf(l) = rules.get(&rule_id).unwrap() {
            ans = l.to_string()
        } else {
            unreachable!();
        }
        solved_rules.insert(rule_id, ans.clone());
        ans
    }
}

fn part1(lines: &Vec<String>) -> usize {
    //      0: 4 1 5
    //      1: 2 3 | 3 2
    //      2: 4 4 | 5 5
    //      3: 4 5 | 5 4
    //      4: "a"
    //      5: "b"
    let rules: HashMap<_, _> = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|rule_str| {
            let rule_tuple = rule_str
                .splitn(2, ": ")
                .collect_tuple::<(&str, &str)>()
                .unwrap();

            (
                rule_tuple.0.parse::<i64>().unwrap(),
                Rule::from_str(rule_tuple.1).unwrap(),
            )
        })
        .collect();
    let to_match: Vec<_> = lines.iter().skip_while(|s| !s.is_empty()).skip(1).collect();

    let mut solved_rules: HashMap<_, _> = HashMap::new();
    let rule_0: String = solve_rule(0, &rules, &mut solved_rules);

    let re: Regex = Regex::new(format!(r"\A{}\z", rule_0).as_str()).unwrap();

    to_match.iter().filter(|word| re.is_match(word)).count()
}

fn part2(lines: &Vec<String>) -> usize {
    //      0: 4 1 5
    //      1: 2 3 | 3 2
    //      2: 4 4 | 5 5
    //      3: 4 5 | 5 4
    //      4: "a"
    //      5: "b"
    let rules: HashMap<_, _> = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|rule_str| {
            let rule_tuple = rule_str
                .splitn(2, ": ")
                .collect_tuple::<(&str, &str)>()
                .unwrap();

            (
                rule_tuple.0.parse::<i64>().unwrap(),
                Rule::from_str(rule_tuple.1).unwrap(),
            )
        })
        .collect();
    let to_match: Vec<_> = lines.iter().skip_while(|s| !s.is_empty()).skip(1).collect();

    let mut solved_rules: HashMap<_, _> = HashMap::new();

    let rule_42 = solve_rule(42, &rules, &mut solved_rules);
    let rule_31 = solve_rule(31, &rules, &mut solved_rules);

    // 8: 42 | 42 8 == 42n
    // 11: 42 31 | 42 11 31 == 42n31n
    // 0: 8 11 == 42x42n31n == 42m31n; m > n
    let rule_8 = format!("(({})+)", rule_42);
    let rule_11 = format!("((({})+)(({})+))", rule_42, rule_31);

    // 0: 8 11
    let rule_0 = format!("(({})({}))", rule_8, rule_11);

    let r0 = Regex::new(format!(r"\A{}\z", rule_0).as_str()).unwrap();
    let r8 = Regex::new(format!(r"({})", rule_8).as_str()).unwrap();
    let r42 = Regex::new(format!(r"({})", rule_42).as_str()).unwrap();
    let r31 = Regex::new(format!(r"({})", rule_31).as_str()).unwrap();
    println!("{:#?}", r0);

    to_match
        .iter()
        .filter(|word| {
            let mat = r8.find(word).unwrap();
            let count42 = r42.find_iter(&word[mat.start()..mat.end()]).count();
            let count31 = r31.find_iter(&word[mat.end()..]).count();
            println!("count42: {}, count31: {}", count42, count31);
            r0.is_match(word) && (count42 > count31)
        })
        .count()
}

fn main() {
    let lines: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}
