use itertools::Itertools;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn part1(lines: Vec<String>) -> usize {
    let ruleset: Vec<Vec<(usize, usize)>> = lines
        .iter()
        .take_while(|&line| !line.is_empty())
        .map(|line| {
            line.splitn(2, ": ")
                .collect_tuple::<(&str, &str)>()
                .unwrap()
                .1
                .split(" or ")
                .map(|rule| {
                    rule.splitn(2, '-')
                        .map(|num_str| num_str.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let invalid_other_ticket_values: Vec<Vec<usize>> = lines
        .iter()
        .skip_while(|line| line.as_str() != "nearby tickets:")
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|value_str| value_str.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|ticket| {
            ticket
                .into_iter()
                .filter(|&value| {
                    //value could be some feild?
                    !ruleset.iter().any(|rulegrp| {
                        rulegrp
                            .iter()
                            .any(|rule| (rule.0 <= value) && (rule.1 >= value))
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect();

    invalid_other_ticket_values
        .iter()
        .fold(0usize, |acc, tickets| acc + tickets.iter().sum::<usize>())
}

fn part2(lines: Vec<String>) -> u128 {
    let rulegrp_vec: Vec<Vec<(usize, usize)>> = lines
        .iter()
        .take_while(|&line| !line.is_empty())
        .map(|line| {
            line.splitn(2, ": ")
                .collect_tuple::<(&str, &str)>()
                .unwrap()
                .1
                .split(" or ")
                .map(|rule| {
                    rule.splitn(2, '-')
                        .map(|num_str| num_str.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let ticket_validator_filter_map = |ticket: Vec<usize>| {
        let valid_feilds_to_rulegrp_sets = ticket
            .iter()
            .filter_map(|&feild| {
                // Get (feild, rulegrp_id_set) for each field that follows atleast one rule_grp
                let rules_followed_set = rulegrp_vec
                    .iter()
                    .enumerate()
                    .filter_map(|(rulegrp_id, rulegrp)| {
                        // Get rulegrp_id if rule is followed
                        if rulegrp
                            .iter()
                            .any(|rule| (rule.0 <= feild) && (rule.1 >= feild))
                        {
                            Some(rulegrp_id)
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<usize>>();
                if !rules_followed_set.is_empty() {
                    // Check for non-zero number of rule-grps followed
                    // If it is zero, then feild is invalid
                    Some((feild, rules_followed_set))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if valid_feilds_to_rulegrp_sets.len() == ticket.len() {
            Some(valid_feilds_to_rulegrp_sets) // All feilds have atleast one rule
        } else {
            None // Invalid ticket as atleast 1 feild does not follow any rule
        }
    };

    let my_ticket: Vec<_> = lines
        .iter()
        .skip_while(|line| line.as_str() != "your ticket:")
        .skip(1)
        .take(1)
        .map(|line| {
            line.split(',')
                .map(|feild_str| feild_str.parse::<usize>().unwrap())
                // ticket
                .collect::<Vec<_>>()
        })
        .filter_map(ticket_validator_filter_map)
        .next()
        .unwrap(); // my ticket ought to be valid!

    let valid_other_tickets: Vec<Vec<(usize, HashSet<_>)>> = lines
        .iter()
        .skip_while(|line| line.as_str() != "nearby tickets:")
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|value_str| value_str.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter_map(ticket_validator_filter_map)
        .collect();

    // For each position in ticket, find what feilds it could be
    let temp = vec![my_ticket.clone()];
    let mut feilds_type: Vec<HashSet<usize>> =
        temp.iter()
            .chain(valid_other_tickets.iter())
            .fold(vec![], |mut acc, i| {
                acc = if acc.is_empty() {
                    i.iter().map(|(_, y)| y.clone()).collect()
                } else {
                    acc.iter()
                        .zip(i.iter().map(|(_, y)| y.clone()))
                        .map(|(hm1, hm2)| {
                            hm1.intersection(&hm2).cloned().collect::<HashSet<usize>>()
                        })
                        .collect()
                };
                acc
            });

    // Starting with the column that maps to only one feild
    // go with process of elimination to get unique mappings for each column to feild
    let num_feilds = feilds_type.len();
    let mut visited = HashSet::new();
    for _ in 0..num_feilds {
        let (i, rule_id) = feilds_type
            .iter()
            .enumerate()
            .find_map(|(i, hs)| {
                if (hs.len() == 1) && (!visited.contains(&i)) {
                    Some((i, hs.iter().cloned().next().unwrap()))
                } else {
                    None
                }
            })
            .unwrap();
        visited.insert(i);
        feilds_type
            .iter_mut()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .for_each(|(_, hs)| {
                hs.remove(&rule_id);
            });
    }

    my_ticket
        .iter()
        .map(|(x, _)| x)
        .zip(feilds_type.iter())
        .filter_map(|(feild, kind_hm)| {
            // assert_eq!(kind_hm.len(), 1);
            let departure_set: HashSet<_> = [0, 1, 2, 3, 4, 5].iter().cloned().collect();
            let intsec = kind_hm
                .intersection(&departure_set)
                .cloned()
                .collect::<HashSet<_>>();

            if !intsec.is_empty() {
                println!("{}: {:#?}", feild, intsec);
                Some(feild)
            } else {
                None
            }
        })
        .fold(1u128, |acc, val| acc * (*val as u128))
}

fn main() {
    let lines: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    println!("{}", part1(lines.clone()));
    println!("{}", part2(lines));
}

#[test]
fn test_part1() {
    let lines = vec![
        "class: 1-3 or 5-7",
        "row: 6-11 or 33-44",
        "seat: 13-40 or 45-50",
        "",
        "your ticket:",
        "7,1,14",
        "",
        "nearby tickets:",
        "7,3,47",
        "40,4,50",
        "55,2,20",
        "38,6,12",
    ];
    assert_eq!(part1(lines.iter().map(|l| l.to_string()).collect()), 71);
}
