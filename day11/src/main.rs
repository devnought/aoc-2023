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
    let galaxies = parse_data(data);

    let res = part01(&galaxies);
    println!("Part 01: {res}");

    let res = part02(&galaxies);
    println!("Part 02: {res}");

    Ok(())
}

fn part01(galaxies: &[Galaxy]) -> i64 {
    distances(&galaxies, 2)
}

fn part02(galaxies: &[Galaxy]) -> i64 {
    distances(&galaxies, 1_000_000)
}

fn distances(galaxies: &[Galaxy], expansion: i64) -> i64 {
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
        .map(|(left, right)| galaxies[left].distance(expansion - 1, &galaxies[right]))
        .sum()
}

fn parse_data(data: String) -> Vec<Galaxy> {
    let mut rows = Vec::new();
    let mut column_map: HashMap<usize, Vec<usize>> = HashMap::new();

    {
        let iter = data.lines().map(parser).enumerate();
        let mut y_expansion = 0;

        for (y, row) in iter {
            let TokenLine::Galaxy(row) = row else {
                y_expansion += 1;
                continue;
            };

            let mut x_offset = 0;

            for x_parsed in row.iter() {
                let x = *x_parsed + x_offset;
                let y = y as i64;
                let galaxy = Galaxy {
                    x,
                    y,
                    x_expansion: 0,
                    y_expansion,
                };

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
    let mut x_expansion = 0;

    for x in 0..width {
        if let Some(indicies) = column_map.get(&x) {
            for index in indicies {
                rows[*index].x_expansion += x_expansion;
            }
        } else {
            x_expansion += 1;
        }
    }

    rows
}

struct Galaxy {
    x: i64,
    y: i64,
    x_expansion: i64,
    y_expansion: i64,
}

impl Galaxy {
    fn distance(&self, expansion: i64, other: &Galaxy) -> i64 {
        let other_x = other.x + (other.x_expansion * expansion);
        let other_y = other.y + (other.y_expansion * expansion);

        let x = self.x + (self.x_expansion * expansion);
        let y = self.y + (self.y_expansion * expansion);

        (other_x - x).abs() + (other_y - y).abs()
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
