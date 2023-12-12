use std::{collections::HashMap, fs, iter::repeat, ops::Range};

use nom::{
    character::complete::char,
    combinator::map,
    multi::{many0, many1},
    sequence::tuple,
    Finish, IResult,
};

fn main() -> anyhow::Result<()> {
    let data = fs::read_to_string("day11.txt")?;

    let res = part01(&data);
    println!("Part 01: {res}");

    let res = part02(&data);
    println!("Part 02: {res}");

    Ok(())
}

fn part01(data: &str) -> i64 {
    let galaxies = parse_data(data, 2);
    distances(&galaxies)
}

fn part02(data: &str) -> i64 {
    let galaxies = parse_data(data, 1_000_000);
    distances(&galaxies)
}

fn distances(galaxies: &[Galaxy]) -> i64 {
    let len: usize = galaxies.len();

    (0..len)
        .flat_map(|index| {
            let first = repeat(index);
            let second = Range {
                start: index + 1,
                end: len,
            };

            first.zip(second)
        })
        .map(|(left, right)| galaxies[left].distance(&galaxies[right]))
        .sum()
}

fn parse_data(data: &str, oldness: i64) -> Vec<Galaxy> {
    let oldness = oldness - 1;
    let mut rows = Vec::new();
    let mut column_map: HashMap<usize, Vec<usize>> = HashMap::new();

    {
        let iter = data.lines().map(parser).enumerate();
        let mut y_offset = 0;

        for (y, row) in iter {
            let TokenLine::Galaxy(row) = row else {
                y_offset += oldness;
                continue;
            };

            let mut x_offset = 0;

            for x_parsed in row.iter() {
                let x = *x_parsed + x_offset;
                let y = y as i64 + y_offset;
                let galaxy = Galaxy::new(x, y);

                rows.push(galaxy);
                x_offset += x_parsed + 1;

                let key = x as usize;
                let value = rows.len() - 1;

                if let Some(v) = column_map.get_mut(&key) {
                    v.push(value);
                } else {
                    column_map.insert(key, vec![value]);
                }
            }
        }
    }

    let width = data.lines().next().map(str::len).unwrap_or(0);
    let mut x_offset = 0;

    for x in 0..width {
        if let Some(indicies) = column_map.get(&x) {
            for index in indicies {
                rows[*index].x += x_offset;
            }
        } else {
            x_offset += oldness;
        }
    }

    rows
}

struct Galaxy {
    x: i64,
    y: i64,
}

impl Galaxy {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Galaxy {
    fn distance(&self, other: &Galaxy) -> i64 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

enum TokenLine {
    Blank,
    Galaxy(Vec<i64>),
}

fn parser(input: &str) -> TokenLine {
    match galaxy(input).finish() {
        Ok((_, res)) => TokenLine::Galaxy(res),
        _ => TokenLine::Blank,
    }
}

fn galaxy(input: &str) -> IResult<&str, Vec<i64>> {
    let parser = tuple((blank, char('#')));
    many1(map(parser, |(blanks, _)| blanks))(input)
}

fn blank(input: &str) -> IResult<&str, i64> {
    map(many0(char('.')), |chars| chars.len() as i64)(input)
}
