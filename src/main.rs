use nom::{branch::alt, bytes::complete::tag, character::complete::char, Finish, IResult};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    let file = File::open("day01.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0usize;

    for line in reader.lines() {
        let line = line?;
        let output = parser(&line);

        if output.is_empty() {
            continue;
        }

        let first = output.first().map(|v| v * 10).unwrap() as usize;
        let last = *output.last().unwrap() as usize;

        sum += first + last;
    }

    println!("{sum}");

    Ok(())
}

fn parser(input: &str) -> Vec<u8> {
    let mut input = input;
    let mut output = Vec::new();

    while !input.is_empty() {
        let res = valid_value(input).finish();

        if let Ok((_, value)) = res {
            output.push(value);
        }

        input = &input[1..];
    }

    output
}

fn valid_value(input: &str) -> IResult<&str, u8> {
    alt((
        zero_digit,
        one_digit,
        two_digit,
        three_digit,
        four_digit,
        five_digit,
        six_digit,
        seven_digit,
        eight_digit,
        nine_digit,
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

fn zero_digit(input: &str) -> IResult<&str, u8> {
    let (input, _) = char('0')(input)?;
    Ok((input, 0))
}

fn one_digit(input: &str) -> IResult<&str, u8> {
    let (input, _) = char('1')(input)?;
    Ok((input, 1))
}

fn two_digit(input: &str) -> IResult<&str, u8> {
    let (input, _) = char('2')(input)?;
    Ok((input, 2))
}

fn three_digit(input: &str) -> IResult<&str, u8> {
    let (input, _) = char('3')(input)?;
    Ok((input, 3))
}

fn four_digit(input: &str) -> IResult<&str, u8> {
    let (input, _) = char('4')(input)?;
    Ok((input, 4))
}

fn five_digit(input: &str) -> IResult<&str, u8> {
    let (input, _) = char('5')(input)?;
    Ok((input, 5))
}

fn six_digit(input: &str) -> IResult<&str, u8> {
    let (input, _) = char('6')(input)?;
    Ok((input, 6))
}

fn seven_digit(input: &str) -> IResult<&str, u8> {
    let (input, _) = char('7')(input)?;
    Ok((input, 7))
}

fn eight_digit(input: &str) -> IResult<&str, u8> {
    let (input, _) = char('8')(input)?;
    Ok((input, 8))
}

fn nine_digit(input: &str) -> IResult<&str, u8> {
    let (input, _) = char('9')(input)?;
    Ok((input, 9))
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
