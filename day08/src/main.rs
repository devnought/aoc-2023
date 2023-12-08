use anyhow::anyhow;
use nom::{
    branch::alt,
    character::complete::{alphanumeric1, char, multispace0, space0},
    combinator::{map, value},
    multi::many0,
    sequence::tuple,
    Finish, IResult,
};
use num::integer::lcm;
use std::{
    collections::HashMap,
    fs::{self},
};

fn main() -> anyhow::Result<()> {
    let data = fs::read_to_string("day08.txt")?;
    let map_data = parser(data)?;

    let res = part01(&map_data);
    println!("Part 01: {res}");

    let res = part02(&map_data);
    println!("Part 02: {res}");

    Ok(())
}

fn part01(map_data: &MapData) -> u64 {
    let MapData(directions, map) = map_data;
    let starts = ["AAA"];

    solver(&starts, directions, map)
}

fn part02(map_data: &MapData) -> u64 {
    let MapData(directions, map) = map_data;
    let starts = &map
        .keys()
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<_>>();

    solver(starts, directions, map)
}

fn solver<T>(starts: &[T], directions: &[Direction], map: &HashMap<String, Mapping>) -> u64
where
    T: AsRef<str>,
{
    let len = directions.len() as u64;

    starts
        .iter()
        .map(|s| s.as_ref())
        .map(|start| {
            let mut start = start;
            let mut count = 0;
            let mut destination = "";

            while !destination.ends_with('Z') {
                destination = directions.iter().fold(start, |location, direction| {
                    let entry = map.get(location).unwrap();

                    match direction {
                        Direction::Left => entry.left(),
                        Direction::Right => entry.right(),
                    }
                });

                count += len;
                start = destination;
            }

            count
        })
        .fold(1, lcm)
}

#[derive(Debug, Clone)]
struct Mapping(String, String);

impl Mapping {
    fn new(left: &str, right: &str) -> Self {
        let left = left.to_string();
        let right = right.to_string();

        Self(left, right)
    }

    fn left(&self) -> &str {
        &self.0
    }

    fn right(&self) -> &str {
        &self.1
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct MapData(Vec<Direction>, HashMap<String, Mapping>);

fn parser(input: String) -> anyhow::Result<MapData> {
    let (_, data) = data(&input)
        .finish()
        .map_err(|_| anyhow!("Could not parse data"))?;

    Ok(data)
}

fn data(input: &str) -> IResult<&str, MapData> {
    let parser = tuple((directions, multispace0, many0(map_line)));

    map(parser, |(directions, _, mappings)| {
        MapData(directions, mappings.into_iter().collect::<HashMap<_, _>>())
    })(input)
}

fn directions(input: &str) -> IResult<&str, Vec<Direction>> {
    many0(direction)(input)
}

fn direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Left, char('L')),
        value(Direction::Right, char('R')),
    ))(input)
}

fn map_line(input: &str) -> IResult<&str, (String, Mapping)> {
    let parser = tuple((
        alphanumeric1,
        space0,
        char('='),
        space0,
        char('('),
        alphanumeric1,
        space0,
        char(','),
        space0,
        alphanumeric1,
        space0,
        char(')'),
        multispace0,
    ));

    map(
        parser,
        |(source, _, _, _, _, left, _, _, _, right, _, _, _)| {
            let source: &str = source;
            (source.to_string(), Mapping::new(left, right))
        },
    )(input)
}
