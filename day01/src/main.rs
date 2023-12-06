use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map, value, verify},
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
        .filter_map(|numbers| Some((*numbers.first()?, *numbers.last()?)))
        .map(|(first, last)| (first * 10) + last)
        .sum::<u32>();

    println!("{sum}");

    Ok(())
}

fn parser(input: &str) -> Vec<u32> {
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

fn valid_value(input: &str) -> IResult<&str, u32> {
    alt((
        digit_value,
        value(0, tag("zero")),
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input)
}

fn digit_value(input: &str) -> IResult<&str, u32> {
    let parser = verify(anychar, |c| c.is_ascii_digit());
    map(parser, |c| c.to_digit(10).unwrap())(input)
}
