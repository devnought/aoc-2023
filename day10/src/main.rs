use nom::{
    branch::alt, character::complete::char, combinator::value, multi::many1, Finish, IResult,
};
use std::{cmp::Eq, collections::HashMap, fs};

fn main() -> anyhow::Result<()> {
    let data_raw = fs::read_to_string("day10.txt")?;
    let data = parser(data_raw);

    let res = part01(&data);
    println!("Part 01: {res}");

    let res = part02(&data);
    println!("Part 02: {res}");

    Ok(())
}

fn part01(_data: &Data) -> i64 {
    0
}

fn part02(_data: &Data) -> i64 {
    0
}

#[derive(Debug)]
struct Data {
    start: Start,
    elements: HashMap<Position, PipeCorner>,
}

impl Data {
    fn new(start: Position, elements: HashMap<Position, PipeCorner>) -> Self {
        Self {
            start: Start::new(start),
            elements,
        }
    }

    fn path_distance(&self) -> i64 {
        let start_adjacencies = self.start.adjacencies();
        let (mut element, mut connection) = start_adjacencies
            .into_iter()
            .filter_map(|connection| {
                let element = self.elements.get(&connection.position)?;

                if element.valid_orientation(connection.orientation) {
                    Some((*element, connection))
                } else {
                    None
                }
            })
            .next()
            .expect("No adjacent to start?");

        let mut distance = 0;

        while element.position() != self.start.position {
            distance += element.len();

            println!("Current element: {element:?}");
            let next_connection = element.next_connection(connection);

            println!("Next position:   {:?}", next_connection.position);

            element = *self.elements.get(&next_connection.position).unwrap();
            connection = next_connection;
        }

        distance
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
            let reverse_pipe = pipe.reverse();
            elements.insert(pipe.start, PipeCorner::Pipe(pipe));
            elements.insert(reverse_pipe.start, PipeCorner::Pipe(reverse_pipe));
        }

        for corner in corners {
            elements.insert(corner.position(), PipeCorner::Corner(corner));
        }
    }

    // Parse each constructed column for vertical pipes.
    // All other relevant data was pulled from the row parsing.
    for (x, column) in columns_raw.iter().enumerate() {
        let pipes = parse_column(x as i64, &column);

        for pipe in pipes {
            let reverse_pipe = pipe.reverse();
            elements.insert(pipe.start, PipeCorner::Pipe(pipe));
            elements.insert(reverse_pipe.start, PipeCorner::Pipe(reverse_pipe));
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
            DataType::Horizontal => pipes.push(Pipe::new(start, end, Orientation::Horizontal)),
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

        match data_type {
            DataType::Vertical => pipes.push(Pipe::new(start, end, Orientation::Vertical)),
            _ => {}
        }

        line = r;
        y = offset;
    }

    pipes
}

#[derive(Debug, Clone, Copy)]
enum PipeCorner {
    Pipe(Pipe),
    Corner(Corner),
}

impl PipeCorner {
    fn position(&self) -> Position {
        match self {
            Self::Pipe(p) => p.start,
            Self::Corner(c) => c.position(),
        }
    }

    fn next_connection(&self, connection: Connection) -> Connection {
        match self {
            Self::Pipe(p) => p.end_connection(),
            Self::Corner(c) => c.output_connection(connection),
        }
    }

    fn valid_orientation(&self, orientation: Orientation) -> bool {
        match self {
            Self::Pipe(p) => p.orientation == orientation,
            Self::Corner(c) => c.valid_orientation(orientation),
        }
    }

    fn len(&self) -> i64 {
        match self {
            Self::Pipe(p) => p.len(),
            Self::Corner(_) => 1,
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
            Connection::new(Position::new(p.x + 1, p.y), Orientation::Horizontal),
            Connection::new(Position::new(p.x - 1, p.y), Orientation::Horizontal),
            Connection::new(Position::new(p.x, p.y + 1), Orientation::Vertical),
            Connection::new(Position::new(p.x, p.y - 1), Orientation::Vertical),
        ]
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
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy)]
struct Pipe {
    start: Position,
    end: Position,
    orientation: Orientation,
}

impl Pipe {
    fn new(start: Position, end: Position, orientation: Orientation) -> Self {
        Self {
            start,
            end,
            orientation,
        }
    }

    fn reverse(&self) -> Self {
        Self::new(self.end, self.start, self.orientation)
    }

    fn len(&self) -> i64 {
        match self.orientation {
            Orientation::Horizontal => (self.start.x - self.end.x).abs(),
            Orientation::Vertical => (self.start.y - self.end.y).abs(),
        }
    }

    fn end_connection(&self) -> Connection {
        match self.orientation {
            Orientation::Horizontal => {
                let end = self.end;
                let range = self.start.x..end.x + 1;

                let x = if range.contains(&(end.x + 1)) {
                    end.x - 1
                } else {
                    end.x + 1
                };

                Connection::new(Position::new(x, end.y), self.orientation)
            }
            Orientation::Vertical => {
                let end = self.end;
                let range = self.start.y..end.y + 1;

                let y = if range.contains(&(end.y + 1)) {
                    end.y - 1
                } else {
                    end.y + 1
                };

                Connection::new(Position::new(end.x, y), self.orientation)
            }
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

    fn connections(&self) -> (Connection, Connection) {
        match self {
            Self::SouthWest(p) => (
                Connection::new(Position::new(p.x - 1, p.y), Orientation::Horizontal),
                Connection::new(Position::new(p.x, p.y + 1), Orientation::Vertical),
            ),
            Self::NorthWest(p) => (
                Connection::new(Position::new(p.x, p.y - 1), Orientation::Vertical),
                Connection::new(Position::new(p.x - 1, p.y), Orientation::Horizontal),
            ),
            Self::NorthEast(p) => (
                Connection::new(Position::new(p.x + 1, p.y), Orientation::Horizontal),
                Connection::new(Position::new(p.x, p.y - 1), Orientation::Vertical),
            ),
            Self::SouthEast(p) => (
                Connection::new(Position::new(p.x, p.y - 1), Orientation::Vertical),
                Connection::new(Position::new(p.x + 1, p.y), Orientation::Horizontal),
            ),
        }
    }

    fn valid_orientation(&self, orientation: Orientation) -> bool {
        let (one, two) = self.connections();

        orientation == one.orientation || orientation == two.orientation
    }

    fn valid_connection(&self, connection: Connection) -> bool {
        let (one, two) = self.connections();

        connection == one || connection == two
    }

    fn output_connection(&self, connection: Connection) -> Connection {
        let (one, two) = self.connections();

        if connection == one {
            two
        } else if connection == two {
            one
        } else {
            panic!("No matching connection for {connection:?}");
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Connection {
    position: Position,
    orientation: Orientation,
}

impl Connection {
    fn new(position: Position, orientation: Orientation) -> Self {
        Self {
            position,
            orientation,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let raw = String::from(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );

        let data = parser(raw);

        for element in data.elements.values() {
            match element {
                PipeCorner::Pipe(p) => {
                    let start = p.start;
                    let end = p.end;

                    println!("Pipe:   ({},{}) -> ({},{})", start.x, start.y, end.x, end.y);
                }
                PipeCorner::Corner(c) => {
                    let position = c.position();
                    println!("Corner: ({},{})", position.x, position.y);
                }
                _ => {}
            }
        }

        let path_distance = data.path_distance();

        assert_eq!(8, path_distance);
    }
}
