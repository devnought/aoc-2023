use nom::{
    branch::alt,
    character::complete::{anychar, char, u64},
    combinator::{map, verify},
    Finish, IResult,
};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    let res = part01()?;
    println!("Part 01: {res}");

    let res = part02()?;
    println!("Part 02: {res}");

    Ok(())
}

fn part01() -> anyhow::Result<u64> {
    let ParsedData { symbols, values } = symbols_and_values()?;
    let mut seen_values = HashSet::new();

    let sum = symbols
        .iter()
        .flat_map(|symbol| symbol.adjacent_cells())
        .filter_map(|coords| {
            let stored_value = values.get(&coords)?;

            if seen_values.contains(&stored_value.id) {
                None
            } else {
                seen_values.insert(stored_value.id);
                Some(stored_value.value)
            }
        })
        .sum();

    Ok(sum)
}

fn part02() -> anyhow::Result<u64> {
    let ParsedData { symbols, values } = symbols_and_values()?;
    let iter = symbols.iter().filter(|symbol| symbol.value() == '*');
    let mut sum = 0;

    for symbol in iter {
        let mut seen_values = HashMap::new();

        for coords in symbol.adjacent_cells() {
            if let Some(stored_value) = values.get(&coords) {
                if seen_values.contains_key(&stored_value.id) {
                    continue;
                }

                seen_values.insert(stored_value.id, stored_value.value);
            }
        }

        if seen_values.len() == 2 {
            sum += seen_values.values().product::<u64>();
        }
    }

    Ok(sum)
}

#[derive(Debug)]
struct ParsedData {
    symbols: Vec<Symbol>,
    values: HashMap<(i64, i64), StoredValue>,
}

fn symbols_and_values() -> anyhow::Result<ParsedData> {
    let file = File::open("day03.txt")?;
    let reader = BufReader::new(file);

    let iter = reader
        .lines()
        .map_while(Result::ok)
        .enumerate()
        .map(|(y, line_value)| line(y as i64, &line_value));

    let mut values = HashMap::new();
    let mut symbols = Vec::new();

    for (v, s) in iter {
        let stored_values = v.iter().map(|value| {
            let stored_value = StoredValue {
                id: value.start,
                value: value.value,
            };

            (value.coords(), stored_value)
        });

        values.extend(stored_values);
        symbols.extend(s);
    }

    Ok(ParsedData { symbols, values })
}

#[derive(Debug)]
struct StoredValue {
    id: (i64, i64),
    value: u64,
}

#[derive(Debug)]
struct Symbol(char, i64, i64);

impl Symbol {
    fn value(&self) -> char {
        self.0
    }

    fn adjacent_cells(&self) -> [(i64, i64); 8] {
        let x = self.1;
        let y = self.2;

        let mut index = 0;
        let mut positions = [(0, 0); 8];

        for x_pos in (x - 1)..(x + 2) {
            for y_pos in (y - 1)..(y + 2) {
                if x_pos == x && y_pos == y {
                    continue;
                }

                positions[index] = (x_pos, y_pos);
                index += 1;
            }
        }

        positions
    }
}

#[derive(Debug)]
struct DataValue {
    start: (i64, i64),
    value: u64,
    x: i64,
    y: i64,
}

impl DataValue {
    fn coords(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

#[derive(Debug)]
enum DataRaw {
    Blank,
    Symbol(char),
    Value(u64),
}

fn line(y: i64, input: &str) -> (Vec<DataValue>, Vec<Symbol>) {
    let mut values = Vec::new();
    let mut symbols = Vec::new();

    let mut x = 0;
    let mut remaining = input;

    while !remaining.is_empty() {
        let (r, data) = data(remaining).finish().unwrap();
        let len = (remaining.len() - r.len()) as i64;

        match data {
            DataRaw::Blank => {}
            DataRaw::Symbol(s) => {
                let value = Symbol(s, x, y);
                symbols.push(value);
            }
            DataRaw::Value(value) => {
                for new_x in x..(x + len) {
                    let value = DataValue {
                        start: (x, y),
                        value,
                        x: new_x,
                        y,
                    };
                    values.push(value);
                }
            }
        }

        x += len;
        remaining = r;
    }

    (values, symbols)
}

fn symbol(input: &str) -> IResult<&str, DataRaw> {
    let parser = verify(anychar, |c| !c.is_alphanumeric() && *c != '.');
    map(parser, DataRaw::Symbol)(input)
}

fn blank(input: &str) -> IResult<&str, DataRaw> {
    map(char('.'), |_| DataRaw::Blank)(input)
}

fn number(input: &str) -> IResult<&str, DataRaw> {
    map(u64, DataRaw::Value)(input)
}

fn data(input: &str) -> IResult<&str, DataRaw> {
    alt((blank, symbol, number))(input)
}
