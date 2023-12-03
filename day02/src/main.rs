use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, newline, space0, u64},
    combinator::{eof, map, opt},
    multi::{many0, many_till},
    sequence::tuple,
    Finish, IResult,
};
use std::{
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
    let file = File::open("day02.txt")?;
    let reader = BufReader::new(file);
    let sum = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| parser(&line))
        .filter_map(|game| {
            let (red, green, blue) = game.max_cubes();

            if red <= 12 && green <= 13 && blue <= 14 {
                Some(game.id())
            } else {
                None
            }
        })
        .sum();

    Ok(sum)
}

fn part02() -> anyhow::Result<u64> {
    let file = File::open("day02.txt")?;
    let reader = BufReader::new(file);
    let sum = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| parser(&line))
        .map(|game| {
            let (red, green, blue) = game.max_cubes();
            red * green * blue
        })
        .sum();

    Ok(sum)
}

#[derive(Debug, PartialEq)]
enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq)]
struct Cube(u64, Colour);

#[derive(Debug, PartialEq)]
struct Game(u64, Vec<Vec<Cube>>);

impl Game {
    fn id(&self) -> u64 {
        self.0
    }

    fn max_cubes(&self) -> (u64, u64, u64) {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let cubes = self.1.iter().flatten();
        for Cube(count, colour) in cubes {
            let count = *count;

            match colour {
                Colour::Red => {
                    if count > red {
                        red = count;
                    }
                }
                Colour::Green => {
                    if count > green {
                        green = count;
                    }
                }
                Colour::Blue => {
                    if count > blue {
                        blue = count;
                    }
                }
            }
        }

        (red, green, blue)
    }
}

fn parser(input: &str) -> Option<Game> {
    game_record(input).finish().map(|(_, game)| game).ok()
}

fn record_start(input: &str) -> IResult<&str, u64> {
    let (input, (_, _, _, num, _, _)) =
        tuple((multispace0, tag("Game"), space0, u64, char(':'), space0))(input)?;

    Ok((input, num))
}

fn game_record(input: &str) -> IResult<&str, Game> {
    let (input, (id, sets)) = tuple((record_start, cube_set_terminators))(input)?;
    Ok((input, Game(id, sets)))
}

fn red(input: &str) -> IResult<&str, Colour> {
    let (input, _) = tag("red")(input)?;
    Ok((input, Colour::Red))
}

fn green(input: &str) -> IResult<&str, Colour> {
    let (input, _) = tag("green")(input)?;
    Ok((input, Colour::Green))
}

fn blue(input: &str) -> IResult<&str, Colour> {
    let (input, _) = tag("blue")(input)?;
    Ok((input, Colour::Blue))
}

fn colour(input: &str) -> IResult<&str, Colour> {
    alt((red, green, blue))(input)
}

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (num, _, colour)) = tuple((u64, space0, colour))(input)?;

    Ok((input, Cube(num, colour)))
}

fn separator(input: &str) -> IResult<&str, ()> {
    let (input, _) = opt(tuple((char(','), space0)))(input)?;
    Ok((input, ()))
}

fn cube_separator(input: &str) -> IResult<&str, Cube> {
    let (input, (cube, _, _)) = tuple((cube, space0, separator))(input)?;
    Ok((input, cube))
}

fn cube_set(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = many0(cube_separator)(input)?;
    Ok((input, cubes))
}

fn terminator(input: &str) -> IResult<&str, ()> {
    let (input, _) = opt(tuple((char(';'), space0)))(input)?;
    Ok((input, ()))
}

fn cube_set_terminator(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, (cubes, _)) = tuple((cube_set, terminator))(input)?;
    Ok((input, cubes))
}

fn cube_set_terminators(input: &str) -> IResult<&str, Vec<Vec<Cube>>> {
    let newline = map(tuple((newline, multispace0)), |_| ());
    let eof = map(eof, |_| ());
    let end = alt((newline, eof));
    let (input, (cubes, _)) = many_till(cube_set_terminator, end)(input)?;
    Ok((input, cubes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn individual_colour_test() {
        let res = red("red").finish().unwrap();
        assert_eq!(res, ("", Colour::Red));

        let res = green("green").finish().unwrap();
        assert_eq!(res, ("", Colour::Green));

        let res = blue("blue").finish().unwrap();
        assert_eq!(res, ("", Colour::Blue));
    }

    #[test]
    fn colour_test() {
        let res = colour("red").finish().unwrap();
        assert_eq!(res, ("", Colour::Red));

        let res = colour("green").finish().unwrap();
        assert_eq!(res, ("", Colour::Green));

        let res: (&str, Colour) = colour("blue").finish().unwrap();
        assert_eq!(res, ("", Colour::Blue));
    }

    #[test]
    fn cube_test() {
        let res = cube("3 blue").finish().unwrap();
        assert_eq!(res, ("", Cube(3, Colour::Blue)));

        let res = cube("2 red").finish().unwrap();
        assert_eq!(res, ("", Cube(2, Colour::Red)));
    }

    #[test]
    fn cube_separator_test() {
        let res = cube_separator("4 blue,").finish().unwrap();
        assert_eq!(res, ("", Cube(4, Colour::Blue)));

        let res = cube_separator("2 red").finish().unwrap();
        assert_eq!(res, ("", Cube(2, Colour::Red)));

        let res = cube_separator("1000 green,            ").finish().unwrap();
        assert_eq!(res, ("", Cube(1000, Colour::Green)));

        let res = cube_separator("9 green          ,            ")
            .finish()
            .unwrap();
        assert_eq!(res, ("", Cube(9, Colour::Green)));
    }

    #[test]
    fn cube_set_test() {
        let res = cube_set("1 blue, 2 red, 3 green").finish().unwrap();
        assert_eq!(
            res,
            (
                "",
                vec![
                    Cube(1, Colour::Blue),
                    Cube(2, Colour::Red),
                    Cube(3, Colour::Green)
                ]
            )
        )
    }

    #[test]
    fn cube_set_terminator_test() {
        let res = cube_set_terminator("1 blue, 2 red, 3 green;")
            .finish()
            .unwrap();
        assert_eq!(
            res,
            (
                "",
                vec![
                    Cube(1, Colour::Blue),
                    Cube(2, Colour::Red),
                    Cube(3, Colour::Green)
                ]
            )
        );

        let res = cube_set_terminator("1 blue, 2 red").finish().unwrap();
        assert_eq!(
            res,
            ("", vec![Cube(1, Colour::Blue), Cube(2, Colour::Red),])
        );

        let res = cube_set_terminator("1 blue, 2 red;     3 green")
            .finish()
            .unwrap();
        assert_eq!(
            res,
            (
                "3 green",
                vec![Cube(1, Colour::Blue), Cube(2, Colour::Red),]
            )
        );
    }

    #[test]
    fn cube_set_terminators_test() {
        let res = cube_set_terminators("1 blue, 2 red, 3 green; 4 red")
            .finish()
            .unwrap();
        assert_eq!(
            res,
            (
                "",
                vec![
                    vec![
                        Cube(1, Colour::Blue),
                        Cube(2, Colour::Red),
                        Cube(3, Colour::Green)
                    ],
                    vec![Cube(4, Colour::Red)]
                ]
            )
        );
    }

    #[test]
    fn game_record_test() {
        let res = game_record("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green    ")
            .finish()
            .unwrap();
        assert_eq!(
            res,
            (
                "",
                Game(
                    1,
                    vec![
                        vec![Cube(3, Colour::Blue), Cube(4, Colour::Red)],
                        vec![
                            Cube(1, Colour::Red),
                            Cube(2, Colour::Green),
                            Cube(6, Colour::Blue)
                        ],
                        vec![Cube(2, Colour::Green)]
                    ]
                )
            )
        )
    }
}
