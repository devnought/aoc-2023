use nom::{
    branch::alt, bytes::complete::tag, character::complete::anychar, combinator::verify, Finish,
    IResult,
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
    let (input, v) = verify(anychar, |c| c.is_digit(10))(input)?;
    Ok((input, v.to_digit(10).unwrap() as u8))
}

fn zero_str(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("zero")(input)?;
    Ok((input, 0))
}

fn one_str(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("one")(input)?;
    Ok((input, 1))
}

fn two_str(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("two")(input)?;
    Ok((input, 2))
}

fn three_str(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("three")(input)?;
    Ok((input, 3))
}

fn four_str(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("four")(input)?;
    Ok((input, 4))
}

fn five_str(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("five")(input)?;
    Ok((input, 5))
}

fn six_str(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("six")(input)?;
    Ok((input, 6))
}

fn seven_str(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("seven")(input)?;
    Ok((input, 7))
}

fn eight_str(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("eight")(input)?;
    Ok((input, 8))
}

fn nine_str(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("nine")(input)?;
    Ok((input, 9))
}
