use nom::{
    branch::alt, character::complete::char, combinator::value, multi::many1, Finish, IResult,
};
use std::fs;

fn main() -> anyhow::Result<()> {
    let data = fs::read_to_string("day10.txt")?;
    parser(data);

    let res = part01();

    Ok(())
}

fn part01() -> i64 {
    0
}

fn part02() -> i64 {
    0
}

fn parser(data: String) {
    let width = data.lines().next().unwrap_or("").chars().count();
    let mut columns_raw = (0..width).map(|_| String::new()).collect::<Vec<_>>();

    // Parse each row, and build up the data for each column.
    // Pull the start position, corners, and horizontal pipes.
    for (y, row) in data.lines().enumerate() {
        for (index, c) in row.chars().enumerate() {
            columns_raw[index].push(c);
        }

        parse_row(y as i64, row);
    }

    // Parse each constructed column for vertical pipes.
    // All other relevant data was pulled from the row parsing.
    for (x, column) in columns_raw.iter().enumerate().take(1) {
        let pipes = parse_column(x as i64, &column);
        println!("{pipes:#?}");
    }
}

fn parse_row(y: i64, line: &str) -> (Option<Position>, Vec<Pipe>, Vec<Corner>) {
    let line_len = line.len();
    let mut line = line;

    let mut start_position = None;
    let mut pipes = Vec::new();
    let mut corners = Vec::new();

    let mut x = 0;

    while !line.is_empty() {
        let (r, data_type) = map_data(line).finish().unwrap();
        let len = (line.len() - r.len()) as i64;
        let offset = (line_len - r.len()) as i64;

        let start = Position(x, y);
        let end = Position(x + len - 1, y);

        match data_type {
            DataType::Start => start_position = Some(start),
            DataType::Horizontal => pipes.push(Pipe(start, end)),
            DataType::SouthWestCorner => corners.push(Corner::SouthWest(start)),
            DataType::NorthWestCorner => corners.push(Corner::NorthWest(start)),
            DataType::NorthEastCorner => corners.push(Corner::NorthEast(start)),
            DataType::SouthEastCorner => corners.push(Corner::SouthEast(start)),
            _ => {}
        }

        line = r;
        x = offset;
    }

    (start_position, pipes, corners)
}

fn parse_column(x: i64, line: &str) -> Vec<Pipe> {
    let line_len = line.len();
    let mut line = line;

    let mut pipes = Vec::new();

    let mut y = 0;

    while !line.is_empty() {
        let (r, data_type) = map_data(line).finish().unwrap();
        let len = (line.len() - r.len()) as i64;
        let offset = (line_len - r.len()) as i64;

        let start = Position(x, y);
        let end = Position(x, y + len - 1);

        match data_type {
            DataType::Vertical => pipes.push(Pipe(start, end)),
            _ => {}
        }

        line = r;
        y = offset;
    }

    pipes
}

#[derive(Debug)]
struct Position(i64, i64);

#[derive(Debug)]
struct Pipe(Position, Position);

#[derive(Debug)]
enum Corner {
    SouthWest(Position),
    NorthWest(Position),
    NorthEast(Position),
    SouthEast(Position),
}

#[derive(Debug, Clone)]
enum DataType {
    Start,
    Ground,
    Horizontal,
    Vertical,
    SouthWestCorner,
    NorthWestCorner,
    NorthEastCorner,
    SouthEastCorner,
}

fn map_data(input: &str) -> IResult<&str, DataType> {
    alt((
        ground, start, horizontal, vertical, south_west, north_west, north_east, south_east,
    ))(input)
}

fn ground(input: &str) -> IResult<&str, DataType> {
    value(DataType::Ground, many1(char('.')))(input)
}

fn start(input: &str) -> IResult<&str, DataType> {
    value(DataType::Start, char('S'))(input)
}

fn horizontal(input: &str) -> IResult<&str, DataType> {
    value(DataType::Horizontal, many1(char('-')))(input)
}

fn vertical(input: &str) -> IResult<&str, DataType> {
    value(DataType::Vertical, many1(char('|')))(input)
}

fn south_west(input: &str) -> IResult<&str, DataType> {
    value(DataType::SouthWestCorner, char('7'))(input)
}

fn north_west(input: &str) -> IResult<&str, DataType> {
    value(DataType::NorthWestCorner, char('J'))(input)
}

fn north_east(input: &str) -> IResult<&str, DataType> {
    value(DataType::NorthEastCorner, char('L'))(input)
}

fn south_east(input: &str) -> IResult<&str, DataType> {
    value(DataType::SouthEastCorner, char('F'))(input)
}
