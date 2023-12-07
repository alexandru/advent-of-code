use std::collections::HashMap;

use itertools::Itertools;

enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

struct Hand {
    value: String,
    num_list: Vec<i32>,
    hand_type: HandType,
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
        let num_list = value
            .chars()
            .map(|it| it.to_string())
            .map(|it| *self.cards.get(&it).unwrap_or(&0i32))
            .collect_vec();
        let hand_type: HandType = todo!();
        Hand {
            value: value.to_string(),
            hand_type,
            num_list,
        }
    }
}

fn main() {
    println!("{:#?}", CardDeck::part1());
    println!("{:#?}", CardDeck::part2());
}
