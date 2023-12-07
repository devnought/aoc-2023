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
        .filter_map(|line| parser(&line))
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

fn part02() -> anyhow::Result<i64> {
    Ok(0)
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

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.class.cmp(&other.class) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .filter_map(|(left, right)| match left.cmp(right) {
                    Ordering::Equal => None,
                    Ordering::Greater => Some(Ordering::Greater),
                    Ordering::Less => Some(Ordering::Less),
                })
                .next()
                .unwrap_or(Ordering::Equal),
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
        let mut map = HashMap::new();

        for card in cards {
            if let Some(c) = map.get_mut(&card) {
                *c += 1;
            } else {
                map.insert(card, 1u8);
            }
        }

        let mut counts = map.values().into_iter().collect::<Vec<_>>();
        counts.sort();

        let class = match counts.len() {
            1 => Class::FiveOfAKind,
            2 => match (counts[0], counts[1]) {
                (1, 4) => Class::FourOfAKind,
                (2, 3) => Class::FullHouse,
                _ => panic!("impossible 2 count"),
            },
            3 => match (counts[0], counts[1], counts[2]) {
                (1, 1, 3) => Class::ThreeOfAKind,
                (1, 2, 2) => Class::TwoPair,
                _ => panic!("impossible 3 count"),
            },
            4 => Class::OnePair,
            5 => Class::HighCard,
            _ => panic!("impossible out of range"),
        };

        Self { cards, class, bid }
    }
}

fn parser(input: &str) -> Option<Hand> {
    let (_, hand_bid) = hand_bid(input).finish().ok()?;
    Some(hand_bid)
}

fn hand_bid(input: &str) -> IResult<&str, Hand> {
    let parser = tuple((hand, space0, u64));
    map(parser, |(hand, _, bid)| (Hand::new(hand, bid)))(input)
}

fn hand(input: &str) -> IResult<&str, [u8; 5]> {
    let parser = many0(card);
    map(parser, |cards| {
        if cards.len() != 5 {
            [0u8; 5]
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
