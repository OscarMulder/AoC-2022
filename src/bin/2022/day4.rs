use anyhow::Result;
use std::path::Path;

const DAY: u32 = 4;

fn check_pairs_1(first: &Vec<u32>, second: &Vec<u32>) -> Option<bool> {
    if (first[0] >= second[0] && first[1] <= second[1])
        || (second[0] >= first[0] && second[1] <= first[1])
    {
        return Some(true);
    }
    None
}

fn check_pairs_2(first: &Vec<u32>, second: &Vec<u32>) -> Option<bool> {
    if (first[0] >= second[0] && first[0] <= second[0])
        || (first[1] <= second[1] && first[1] >= second[0])
        || (second[0] >= first[0] && second[0] <= first[1])
        || (second[1] <= first[1] && second[1] >= first[0])
    {
        return Some(true);
    }
    None
}

fn solve(
    input: &str,
    check_pairs: fn(first: &Vec<u32>, second: &Vec<u32>) -> Option<bool>,
) -> Result<String> {
    let res = input
        .lines()
        .filter_map(|l| {
            let pairs: Vec<&str> = l.split(",").collect();
            let first: Vec<u32> = pairs[0]
                .split("-")
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            let second: Vec<u32> = pairs[1]
                .split("-")
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            check_pairs(&first, &second)
        })
        .collect::<Vec<bool>>()
        .len();
    Ok(res.to_string())
}

fn solve_1(input: &str) -> Result<String> {
    solve(input, check_pairs_1)
}

fn solve_2(input: &str) -> Result<String> {
    solve(input, check_pairs_2)
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

        let result = "2";
        let solve = solve_1(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }

    #[test]
    fn example_second() {
        let input = example_input();

        let result = "4";
        let solve = solve_2(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }
}
