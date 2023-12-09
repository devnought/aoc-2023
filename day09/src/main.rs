use nom::{
    character::complete::{i64, space0},
    combinator::map,
    multi::many0,
    sequence::tuple,
    Finish, IResult,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    let file = File::open("day09.txt")?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| parser(&line))
        .collect::<Vec<_>>();

    let res = part01(&lines);
    println!("Part 01: {res}");

    let res = part02(&lines);
    println!("Part 02: {res}");

    Ok(())
}

#[derive(Debug)]
struct Line(Vec<i64>);

fn part01(lines: &[Line]) -> i64 {
    lines
        .iter()
        .map(|line| {
            let mut values = line.0.clone();
            let mut acc = *values.last().unwrap();

            while !values.iter().all(|v| *v == 0) {
                values = values
                    .windows(2)
                    .filter_map(|data| match data {
                        &[left, right] => Some(right - left),
                        _ => None,
                    })
                    .collect::<Vec<_>>();

                acc += values.last().unwrap();
            }

            acc
        })
        .sum()
}

fn part02(lines: &[Line]) -> i64 {
    lines
        .iter()
        .map(|line| {
            let mut values = line.0.clone();
            let mut acc = *values.first().unwrap();

            while !values.iter().all(|v| *v == 0) {
                values = values
                    .windows(2)
                    .filter_map(|data| match data {
                        &[left, right] => Some(left - right),
                        _ => None,
                    })
                    .collect::<Vec<_>>();

                acc += values.first().unwrap();
            }

            acc
        })
        .sum()
}

fn parser(input: &str) -> Option<Line> {
    let (_, line) = line(input).finish().ok()?;
    Some(Line(line))
}

fn line(input: &str) -> IResult<&str, Vec<i64>> {
    many0(map(tuple((i64, space0)), |(num, _)| num))(input)
}
