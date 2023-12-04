use anyhow::Result;
use nom::character::is_digit;
use std::path::Path;

const DAY: u32 = 3;
const YEAR: u32 = 2023;
#[derive(Debug)]

struct Coord {
    x: usize,
    y: usize,
}
#[derive(Debug)]
struct Part {
    number: u32,
    symbol_min: Coord,
    symbol_max: Coord,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord {
            x: if x < 0 { 0 } else { x as usize },
            y: if y < 0 { 0 } else { y as usize },
        }
    }
}

#[derive(Debug)]

struct Digit {
    digit: u32,
    coord: Coord,
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
    coord: Coord,
}

fn solve(input: &str) -> Result<String> {
    let symbols = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes().enumerate().flat_map(move |(x, c)| {
                if !is_digit(c) && c != '.' as u8 {
                    return Some(Symbol {
                        symbol: c as char,
                        coord: Coord { x, y },
                    });
                }
                None
            })
        })
        .collect::<Vec<Symbol>>();
    let digits = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes().enumerate().flat_map(move |(x, c)| {
                if is_digit(c) {
                    return Some(Digit {
                        digit: (c - ('0' as u8)) as u32,
                        coord: Coord { x, y },
                    });
                }
                None
            })
        })
        .collect::<Vec<Digit>>();
    let result = digits
        .windows(3)
        .map(|wind| {
            if wind[0].coord.y == wind[1].coord.y
                && wind[1].coord.y == wind[2].coord.y
                && wind[0].coord.x == wind[1].coord.x - 1
                && wind[1].coord.x == wind[2].coord.x - 1
            {
                return Part {
                    number: wind[0].digit * 100 + wind[1].digit * 10 + wind[2].digit,
                    symbol_min: Coord::new(wind[0].coord.x as i32 - 1, wind[0].coord.y as i32 - 1),
                    symbol_max: Coord::new(wind[2].coord.x as i32 + 1, wind[2].coord.y as i32 + 1),
                };
            }
            if wind[0].coord.y == wind[1].coord.y && wind[0].coord.x == wind[1].coord.x - 1 {
                return Part {
                    number: wind[0].digit * 10 + wind[1].digit * 1,
                    symbol_min: Coord::new(wind[0].coord.x as i32 - 1, wind[0].coord.y as i32 - 1),
                    symbol_max: Coord::new(wind[1].coord.x as i32 + 1, wind[1].coord.y as i32 + 1),
                };
            }
            Part {
                number: wind[0].digit,
                symbol_min: Coord::new(wind[0].coord.x as i32 - 1, wind[0].coord.y as i32 - 1),
                symbol_max: Coord::new(wind[0].coord.x as i32 + 1, wind[0].coord.y as i32 + 1),
            }
        })
        .filter(|part| {
            if let Some(_) = symbols.iter().find(|s| {
                s.coord.x >= part.symbol_min.x
                    && s.coord.x <= part.symbol_max.x
                    && s.coord.y >= part.symbol_min.y
                    && s.coord.y <= part.symbol_max.y
            }) {
                return true;
            }
            dbg!(part);
            false
        })
        .map(|part| part.number)
        .sum::<u32>();
    Ok(result.to_string())
}

fn solve_1(input: &str) -> Result<String> {
    solve(input)
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

        let result = "4361";
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

        let result = "-";
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
