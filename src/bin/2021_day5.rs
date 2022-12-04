use std::{path::Path, collections::HashMap, str::FromStr};
use anyhow::Result;

const YEAR: u32 = 2021;
const DAY: u32 = 5;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Line {
    a: Point,
    b: Point,
}

impl FromStr for Point {
    type Err = std::num::ParseIntError;

    fn from_str(point: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = point.split(",").collect();
        let x: u32 = parts[0].parse()?;
        let y: u32 = parts[1].parse()?;

        Ok(Point { x, y })
    }
}

impl FromStr for Line {
    type Err = std::num::ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let a: Point = parts[0].parse()?;
        let b: Point = parts[1].parse()?;

        Ok(Line { a, b })
    }
}

fn get_complete_line(l: &Line) -> Vec<Point> {
    if l.a.x == l.b.x {
        let first = if l.a.y <= l.b.y { l.a.y} else { l.b.y };
        let last = if l.a.y <= l.b.y { l.b.y} else { l.a.y };
        let res = (first..=last).map(|n| Point{ x: l.a.x, y: n}).collect::<Vec<Point>>();
        return res;
    }
    if l.a.y == l.b.y {
        let first = if l.a.x <= l.b.x { l.a.x} else { l.b.x };
        let last = if l.a.x <= l.b.x { l.b.x} else { l.a.x };
        let res = (first..=last).map(|n| Point{ x: n, y: l.a.y}).collect::<Vec<Point>>();
        return res;
    }
    if l.a.x >= l.b.x {
        let len = l.a.x - l.b.x;
        return (0..=len).map(|n| Point{ x: l.a.x - n, y: l.a.y + (len - n)}).collect::<Vec<Point>>();
    }
    let len = l.b.x - l.a.x;
    let res = (0..=len).map(|n| Point{ x: l.b.x - n, y: l.b.y + (len - n)}).collect::<Vec<Point>>();
    return res;
}

fn solve(input: &str) -> Result<String> {
    let lines: Vec<Line> = input.lines()
        .map(|l| l.parse::<Line>().unwrap())
        .collect();
    let points: Vec<Point> = lines.iter()
        .flat_map(get_complete_line)
        .collect();
        let mut overlap = HashMap::new();
    for point in points {
        *overlap.entry(point).or_insert(0) += 1;
    }
    let result: usize = overlap.values().filter_map(|v| if *v > 1 { Some(1)} else { None}).sum();
    Ok(result.to_string())
}

fn solve_1(input: &str) -> Result<String> {
    let lines: Vec<Line> = input.lines()
        .map(|l| l.parse::<Line>().unwrap())
        .filter(|l| l.a.x == l.b.x || l.a.y == l.b.y)
        .collect();
    let points: Vec<Point> = lines.iter()
        .flat_map(get_complete_line)
        .collect();
        let mut overlap = HashMap::new();
    for point in points {
        *overlap.entry(point).or_insert(0) += 1;
    }
    let result: usize = overlap.values().filter_map(|v| if *v > 1 { Some(1)} else { None}).sum();
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

        let result = "5";
        let solve = solve_1(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }

    #[test]
    fn example_second() {
        let input = example_input();

        let result = "12";
        let solve = solve_2(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }
}
