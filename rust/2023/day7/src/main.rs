use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Hand {
    value: String,
    num_list: Vec<i32>,
    hand_type: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.num_list.cmp(&other.num_list),
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Bid {
    hand: Hand,
    bid: i32,
}

#[derive(Debug)]
struct CardDeck {
    cards: HashMap<String, i32>,
    with_joker: bool,
}

impl CardDeck {
    fn new(cards: &str, with_joker: bool) -> CardDeck {
        let chars = cards.chars().rev().collect_vec();
        let mut cards = HashMap::new();
        for (i, item) in chars.iter().enumerate() {
            cards.insert(item.to_string(), i as i32);
        }
        CardDeck { cards, with_joker }
    }

    fn part1() -> CardDeck {
        CardDeck::new("AKQJT98765432", false)
    }

    fn part2() -> CardDeck {
        CardDeck::new("AKQT98765432J", true)
    }

    fn hand(&self, value: &str) -> Hand {
        fn eval(groups: &[i32]) -> HandType {
            if groups.iter().any(|it| *it == 5) {
                HandType::FiveOfAKind
            } else if groups.iter().any(|it| *it == 4) {
                HandType::FourOfAKind
            } else if groups.iter().any(|it| *it == 3) {
                if groups.iter().any(|it| *it == 2) {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            } else if groups.iter().filter(|it| **it == 2).count() == 2 {
                HandType::TwoPair
            } else if groups.iter().any(|it| *it == 2) {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        }
        let num_list = value
            .chars()
            .map(|it| it.to_string())
            .map(|it| *self.cards.get(&it).unwrap_or(&0i32))
            .collect_vec();

        let possibilities = if self.with_joker {
            self.cards.keys().map(|it| it.as_str()).collect_vec()
        } else {
            vec!["J"]
        };

        let hand_type = possibilities.iter()
            .map(|it| value.replace('J', it))
            .map(|hand| {
                let groups = hand
                    .chars()
                    .sorted()
                    .group_by(|it| *it)
                    .into_iter()
                    .map(|it| it.1.count() as i32)
                    .collect_vec();
                eval(&groups)
            })
            .max()
            .unwrap();

        Hand {
            value: value.to_string(),
            hand_type,
            num_list,
        }
    }
}

fn calculate(bids: &[Bid]) -> i32 {
    bids.iter()
        .sorted_by_key(|it| &it.hand)
        .enumerate()
        .map(|(rank, bid)| bid.bid * (rank as i32 + 1))
        .sum::<i32>()
}

fn main() {
    let part1_deck = CardDeck::part1();
    let part1_bids = include_str!("input.txt")
        .lines()
        .map(|it| it.split_whitespace().collect_vec())
        .filter_map(|it| {
            if let &[hand, bid] = it.as_slice() {
                let hand = part1_deck.hand(hand);
                Some(Bid {
                    hand,
                    bid: bid.parse().unwrap(),
                })
            } else {
                None
            }
        })
        .collect_vec();
    let part1 = calculate(&part1_bids);

    let part2_deck = CardDeck::part2();
    let part2_bids = part1_bids
        .iter()
        .map(|it| Bid {
            hand: part2_deck.hand(&it.hand.value),
            bid: it.bid,
        })
        .collect_vec();
    let part2 = calculate(&part2_bids);

    println!("\nDay 7");
    println!("-----------------");
    println!("Part 1: {}", part1);
    println!("Part 2: {}\n", part2)
}
