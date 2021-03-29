use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

fn get_score(winner: VecDeque<usize>) -> usize {
    let deck_size_mul_2 = winner.len();
    winner
        .iter()
        .enumerate()
        .fold(0usize, |acc, (i, x)| acc + ((deck_size_mul_2 - i) * x))
}

fn combat(mut player1: VecDeque<usize>, mut player2: VecDeque<usize>) -> usize {
    while !player1.is_empty() && !player2.is_empty() {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();

        if card1 > card2 {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }
    }

    if !player1.is_empty() {
        get_score(player1)
    } else {
        get_score(player2)
    }
}

fn recursive_combat(
    mut player1: VecDeque<usize>,
    mut player2: VecDeque<usize>,
    hs: &mut HashSet<(VecDeque<usize>, VecDeque<usize>)>,
    game_id: &mut usize,
) -> (usize, bool) {
    *game_id += 1;
    //println!("=== Game {} ===\n", game_id);

    let mut _round = 0;
    let mut _sub_game_id = *game_id + 1;
    while !player1.is_empty() && !player2.is_empty() {
        _round += 1;
        //println!("-- Round: {} (Game {}) --", round, game_id);
        //println!("Player 1's deck: {:?}", player1);
        //println!("Player 2's deck: {:?}", player2);
        if hs.contains(&(player1.clone(), player2.clone())) {
            //println!("Wait a minute, we've been here before - Player 1 wins!");
            return (get_score(player1), true);
        } else {
            hs.insert((player1.clone(), player2.clone()));
            let card1 = player1.pop_front().unwrap();
            let card2 = player2.pop_front().unwrap();
            let p1_win: bool;

            //println!("Player 1 plays: {}", card1);
            //println!("Player 2 plays: {}", card2);

            if (player1.len() >= card1) && (player2.len() >= card2) {
                //println!("Playing a sub-game to determine the winner...");
                let (_score, p1) = recursive_combat(
                    player1
                        .clone()
                        .into_iter()
                        .take(card1)
                        .collect::<VecDeque<_>>(),
                    player2
                        .clone()
                        .into_iter()
                        .take(card2)
                        .collect::<VecDeque<_>>(),
                    &mut HashSet::new(),
                    &mut _sub_game_id,
                );
                //println!("...anyway, back to game {}.", game_id);
                p1_win = p1;
            } else if card1 > card2 {
                p1_win = true;
            } else {
                p1_win = false;
            }

            if p1_win {
                //println!("Player 1 wins round {} of game {}!\n", round, game_id);
                player1.push_back(card1);
                player1.push_back(card2);
            } else {
                //println!("Player 2 wins round {} of game {}!\n", round, game_id);
                player2.push_back(card2);
                player2.push_back(card1);
            }
        }
    }

    if !player1.is_empty() {
        (get_score(player1), true)
    } else {
        (get_score(player2), false)
    }
}

fn get_player_queues(lines: Vec<String>) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut p = true;
    let (player1, player2): (Vec<_>, Vec<_>) = lines.iter().partition(|line| {
        if p && line.is_empty() {
            p = false;
        }
        p
    });

    (
        player1
            .iter()
            .filter_map(|line| line.parse::<usize>().ok())
            .collect(),
        player2
            .iter()
            .filter_map(|line| line.parse::<usize>().ok())
            .collect(),
    )
}

fn part1(lines: Vec<String>) -> usize {
    let (player1, player2) = get_player_queues(lines);
    combat(player1, player2)
}

fn part2(lines: Vec<String>) -> usize {
    let mut hs: HashSet<(VecDeque<usize>, VecDeque<usize>)> = HashSet::new();
    let (player1, player2) = get_player_queues(lines);
    let mut game_id = 0;

    recursive_combat(player1, player2, &mut hs, &mut game_id).0
}

fn main() {
    let lines: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    println!("Part1: {}", part1(lines.clone()));
    println!("Part2: {}", part2(lines));
}

#[test]
fn test_part1() {
    let lines = "Player 1:\n\
    9\n\
    2\n\
    6\n\
    3\n\
    1\n\
    \n\
    Player 2:\n\
    5\n\
    8\n\
    4\n\
    7\n\
    10\n\
    "
    .split('\n')
    .map(|line| line.to_string())
    .collect();

    assert_eq!(306, part1(lines));
}

#[test]
fn test_part2() {
    let lines = "Player 1:\n\
    9\n\
    2\n\
    6\n\
    3\n\
    1\n\
    \n\
    Player 2:\n\
    5\n\
    8\n\
    4\n\
    7\n\
    10\n\
    "
    .split('\n')
    .map(|line| line.to_string())
    .collect();

    assert_eq!(291, part2(lines));
}
