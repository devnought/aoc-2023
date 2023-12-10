use geo::{BoundingRect, Contains, Coord, LineString, Point, Polygon};
use nom::{
    branch::alt, character::complete::char, combinator::value, multi::many1, Finish, IResult,
};
use std::{cmp::Eq, collections::HashMap, fmt::Display, fs, iter::repeat};

fn main() -> anyhow::Result<()> {
    let data_raw = fs::read_to_string("day10.txt")?;
    let data = parser(data_raw);

    let res = part01(&data);
    println!("Part 01: {res}");

    let res = part02(&data);
    println!("Part 02: {res}");

    Ok(())
}

fn part01(data: &Data) -> i64 {
    data.build_path().iter().map(Element::len).sum::<i64>() / 2
}

fn part02(data: &Data) -> i64 {
    let path = data.build_path();
    let verticies = path
        .iter()
        .filter_map(|e| match e {
            Element::Corner(c) => Some(c.position()),
            Element::Start(s) => Some(s.position),
            _ => None,
        })
        .map(|p| Coord {
            x: p.x as f64,
            y: p.y as f64,
        })
        .collect::<Vec<_>>();

    let polygon = Polygon::new(LineString::new(verticies), Vec::new());
    let bounds = polygon.bounding_rect().unwrap();

    let x_range = bounds.min().x as i64 + 1..bounds.max().x as i64;
    let y_range = bounds.min().y as i64 + 1..bounds.max().y as i64;

    x_range
        .into_iter()
        .flat_map(|x| repeat(x).zip(y_range.clone()))
        .map(|(x, y)| Point::new(x as f64, y as f64))
        .filter(|p| polygon.contains(p))
        .count() as i64
}

#[derive(Debug)]
struct Data {
    start: Start,
    elements: HashMap<Position, Element>,
}

impl Data {
    fn new(start: Position, elements: HashMap<Position, Element>) -> Self {
        Self {
            start: Start::new(start),
            elements,
        }
    }

    fn build_path(&self) -> Vec<Element> {
        let start_adjacencies = self.start.adjacencies();
        let (mut element, mut connection) = start_adjacencies
            .into_iter()
            .filter_map(|connection| {
                let element = self.elements.get(&connection.position)?;
                element.next_connection(connection.direction)?;
                Some((element, connection))
            })
            .next()
            .expect("No adjacent to start?");

        let mut parts = vec![Element::Start(self.start), *element];

        loop {
            connection = element.next_connection(connection.direction).unwrap();

            if connection.position == self.start.position {
                break;
            }

            element = self.elements.get(&connection.position).unwrap();
            parts.push(*element);
        }

        parts
    }
}

fn parser(data: String) -> Data {
    let width = data.lines().next().unwrap_or("").chars().count();
    let mut columns_raw = (0..width).map(|_| String::new()).collect::<Vec<_>>();

    let mut start_position = Position::new(-1, -1);
    let mut elements = HashMap::new();

    // Parse each row, and build up the data for each column.
    // Pull the start position, corners, and horizontal pipes.
    for (y, row) in data.lines().enumerate() {
        for (index, c) in row.chars().enumerate() {
            columns_raw[index].push(c);
        }

        let (start, pipes, corners) = parse_row(y as i64, row);

        if let Some(s) = start {
            start_position = s;
        }

        for pipe in pipes {
            elements.insert(pipe.start, Element::Pipe(pipe));
        }

        for corner in corners {
            elements.insert(corner.position(), Element::Corner(corner));
        }
    }

    // Parse each constructed column for vertical pipes.
    // All other relevant data was pulled from the row parsing.
    for (x, column) in columns_raw.iter().enumerate() {
        let pipes = parse_column(x as i64, column);

        for pipe in pipes {
            elements.insert(pipe.start, Element::Pipe(pipe));
        }
    }

    Data::new(start_position, elements)
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

        let start = Position::new(x, y);
        let end = Position::new(x + len - 1, y);

        match data_type {
            DataType::Start => start_position = Some(start),
            DataType::Horizontal => {
                let east_west = Pipe::new(start, end, Direction::East);
                let west_east = Pipe::new(end, start, Direction::West);
                pipes.push(east_west);
                pipes.push(west_east);
            }
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

        let start = Position::new(x, y);
        let end = Position::new(x, y + len - 1);

        if let DataType::Vertical = data_type {
            let north_south = Pipe::new(start, end, Direction::South);
            let south_north = Pipe::new(end, start, Direction::North);
            pipes.push(north_south);
            pipes.push(south_north);
        }

        line = r;
        y = offset;
    }

    pipes
}

#[derive(Debug, Clone, Copy)]
enum Element {
    Pipe(Pipe),
    Corner(Corner),
    Start(Start),
}

impl Element {
    fn next_connection(&self, direction: Direction) -> Option<Connection> {
        match self {
            Self::Pipe(p) => p.next_connection(direction),
            Self::Corner(c) => c.next_connection(direction),
            Self::Start(_) => None,
        }
    }

    fn len(&self) -> i64 {
        match self {
            Self::Pipe(p) => p.len(),
            Self::Corner(_) => 1,
            Self::Start(_) => 1,
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pipe(p) => p.fmt(f),
            Self::Corner(c) => c.fmt(f),
            Self::Start(s) => s.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Start {
    position: Position,
}

impl Start {
    fn new(position: Position) -> Self {
        Self { position }
    }

    fn adjacencies(&self) -> [Connection; 4] {
        let p = self.position;

        [
            Connection::new(Position::new(p.x + 1, p.y), Direction::East),
            Connection::new(Position::new(p.x - 1, p.y), Direction::West),
            Connection::new(Position::new(p.x, p.y + 1), Direction::North),
            Connection::new(Position::new(p.x, p.y - 1), Direction::South),
        ]
    }
}

impl Display for Start {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S")
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn is_horizontal(&self) -> bool {
        self == &Self::East || self == &Self::West
    }

    fn is_vertical(&self) -> bool {
        self == &Self::North || self == &Self::South
    }
}

#[derive(Debug, Clone, Copy)]
struct Pipe {
    start: Position,
    end: Position,
    direction: Direction,
}

impl Pipe {
    fn new(start: Position, end: Position, direction: Direction) -> Self {
        Self {
            start,
            end,
            direction,
        }
    }

    fn len(&self) -> i64 {
        match self.direction {
            Direction::North | Direction::South => (self.start.y - self.end.y).abs() + 1,
            Direction::East | Direction::West => (self.start.x - self.end.x).abs() + 1,
        }
    }

    fn next_connection(&self, direction: Direction) -> Option<Connection> {
        let dirs = (self.direction, direction);
        let test = (
            self.direction.is_horizontal() && direction.is_horizontal(),
            self.direction.is_vertical() && direction.is_vertical(),
        );

        match test {
            (true, false) => match dirs {
                (Direction::East, Direction::East) => Some(Connection::new(
                    Position::new(self.end.x + 1, self.end.y),
                    Direction::East,
                )),
                (Direction::West, Direction::West) => Some(Connection::new(
                    Position::new(self.end.x - 1, self.end.y),
                    Direction::West,
                )),
                (Direction::East, Direction::West) => Some(Connection::new(
                    Position::new(self.start.x - 1, self.start.y),
                    Direction::West,
                )),
                (Direction::West, Direction::East) => Some(Connection::new(
                    Position::new(self.start.x + 1, self.start.y),
                    Direction::East,
                )),
                _ => None,
            },
            (false, true) => match dirs {
                (Direction::North, Direction::North) => Some(Connection::new(
                    Position::new(self.end.x, self.end.y - 1),
                    Direction::North,
                )),
                (Direction::South, Direction::South) => Some(Connection::new(
                    Position::new(self.end.x, self.end.y + 1),
                    Direction::South,
                )),
                (Direction::North, Direction::South) => Some(Connection::new(
                    Position::new(self.start.x, self.start.y + 1),
                    Direction::South,
                )),
                (Direction::South, Direction::North) => Some(Connection::new(
                    Position::new(self.start.x, self.start.y - 1),
                    Direction::North,
                )),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.direction {
            Direction::North | Direction::South => write!(f, "|"),
            _ => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Corner {
    SouthWest(Position),
    NorthWest(Position),
    NorthEast(Position),
    SouthEast(Position),
}

impl Corner {
    fn position(&self) -> Position {
        match *self {
            Self::SouthWest(p) => p,
            Self::NorthWest(p) => p,
            Self::NorthEast(p) => p,
            Self::SouthEast(p) => p,
        }
    }

    fn next_connection(&self, direction: Direction) -> Option<Connection> {
        match self {
            Self::SouthWest(p) => match direction {
                Direction::East => Some(Connection::new(
                    Position::new(p.x, p.y + 1),
                    Direction::South,
                )),
                Direction::North => Some(Connection::new(
                    Position::new(p.x - 1, p.y),
                    Direction::West,
                )),
                _ => None,
            },
            Self::NorthWest(p) => match direction {
                Direction::South => Some(Connection::new(
                    Position::new(p.x - 1, p.y),
                    Direction::West,
                )),
                Direction::East => Some(Connection::new(
                    Position::new(p.x, p.y - 1),
                    Direction::North,
                )),
                _ => None,
            },
            Self::NorthEast(p) => match direction {
                Direction::West => Some(Connection::new(
                    Position::new(p.x, p.y - 1),
                    Direction::North,
                )),
                Direction::South => Some(Connection::new(
                    Position::new(p.x + 1, p.y),
                    Direction::East,
                )),
                _ => None,
            },
            Self::SouthEast(p) => match direction {
                Direction::North => Some(Connection::new(
                    Position::new(p.x + 1, p.y),
                    Direction::East,
                )),
                Direction::West => Some(Connection::new(
                    Position::new(p.x, p.y + 1),
                    Direction::South,
                )),
                _ => None,
            },
        }
    }
}

impl Display for Corner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SouthWest(_) => write!(f, "7"),
            Self::NorthWest(_) => write!(f, "J"),
            Self::NorthEast(_) => write!(f, "L"),
            Self::SouthEast(_) => write!(f, "F"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Connection {
    position: Position,
    direction: Direction,
}

impl Connection {
    fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
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
