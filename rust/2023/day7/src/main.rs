use std::collections::HashMap;
use itertools::Itertools;

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

#[derive(Debug, Clone)]
struct Hand {
    value: String,
    num_list: Vec<i32>,
    hand_type: HandType,
}

impl Ord + PartialOrd for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type.cmp(&other.hand_type)
            .then_with(|| self.num_list.cmp(&other.num_list))
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
        let chars = cards.chars().map(|it| it.to_string()).rev().collect_vec();
        let mut cards = HashMap::new();
        for (i, item) in chars.iter().enumerate() {
            cards.insert(item.clone(), i as i32);
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
        fn eval(groups: &Vec<i32>) -> HandType {
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

        let hand_type = self.cards.keys()
            .map(|it| value.replace("J", it))
            .map(|hand| {
                let groups = hand.chars().group_by(|it| *it)
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

fn calculate(bids: Vec<Bid>) -> i32 {
    let mut bids = bids.clone().sort_by_key(|it| it.hand);
    todo!()
}

fn main() {
    let part1Deck = CardDeck::part1();
    let input = include_str!("input.txt").lines()
        .map(|it| it.split_whitespace().collect_vec())
        .filter_map(|it| {
            if let &[hand, bid] = it.as_slice() {
                let hand = part1Deck.hand(hand);
                Some(Bid { hand, bid: bid.parse().unwrap() })
            } else {
                None
            }
        })
        .collect_vec();


    // let part2Deck = CardDeck::part2();
}
