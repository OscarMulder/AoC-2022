use anyhow::Result;
use nom::*;
use std::path::Path;

const DAY: u32 = 11;

struct Monkey {
    nr: i32,
    starting_items: Vec<i32>,
    operation: String,
    div_test: i32,
    true_throw: i32,
    false_throw: i32,
}

impl Monkey {
    fn new(
        nr: i32,
        starting_items: Vec<i32>,
        operation: String,
        div_test: i32,
        true_throw: i32,
        false_throw: i32,
    ) -> Self {
        Monkey {
            nr,
            starting_items,
            operation,
            div_test,
            true_throw,
            false_throw,
        }
    }
}

fn parse_monkey_nr(input: &str) ->

fn parse(input: &str) -> Vec<Monkey> {
    let mut monkeys = input
}

fn solve(input: &str) -> Result<String> {
    Ok("".to_string())
}

fn solve_1(input: &str) -> Result<String> {
    solve(input)
}

fn solve_2(input: &str) -> Result<String> {
    solve(input)
}

fn input() -> String {
    let path = format!("./data/{}.input", DAY);
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
        let path = format!("./data/{}.example", DAY);
        let input_path = Path::new(&path);
        let input = std::fs::read_to_string(input_path).unwrap();
        input
    }

    #[test]
    fn example_first() {
        let input = example_input();

        let result = "10605";
        let solve = solve_1(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
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
