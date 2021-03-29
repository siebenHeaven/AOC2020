use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

//fn dfs(bag_map: HashMap<_>, &str) ->

fn main() {
    let mut bag_contained_by_map = HashMap::new();
    let mut bag_contains_map = HashMap::new();
    io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            // <---- lhs ----->         <----------------- rhs ----------------->
            // dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            // bright white bags contain 1 shiny gold bag.
            // dotted black bags contain no other bags.
            let (lhs, rhs) = line.splitn(2, " contain ").collect_tuple().unwrap();
            let (lhs_string, rhs_string) = (lhs.to_string(), rhs.to_string());
            let lhs = lhs_string.strip_suffix(" bags").unwrap();

            let rhs = rhs_string.strip_suffix(".").unwrap();
            let rhs_term_parser = |rhs_term: &str| {
                let (num, rest): (String, String) = rhs_term.chars().partition(|&c| c.is_digit(10));
                let color = rest.trim_end_matches('s').trim_end_matches(" bag").trim();
                if num.is_empty() {
                    (color.to_string(), 0)
                } else {
                    let num = num.parse::<usize>().unwrap();
                    (color.to_string(), num)
                }
            };
            rhs.split(", ")
                .filter(|rhs_term| !rhs_term.is_empty())
                .map(rhs_term_parser)
                .for_each(|(contained_bag, contained_num)| {
                    bag_contained_by_map
                        .entry(contained_bag.clone())
                        .or_insert(vec![])
                        .push((lhs.to_string(), contained_num));
                    if contained_num != 0 {
                        bag_contains_map
                            .entry(lhs.to_string())
                            .or_insert(vec![])
                            .push((contained_bag, contained_num));
                    }
                });
        });
    // {
    //     // Part 1
    //     let mut bags_that_can_contain_shiny_gold: HashSet<_> = bag_contained_by_map
    //         .get("shiny gold")
    //         .unwrap()
    //         .iter()
    //         .map(|(bag, _)| bag)
    //         .collect::<HashSet<_>>();

    //     let mut num_bags = bags_that_can_contain_shiny_gold.len();
    //     loop {
    //         let new_bags: HashSet<_> = bags_that_can_contain_shiny_gold
    //             .iter()
    //             .filter_map(|&bag| bag_contained_by_map.get(bag))
    //             .flat_map(|v| v.iter().map(|(bag, _)| bag))
    //             .collect();

    //         bags_that_can_contain_shiny_gold = bags_that_can_contain_shiny_gold
    //             .union(&new_bags)
    //             .cloned()
    //             .collect();
    //         if num_bags >= bags_that_can_contain_shiny_gold.len() {
    //             break;
    //         } else {
    //             num_bags = bags_that_can_contain_shiny_gold.len();
    //         }
    //     }
    //     println!(
    //         "Num bags that can contain Shiny Gold bag: {}",
    //         bags_that_can_contain_shiny_gold.len()
    //     );
    // }

    {
        // Part 2
        let mut bags_contained_in_shiny_gold: Vec<Vec<_>> = Vec::new();
        bags_contained_in_shiny_gold.push(bag_contains_map.get("shiny gold").unwrap().to_vec());
        loop {
            println!(
                "{:#?}\nNum bags that Shiny Gold bag must contain: {}",
                bags_contained_in_shiny_gold,
                bags_contained_in_shiny_gold
                    .iter()
                    .flatten()
                    .fold(0usize, |acc, (_, num)| acc + num)
            );
            let new_bags: Vec<_> = bags_contained_in_shiny_gold
                [bags_contained_in_shiny_gold.len() - 1]
                .iter()
                .map(|(b, n)| {
                    if bag_contains_map.contains_key(b) {
                        bag_contains_map
                            .get(b)
                            .unwrap()
                            .iter()
                            .cloned()
                            .map(|(cb, n2)| (cb, (n * n2)))
                            .collect()
                    } else {
                        vec![]
                    }
                })
                .flatten()
                .collect();

            if new_bags.is_empty() {
                break;
            } else {
                bags_contained_in_shiny_gold.push(new_bags);
            }
        }
        println!(
            "{:#?}\nNum bags that Shiny Gold bag must contain: {}",
            bags_contained_in_shiny_gold,
            bags_contained_in_shiny_gold
                .iter()
                .flatten()
                .fold(0usize, |acc, (_, num)| acc + num)
        );
    }
}
