use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{alpha0, char, i64, line_ending, space0},
    combinator::{eof, map, value},
    multi::many0,
    sequence::tuple,
    Finish, IResult,
};
use std::fs::{self};

fn main() -> anyhow::Result<()> {
    let res = part01()?;
    println!("Part 01: {res}");

    let res = part02()?;
    println!("Part 02: {res}");

    Ok(())
}

fn part01() -> anyhow::Result<i64> {
    let data_raw = fs::read_to_string("day06.txt")?;
    let data = parser_01(data_raw);
    let res = data.into_iter().map(|d| d.ways_to_win()).product();

    Ok(res)
}

fn part02() -> anyhow::Result<i64> {
    let data_raw = fs::read_to_string("day06.txt")?;
    let data = parser_02(data_raw);

    Ok(data.ways_to_win())
}

#[derive(Debug)]
struct TimeDistance {
    time: i64,
    distance: i64,
}

impl TimeDistance {
    fn new(time: i64, distance: i64) -> Self {
        Self { time, distance }
    }

    fn ways_to_win(&self) -> i64 {
        let a = -1;
        let b = self.time;
        let c = -self.distance - 1;

        let numerator_val = ((b.pow(2) - 4 * a * c) as f64).sqrt();
        let divisor = 2.0 * a as f64;

        let x0 = (-b as f64 + numerator_val) / divisor;
        let x1 = (-b as f64 - numerator_val) / divisor;

        let ret = (x0.ceil() - x1.floor()).abs() + 1.0;

        ret as i64
    }
}

fn parser_01(input: String) -> Vec<TimeDistance> {
    let (_, (time, distance)) = time_distance_parser(&input)
        .finish()
        .unwrap_or_else(|_| ("", (Vec::new(), Vec::new())));

    time.iter()
        .zip(distance.iter())
        .map(|(t, d)| TimeDistance::new(*t, *d))
        .collect()
}

fn parser_02(input: String) -> TimeDistance {
    let (_, (time, distance)) = time_distance_parser_single(&input)
        .finish()
        .unwrap_or(("", (0, 0)));

    TimeDistance::new(time, distance)
}

fn time_distance_parser(input: &str) -> IResult<&str, (Vec<i64>, Vec<i64>)> {
    tuple((data_parser, data_parser))(input)
}

fn time_distance_parser_single(input: &str) -> IResult<&str, (i64, i64)> {
    tuple((single_value_parser, single_value_parser))(input)
}

fn identifier(input: &str) -> IResult<&str, &str> {
    map(tuple((alpha0, char(':'))), |(name, _)| name)(input)
}

fn line_end_or_eof(input: &str) -> IResult<&str, ()> {
    alt((value((), line_ending), value((), eof)))(input)
}

fn data_parser(input: &str) -> IResult<&str, Vec<i64>> {
    let numbers = many0(map(tuple((i64, space0)), |(nums, _)| nums));
    let parser = tuple((identifier, space0, numbers));
    let parser_nums = map(parser, |(_, _, nums)| nums);

    map(tuple((parser_nums, line_end_or_eof)), |(data, _)| data)(input)
}

fn string_numbers(input: &str) -> IResult<&str, Vec<&str>> {
    let parser = tuple((take_while1(|c: char| c.is_ascii_digit()), space0));

    many0(map(parser, |(nums, _)| nums))(input)
}

fn single_value_parser_string(input: &str) -> IResult<&str, String> {
    let parser = tuple((identifier, space0, string_numbers));
    map(parser, |(_, _, num_strs)| num_strs.join(""))(input)
}

fn single_value_parser(input: &str) -> IResult<&str, i64> {
    let parser = tuple((single_value_parser_string, line_end_or_eof));
    map(parser, |(val, _)| val.parse::<i64>().unwrap())(input)
}
