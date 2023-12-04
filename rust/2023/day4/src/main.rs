use std::collections::HashMap;

use regex::{Captures, Regex};

struct Card {
    nr: usize,
    good_guesses: usize,
}

fn parse_numbers(c: &Captures<'_>, i: usize) -> Vec<u32> {
    c.get(i)
        .unwrap()
        .as_str()
        .split_whitespace()
        .filter_map(|nr| nr.parse::<u32>().ok())
        .collect::<Vec<u32>>()
}

fn main() {
    let card_re = Regex::new(r"Card\s+(\d+)\s*:([^|]+)[|]([^$]+)$").unwrap();

    let cards = include_str!("input.txt")
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .filter_map(|l| {
            card_re.captures(l).map(|c| {
                let nr = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let winning = parse_numbers(&c, 2);
                let guesses = parse_numbers(&c, 3);
                let good_guesses = guesses.iter().fold(0, |count, e| {
                    if winning.contains(e) {
                        count + 1
                    } else {
                        count
                    }
                });
                Card { nr, good_guesses }
            })
        })
        .collect::<Vec<Card>>();

    let part1 = cards
        .iter()
        .map(|card| {
            if card.good_guesses > 0 {
                2usize.pow(card.good_guesses as u32 - 1)
            } else {
                0
            }
        })
        .sum::<usize>();

    let part2 = {
        let mut map: HashMap<usize, usize> = HashMap::new();
        for card in &cards {
            let won_this = map.get(&card.nr).unwrap_or(&0) + 1;
            map.insert(card.nr, won_this);
            let slice = &cards[card.nr..(card.nr + card.good_guesses)];
            for c in slice {
                let v = map.get(&c.nr).unwrap_or(&0);
                map.insert(c.nr, v + won_this);
            }
        }
        map.values().sum::<usize>()
    };

    println!("\nDay 4\n------------");
    println!("Part 1: {}", part1);
    println!("Part 2: {}\n", part2);
}
