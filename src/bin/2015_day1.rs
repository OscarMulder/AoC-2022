use std::path::Path;
use anyhow::Result;

const DAY: u32 = 1;

fn solve(input: &str) -> Result<String> {
    let res: i32 = input.chars()
        .map(|c| {
            match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
        })
        .sum();
    Ok(res.to_string())
} 

fn solve_1(input: &str) -> Result<String> {
    let res: i32 = input.chars()
        .map(|c| {
            match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
        })
        .sum();
    Ok(res.to_string())
}

fn solve_2(input: &str) -> Result<String> {
    let res: Vec<i32> = input.chars()
        .map(|c| {
            match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
        })
        .collect();
    let mut floor: i32 = 0;
    for (i, v) in res.iter().enumerate() {
        floor += v;
        if floor < 0 {
            return Ok((i + 1).to_string());
        }
    }
    Ok("error".to_string())
}

fn input() -> String {
    let path = format!("./data/2015/{}.input", DAY);
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
        let path = format!("./data/2015/{}.example", DAY);
        let input_path = Path::new(&path);
        let input = std::fs::read_to_string(input_path).unwrap();
        input
    }

    #[test]
    fn example_first() {
        let input = example_input();

        let result = "-3";
        let solve = solve_1(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }

    #[test]
    fn example_second() {
        let input = example_input();

        let result = "1";
        let solve = solve_2(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }
}
