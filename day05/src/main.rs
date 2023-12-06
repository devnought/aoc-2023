use nom::{
    bytes::complete::{tag, take_till1},
    character::complete::{char, i64, multispace0, space0},
    combinator::{map, value},
    multi::many0,
    sequence::tuple,
    Finish, IResult,
};
use rayon::prelude::*;
use std::{
    fs::{self},
    ops::Range,
};

fn main() -> anyhow::Result<()> {
    let res = part01()?;
    println!("Part 01: {res}");

    let res = part02()?;
    println!("Part 02: {res}");

    Ok(())
}

fn part01() -> anyhow::Result<i64> {
    let data = fs::read_to_string("day05.txt")?;
    let soil_data = parser(data)?;

    let seeds = &soil_data.seeds;
    let location = soil_data.location_from_slice(seeds);

    Ok(location)
}

fn part02() -> anyhow::Result<i64> {
    let data = fs::read_to_string("day05.txt")?;
    let soil_data = parser(data)?;

    let location = soil_data
        .seed_ranges()
        .par_iter()
        .map(|seeds| soil_data.location_from_range(seeds.clone()))
        .min()
        .unwrap();

    Ok(location)
}

#[derive(Debug, Default)]
struct SoilData {
    seeds: Vec<i64>,
    maps: Vec<Vec<MapValue>>,
}

impl SoilData {
    fn location_from_slice(&self, seeds: &[i64]) -> i64 {
        seeds.iter().map(|seed| self.map_seed(*seed)).min().unwrap()
    }

    fn location_from_range(&self, seeds: Range<i64>) -> i64 {
        seeds
            .into_iter()
            .map(|seed| self.map_seed(seed))
            .min()
            .unwrap()
    }

    fn map_seed(&self, seed: i64) -> i64 {
        let mut value = seed;

        for section in &self.maps {
            let res = section.iter().filter_map(|m| m.mapped_value(value)).next();
            if let Some(v) = res {
                value = v;
            }
        }

        value
    }

    fn seed_ranges(&self) -> Vec<Range<i64>> {
        let len = self.seeds.len() / 2;
        let mut seeds_iter = self.seeds.iter();

        (0..len)
            .map(|_| {
                let start = *seeds_iter.next().unwrap();
                let length = *seeds_iter.next().unwrap();

                start..(start + length)
            })
            .collect()
    }
}

#[derive(Debug, Default)]
struct MapValue {
    destination_range: Range<i64>,
    source_range: Range<i64>,
}

impl MapValue {
    fn new(destination_range_start: i64, source_range_start: i64, length: i64) -> Self {
        let destination_range = destination_range_start..(destination_range_start + length);
        let source_range = source_range_start..(source_range_start + length);

        Self {
            destination_range,
            source_range,
        }
    }

    fn mapped_value(&self, value: i64) -> Option<i64> {
        if self.source_range.contains(&value) {
            Some(self.destination_range.start + (value - self.source_range.start).abs())
        } else {
            None
        }
    }
}

fn parser(input: String) -> anyhow::Result<SoilData> {
    let (_, data) = soil_data(&input)
        .finish()
        .map_err(|_| anyhow::format_err!("parsing error"))?;

    Ok(data)
}

fn soil_data(input: &str) -> IResult<&str, SoilData> {
    let section_parser = map(tuple((map_names_and_numbers, multispace0)), |(data, _)| {
        data
    });
    let parser = tuple((seeds_parser, multispace0, many0(section_parser)));
    let (input, (seeds, sections)) = map(parser, |(seeds, _, sections)| (seeds, sections))(input)?;

    Ok((
        input,
        SoilData {
            seeds,
            maps: sections,
        },
    ))
}

fn seeds_parser(input: &str) -> IResult<&str, Vec<i64>> {
    let parser = tuple((tag("seeds"), char(':'), space0, seed_numbers));
    map(parser, |(_, _, _, nums)| nums)(input)
}

fn seed_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    let parser = map(tuple((i64, space0)), |(num, _)| num);
    many0(parser)(input)
}

fn map_end(input: &str) -> IResult<&str, ()> {
    value((), tuple((space0, tag("map:"), space0)))(input)
}

fn name_parser(input: &str) -> IResult<&str, &str> {
    take_till1(|c: char| c.is_whitespace())(input)
}

fn map_name(input: &str) -> IResult<&str, ()> {
    let parser = tuple((name_parser, space0, map_end));
    value((), parser)(input)
}

fn map_numbers(input: &str) -> IResult<&str, MapValue> {
    let parser = tuple((i64, space0, i64, space0, i64, multispace0));
    map(parser, |(one, _, two, _, three, _)| {
        MapValue::new(one, two, three)
    })(input)
}

fn map_names_and_numbers(input: &str) -> IResult<&str, Vec<MapValue>> {
    let parser = tuple((map_name, multispace0, many0(map_numbers)));
    map(parser, |(_, _, numbers)| numbers)(input)
}
