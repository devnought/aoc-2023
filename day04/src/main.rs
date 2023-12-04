use nom::{
    bytes::complete::tag,
    character::complete::{char, space0, u64},
    combinator::map,
    multi::many0,
    sequence::tuple,
    Finish, IResult,
};
use std::{
    collections::{HashMap, HashSet},
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
    let file = File::open("day04.txt")?;
    let reader = BufReader::new(file);

    let sum = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| parser(&line))
        .map(|card| card.points())
        .sum();

    Ok(sum)
}

fn part02() -> anyhow::Result<u64> {
    let file = File::open("day04.txt")?;
    let reader = BufReader::new(file);

    let iter = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| parser(&line));

    let mut map = HashMap::new();

    for card in iter {
        let card_count = if let Some(count) = map.get_mut(&card.id) {
            *count += 1;
            *count
        } else {
            map.insert(card.id, 1);
            1
        };

        for win_id in card.win_set() {
            if let Some(count) = map.get_mut(&win_id) {
                *count += card_count;
            } else {
                map.insert(win_id, card_count);
            };
        }
    }

    Ok(map.values().sum())
}

#[derive(Debug, Clone)]
struct Card {
    id: u64,
    winning: HashSet<u64>,
    numbers: HashSet<u64>,
}

impl Card {
    fn points(&self) -> u64 {
        match self.wins() {
            0 => 0,
            wins => 2u64.pow((wins - 1) as u32),
        }
    }

    fn wins(&self) -> u64 {
        self.numbers.intersection(&self.winning).count() as u64
    }

    fn win_set(&self) -> Vec<u64> {
        let count = self.wins();
        let start = self.id + 1;
        (start..start + count).collect()
    }
}

fn parser(input: &str) -> Option<Card> {
    let (_, card) = card(input).finish().ok()?;
    Some(card)
}

fn card(input: &str) -> IResult<&str, Card> {
    let parser = tuple((
        tag("Card"),
        space0,
        u64,
        char(':'),
        space0,
        numbers,
        separator,
        numbers,
    ));

    map(parser, |(_, _, id, _, _, winning, _, numbers)| Card {
        id,
        winning,
        numbers,
    })(input)
}

fn numbers(input: &str) -> IResult<&str, HashSet<u64>> {
    let nums = map(tuple((u64, space0)), |(nums, _)| nums);
    let parser = many0(nums);
    map(parser, |n| n.into_iter().collect::<HashSet<_>>())(input)
}

fn separator(input: &str) -> IResult<&str, ()> {
    let parser = map(tuple((space0, char('|'), space0)), |_| ());
    map(parser, |_| ())(input)
}
