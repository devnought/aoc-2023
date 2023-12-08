use anyhow::anyhow;
use nom::{
    branch::alt,
    character::complete::{alpha1, char, multispace0, space0},
    combinator::{map, value},
    multi::many0,
    sequence::tuple,
    Finish, IResult,
};
use std::{
    collections::HashMap,
    fs::{self},
};

fn main() -> anyhow::Result<()> {
    let res = part01()?;
    println!("Part 01: {res}");

    // let res = part02()?;
    // println!("Part 02: {res}");

    Ok(())
}

fn part01() -> anyhow::Result<u64> {
    let data = fs::read_to_string("day08.txt")?;
    let MapData(directions, map) = parser(data)?;

    let len = directions.len() as u64;
    let mut count = 0;

    let mut start = "AAA";
    let mut destination = "";

    while destination != "ZZZ" {
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

    Ok(count)
}

// fn part02() -> anyhow::Result<u64> {
//     // let file = File::open("day08.txt")?;
//     // let file = File::open("sample.txt")?;
//     // let reader = BufReader::new(file);

//     Ok(0)
// }

#[derive(Debug)]
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
        alpha1,
        space0,
        char('='),
        space0,
        char('('),
        alpha1,
        space0,
        char(','),
        space0,
        alpha1,
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
