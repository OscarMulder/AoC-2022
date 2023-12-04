use anyhow::Result;
use nom::{
    bytes::complete,
    bytes::complete::tag,
    bytes::complete::take_while1,
    character::{
        complete::{alpha1, digit1, line_ending, space1, u32},
        is_digit,
    },
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, tuple},
    IResult, InputIter,
};
use std::{cmp::Ordering, path::Path};

const DAY: u32 = 2;
const YEAR: u32 = 2023;

#[derive(Debug, Clone, Copy)]
struct Cube<'a> {
    color: &'a str,
    count: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, Clone)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

fn get_cube_count<'a>(cubes: &Vec<Cube<'a>>, color: &str) -> u32 {
    cubes
        .iter()
        .find(|cube| cube.color == color)
        .unwrap_or(&Cube {
            color: "",
            count: 0,
        })
        .count
}

fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (count, color)) = separated_pair(u32, tag(" "), alpha1)(input)?;
    Ok((input, Cube { color, count }))
}

fn round(input: &str) -> IResult<&str, Round> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;
    let round = Round {
        red: get_cube_count(&cubes, "red"),
        green: get_cube_count(&cubes, "green"),
        blue: get_cube_count(&cubes, "blue"),
    };
    Ok((input, round))
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round))(input)?;
    Ok((
        input,
        Game {
            id: id.parse().unwrap(),
            rounds,
        },
    ))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

fn solve(input: &str) -> Result<String> {
    let games = parse_games(input).expect("should parse");
    let result: u32 = games
        .1
        .iter()
        .map(|game| {
            let red = game
                .rounds
                .iter()
                .max_by(|a, b| a.red.cmp(&b.red))
                .unwrap()
                .red;
            let green = game
                .rounds
                .iter()
                .max_by(|a, b| a.green.cmp(&b.green))
                .unwrap()
                .green;
            let blue = game
                .rounds
                .iter()
                .max_by(|a, b| a.blue.cmp(&b.blue))
                .unwrap()
                .blue;
            red * green * blue
        })
        .sum();
    Ok(result.to_string())
}

fn solve_1(input: &str) -> Result<String> {
    let games = parse_games(input).expect("should parse");
    let compare = Round {
        red: 12,
        green: 13,
        blue: 14,
    };
    let result: usize = games
        .1
        .iter()
        .filter(|game| {
            game.rounds.iter().all(|round| {
                round.red <= compare.red
                    && round.blue <= compare.blue
                    && round.green <= compare.green
            })
        })
        .map(|game| game.id)
        .sum();
    Ok(result.to_string())
}

fn solve_2(input: &str) -> Result<String> {
    solve(input)
}

fn input() -> String {
    let path = format!("./data/{}/{}.input", YEAR, DAY);
    let input_path = Path::new(&path);
    let input = std::fs::read_to_string(input_path).unwrap();
    input
}

fn main() {
    let input = input();

    let solve_first = solve_1(&input);
    match solve_first {
        Ok(res) => println!("Day {}, first puzzle: {}", DAY, res),
        Err(e) => println!("{:?}", e),
    }

    let solve_second = solve_2(&input);
    match solve_second {
        Ok(res) => println!("Day {}, second puzzle: {}", DAY, res),
        Err(e) => println!("{:?}", e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> String {
        let path = format!("./data/{}/{}.example", YEAR, DAY);
        let input_path = Path::new(&path);
        let input = std::fs::read_to_string(input_path).unwrap();
        input
    }

    #[test]
    fn example_first() {
        let input = example_input();

        let result = "8";
        let solve = solve_1(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }

    #[test]
    fn multi_example_first() {
        let inputs = [("", "-"), ("", "-"), ("", "-"), ("", "-")];
        for (input, result) in inputs {
            assert_eq!(solve_1(input).unwrap(), result);
        }
    }

    #[test]
    fn example_second() {
        let input = example_input();

        let result = "2286";
        let solve = solve_2(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }

    #[test]
    fn multi_example_second() {
        let inputs = [("", "-"), ("", "-"), ("", "-"), ("", "-")];
        for (input, result) in inputs {
            assert_eq!(solve_2(input).unwrap(), result);
        }
    }
}
