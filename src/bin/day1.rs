use anyhow::Result;
use std::path::Path;

const DAY: u32 = 1;
const YEAR: u32 = 2023;

fn solve(input: &str) -> Result<String> {
    let result = input
        .lines()
        .map(|s| {
            let first_digit = s.chars().filter_map(|c| c.to_digit(10)).next().unwrap();
            let last_digit = s.chars().filter_map(|c| c.to_digit(10)).last().unwrap();
            println!("{} -> {}{}", s, first_digit, last_digit);
            (first_digit, last_digit)
        })
        .fold(0, |acc, i| acc + (i.0 * 10) + i.1);
    Ok(result.to_string())
}

fn convert_to_digit(line: &str) -> u32 {
    let conversion = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];
    if let Some(found) = conversion
        .iter()
        .map(|(s, n)| {
            if let Some(found) = line.find(s) {
                Some((found, s, *n))
            } else {
                None
            }
        })
        .flatten()
        .min_by(|a, b| a.0.cmp(&b.0))
    {
        let first = found.2;
        if let Some(found) = conversion
            .iter()
            .map(|(s, n)| {
                if let Some(found) = line.rfind(s) {
                    Some((found, s, *n))
                } else {
                    None
                }
            })
            .flatten()
            .max_by(|a, b| a.0.cmp(&b.0))
        {
            let second = found.2;
            return (first * 10) + second;
        }
    }
    0
}

fn solve_1(input: &str) -> Result<String> {
    solve(input)
}

fn solve_2(input: &str) -> Result<String> {
    let result: u32 = input.lines().map(|line| convert_to_digit(line)).sum();
    Ok(result.to_string())
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

    fn example2_input() -> String {
        let path = format!("./data/{}/{}.example2", YEAR, DAY);
        let input_path = Path::new(&path);
        let input = std::fs::read_to_string(input_path).unwrap();
        input
    }

    #[test]
    fn example_first() {
        let input = example_input();

        let result = "142";
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
        let input = example2_input();

        let result = "281";
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
