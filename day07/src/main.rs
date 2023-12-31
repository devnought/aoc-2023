use core::panic;
use nom::{
    branch::alt,
    character::complete::{anychar, char, space0, u64},
    combinator::{map, value, verify},
    multi::many0,
    sequence::tuple,
    Finish, IResult,
};
use std::{
    cmp::{Ord, Ordering},
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    let res = part01()?;
    println!("Part 01: {res}");

    let res = part02()?;
    println!("Part 02: {res}");

    Ok(())
}

fn part01() -> anyhow::Result<u64> {
    let file = File::open("day07.txt")?;
    let reader = BufReader::new(file);

    let mut hands = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| parser01(&line))
        .collect::<Vec<_>>();
    hands.sort();

    let sum = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| {
            let multiplier = (index + 1) as u64;
            hand.bid * multiplier
        })
        .sum();

    Ok(sum)
}

fn part02() -> anyhow::Result<u64> {
    let file = File::open("day07.txt")?;
    let reader = BufReader::new(file);

    let mut hands = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| parser02(&line))
        .collect::<Vec<_>>();
    hands.sort();

    let sum = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| {
            let multiplier = (index + 1) as u64;

            hand.bid * multiplier
        })
        .sum();

    Ok(sum)
}

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq)]
enum Class {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [u8; 5],
    class: Class,
    bid: u64,
}

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq)]
struct CountCard(u8, u8);

impl CountCard {
    fn count(&self) -> u8 {
        self.0
    }

    fn card(&self) -> u8 {
        self.1
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.class.cmp(&other.class) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .filter_map(|(left, right)| match left.cmp(right) {
                    Ordering::Equal => None,
                    ord => Some(ord),
                })
                .next()
                .unwrap_or(Ordering::Equal),
            ord => ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn new(cards: [u8; 5], bid: u64) -> Self {
        let class = Self::classify_hand(&cards);

        Self { cards, class, bid }
    }

    fn new_wildcard(cards: [u8; 5], bid: u64) -> Self {
        let mut cards = cards;
        for card in &mut cards {
            if *card == 11 {
                *card = 0;
            }
        }

        let class = Self::classify_hand(&cards);
        Self { cards, class, bid }
    }

    fn classify_hand(cards: &[u8; 5]) -> Class {
        let counts = Self::card_counts(cards);
        let has_wildcard = counts.iter().any(|c| c.card() == 0);

        match counts.len() {
            1 => Class::FiveOfAKind,
            2 => match (counts[0].count(), counts[1].count()) {
                (1, 4) => match has_wildcard {
                    true => Class::FiveOfAKind,
                    _ => Class::FourOfAKind,
                },
                (2, 3) => match has_wildcard {
                    true => Class::FiveOfAKind,
                    _ => Class::FullHouse,
                },
                _ => panic!("impossible 2 count"),
            },
            3 => match (counts[0].count(), counts[1].count(), counts[2].count()) {
                (1, 1, 3) => match has_wildcard {
                    true => Class::FourOfAKind,
                    _ => Class::ThreeOfAKind,
                },
                (1, 2, 2) => match (counts[0].card(), counts[1].card(), counts[2].card()) {
                    (0, _, _) => Class::FullHouse,
                    (_, 0, _) => Class::FourOfAKind,
                    (_, _, 0) => Class::FourOfAKind,
                    _ => Class::TwoPair,
                },
                _ => panic!("impossible 3 count"),
            },
            4 => match has_wildcard {
                true => Class::ThreeOfAKind,
                _ => Class::OnePair,
            },
            5 => match has_wildcard {
                true => Class::OnePair,
                _ => Class::HighCard,
            },
            _ => panic!("impossible out of range"),
        }
    }

    fn card_counts(cards: &[u8]) -> Vec<CountCard> {
        let mut map = HashMap::new();

        for card in cards {
            if let Some(c) = map.get_mut(card) {
                *c += 1;
            } else {
                map.insert(*card, 1u8);
            }
        }

        let mut counts = map
            .into_iter()
            .map(|(card, count)| CountCard(count, card))
            .collect::<Vec<_>>();
        counts.sort();
        counts
    }
}

fn parser01(input: &str) -> Option<Hand> {
    let (_, (hand, bid)) = hand_bid(input).finish().ok()?;
    Some(Hand::new(hand, bid))
}

fn parser02(input: &str) -> Option<Hand> {
    let (_, (hand, bid)) = hand_bid(input).finish().ok()?;
    Some(Hand::new_wildcard(hand, bid))
}

fn hand_bid(input: &str) -> IResult<&str, ([u8; 5], u64)> {
    let parser = tuple((hand_raw, space0, u64));
    map(parser, |(hand, _, bid)| (hand, bid))(input)
}

fn hand_raw(input: &str) -> IResult<&str, [u8; 5]> {
    let parser = many0(card);
    map(parser, |cards| {
        if cards.len() != 5 {
            panic!("impossible parsed hand");
        } else {
            [cards[0], cards[1], cards[2], cards[3], cards[4]]
        }
    })(input)
}

fn card(input: &str) -> IResult<&str, u8> {
    alt((digit, face))(input)
}

fn face(input: &str) -> IResult<&str, u8> {
    alt((
        value(10, char('T')),
        value(11, char('J')),
        value(12, char('Q')),
        value(13, char('K')),
        value(14, char('A')),
    ))(input)
}

fn digit(input: &str) -> IResult<&str, u8> {
    let parser = verify(anychar, |c| *c != '0' && *c != '1' && c.is_ascii_digit());
    map(parser, |c| c.to_digit(10).unwrap() as u8)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_of_a_kind() {
        let card = Hand::new_wildcard([2, 2, 2, 2, 2], 1);
        assert_eq!(Class::FiveOfAKind, card.class);

        let card = Hand::new_wildcard([2, 2, 2, 2, 11], 1);
        assert_eq!(Class::FiveOfAKind, card.class);

        let card = Hand::new_wildcard([2, 2, 2, 11, 11], 1);
        assert_eq!(Class::FiveOfAKind, card.class);

        let card = Hand::new_wildcard([2, 2, 11, 11, 11], 1);
        assert_eq!(Class::FiveOfAKind, card.class);

        let card = Hand::new_wildcard([2, 11, 11, 11, 11], 1);
        assert_eq!(Class::FiveOfAKind, card.class);

        let card = Hand::new_wildcard([11, 11, 11, 11, 11], 1);
        assert_eq!(Class::FiveOfAKind, card.class);
    }

    #[test]
    fn four_of_a_kind() {
        let card = Hand::new_wildcard([4, 4, 4, 4, 2], 1);
        assert_eq!(Class::FourOfAKind, card.class);

        let card = Hand::new_wildcard([4, 4, 4, 11, 2], 1);
        assert_eq!(Class::FourOfAKind, card.class);

        let card = Hand::new_wildcard([4, 4, 11, 11, 2], 1);
        assert_eq!(Class::FourOfAKind, card.class);

        let card = Hand::new_wildcard([4, 11, 11, 11, 2], 1);
        assert_eq!(Class::FourOfAKind, card.class);

        let card = Hand::new_wildcard([3, 3, 11, 2, 11], 1);
        assert_eq!(Class::FourOfAKind, card.class);
    }

    #[test]
    fn full_house() {
        let card = Hand::new_wildcard([3, 3, 3, 2, 2], 1);
        assert_eq!(Class::FullHouse, card.class);

        let card = Hand::new_wildcard([11, 3, 3, 2, 2], 1);
        assert_eq!(Class::FullHouse, card.class);

        let card = Hand::new_wildcard([3, 3, 11, 2, 2], 1);
        assert_eq!(Class::FullHouse, card.class);
    }

    #[test]
    fn three_of_a_kind() {
        let card = Hand::new_wildcard([3, 3, 3, 2, 5], 1);
        assert_eq!(Class::ThreeOfAKind, card.class);

        let card = Hand::new_wildcard([11, 3, 3, 2, 5], 1);
        assert_eq!(Class::ThreeOfAKind, card.class);

        let card = Hand::new_wildcard([11, 11, 3, 2, 5], 1);
        assert_eq!(Class::ThreeOfAKind, card.class);
    }

    #[test]
    fn two_pair() {
        let card = Hand::new_wildcard([2, 2, 3, 3, 4], 1);
        assert_eq!(Class::TwoPair, card.class);

        let card = Hand::new_wildcard([2, 3, 3, 5, 2], 1);
        assert_eq!(Class::TwoPair, card.class);
    }

    #[test]
    fn one_pair() {
        let card = Hand::new_wildcard([2, 3, 4, 5, 5], 1);
        assert_eq!(Class::OnePair, card.class);

        let card = Hand::new_wildcard([2, 3, 4, 5, 11], 1);
        assert_eq!(Class::OnePair, card.class);
    }

    #[test]
    fn high_card() {
        let card = Hand::new_wildcard([2, 3, 4, 5, 14], 1);
        assert_eq!(Class::HighCard, card.class);

        let card = Hand::new_wildcard([6, 3, 2, 8, 9], 1);
        assert_eq!(Class::HighCard, card.class);
    }
}
