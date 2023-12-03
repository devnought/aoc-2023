use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map, verify},
    Finish, IResult,
};
use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    let file = File::open("day01.txt")?;
    let reader = BufReader::new(file);

    let sum = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| parser(&line))
        .filter_map(|numbers| {
            if numbers.is_empty() {
                None
            } else {
                Some((
                    *numbers.first().unwrap() as usize,
                    *numbers.last().unwrap() as usize,
                ))
            }
        })
        .fold(0, |acc, (first, last)| acc + (first * 10) + last);

    println!("{sum}");

    Ok(())
}

fn parser(input: &str) -> Vec<u8> {
    let mut input = input;
    let mut output = Vec::new();

    while !input.is_empty() {
        let res = valid_value(input).finish();

        if let Ok((remaining_input, value)) = res {
            output.push(value);

            if remaining_input.is_empty() {
                input = "";
            } else {
                let start = max(1, input.len() - remaining_input.len() - 1);
                input = &input[start..];
            }
        } else {
            input = &input[1..];
        }
    }

    output
}

fn valid_value(input: &str) -> IResult<&str, u8> {
    alt((
        digit_value,
        zero_str,
        one_str,
        two_str,
        three_str,
        four_str,
        five_str,
        six_str,
        seven_str,
        eight_str,
        nine_str,
    ))(input)
}

fn digit_value(input: &str) -> IResult<&str, u8> {
    let parser = verify(anychar, |c| c.is_ascii_digit());
    map(parser, |c| c.to_digit(10).unwrap() as u8)(input)
}

fn zero_str(input: &str) -> IResult<&str, u8> {
    map(tag("zero"), |_| 0)(input)
}

fn one_str(input: &str) -> IResult<&str, u8> {
    map(tag("one"), |_| 1)(input)
}

fn two_str(input: &str) -> IResult<&str, u8> {
    map(tag("two"), |_| 2)(input)
}

fn three_str(input: &str) -> IResult<&str, u8> {
    map(tag("three"), |_| 3)(input)
}

fn four_str(input: &str) -> IResult<&str, u8> {
    map(tag("four"), |_| 4)(input)
}

fn five_str(input: &str) -> IResult<&str, u8> {
    map(tag("five"), |_| 5)(input)
}

fn six_str(input: &str) -> IResult<&str, u8> {
    map(tag("six"), |_| 6)(input)
}

fn seven_str(input: &str) -> IResult<&str, u8> {
    map(tag("seven"), |_| 7)(input)
}

fn eight_str(input: &str) -> IResult<&str, u8> {
    map(tag("eight"), |_| 8)(input)
}

fn nine_str(input: &str) -> IResult<&str, u8> {
    map(tag("nine"), |_| 9)(input)
}
