use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("input");
    let answer = part_two(input);

    println!("Result: {}", answer);
}

#[test]
fn test() {
    let example = include_str!("example");
    let example_two = part_two(example);
    assert_eq!(example_two, 5905);
}

fn part_two(input: &str) -> i64 {
    let mut hands = read(input);
    hands.sort();

    let mut mult: i64 = hands.len().try_into().unwrap();

    let mut sum: i64 = 0;
    for hand in hands {
        println!("{}, {}, {:?}. Rank: {}", hand.cards, hand.bid, hand.hand_type, mult);

        sum += hand.bid * mult;
        mult -= 1;
    }

    sum
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    /// where all five cards have the same label
    FiveOfAKind,
    /// where four cards have the same label and one card has a different label
    FourOfAKind,
    /// where three cards have the same label, and the remaining two cards share a different label
    FullHouse,
    /// where three cards have the same label, and the remaining two cards are each different from any other card in the hand
    ThreeOfAKind,
    /// where two cards share one label, two other cards share a second label, and the remaining card has a third label
    TwoPair,
    /// where two cards share one label, and the other three cards have a different label from the pair and each other
    OnePair,
    /// where all cards' labels are distinct
    HighCard,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    pub bid: i64,
    pub cards: String,
    pub hand_type: HandType,
}

fn find_type(cards: &String) -> HandType {
    let mut distinct = HashMap::new();
    for char in cards.chars() {
        match distinct.get(&char) {
            Some(value) => distinct.insert(char, value + 1),
            None => distinct.insert(char, 1),
        };
    }

    let hand_type = match distinct.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            let mut t = HandType::FullHouse;
            for value in distinct.values() {
                if *value == 4 {
                    t = HandType::FourOfAKind;
                }
            }
            t
        }
        3 => {
            let mut t = HandType::TwoPair;
            for value in distinct.values() {
                if *value == 3 {
                    t = HandType::ThreeOfAKind;
                }
            }
            t
        }
        4 => HandType::OnePair,
        _ => HandType::HighCard,
    };

    hand_type
}

#[derive(Debug)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.split(' ').collect::<Vec<&str>>();
        let cards = data
            .get(0)
            .and_then(|c| Some(c.trim().to_string()))
            .ok_or(ParseHandError)?;

        let bid = data
            .get(1)
            .and_then(|v| Some(v.trim().parse::<i64>()))
            .ok_or(ParseHandError)?
            .map_err(|_| ParseHandError)?;

        // J cards can pretend to be whatever card is best for the purpose of determining hand type; for example, QJJQ2 is now considered four of a kind.
        let hand_type = if cards.contains('J') {
            let mut best_type = find_type(&cards);
            println!("[Find best joker] {}", cards);

            for char in cards.chars() {
                if char == 'J' {
                    continue;
                }

                let mutated = cards.replace('J', &char.to_string());
                let t = find_type(&mutated);
                // best_type = std::cmp::min(best_type, t);

                if t < best_type {
                    println!("New best type: {}, {:?}.", mutated, t);
                    best_type = t;
                }
            }

            best_type
        } else {
            find_type(&cards)
        };        

        Ok(Hand {
            bid,
            cards,
            hand_type,
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return std::cmp::Ordering::Equal;
        }

        let quick = self.hand_type.cmp(&other.hand_type);
        // J cards are now the weakest individual cards, weaker even than 2. The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.
        let cards_order = vec![
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
        ];

        if quick.is_eq() {
            for i in 0..5 {
                let a = self.cards.chars().nth(i).unwrap();
                let b = other.cards.chars().nth(i).unwrap();

                let ai = cards_order.iter().position(|x| *x == a).unwrap();
                let bi = cards_order.iter().position(|x| *x == b).unwrap();

                let cmp = ai.cmp(&bi);
                if cmp.is_eq() {
                    continue;
                }

                return cmp;
            }
        }

        quick
    }
}

fn read(input: &str) -> Vec<Hand> {
    let mut hands = Vec::new();

    for line in input.lines() {
        let hand = line.parse::<Hand>();
        // println!("parsed hand: {:?}", hand);
        match hand {
            Ok(hand) => hands.push(hand),
            Err(_) => println!("Parse error: {}", line),
        };
    }

    hands
}
