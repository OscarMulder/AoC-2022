use std::{path::Path, collections::{HashSet, VecDeque}};
use anyhow::Result;

const DAY: u32 = 6;

fn solve(input: &str, nr_unique: usize) -> Result<String> {
    let mut som: VecDeque<char> = VecDeque::new();
    for (i, d) in input.chars().enumerate() {
        som.push_back(d);
        if som.len() == nr_unique {
            let mut unique = HashSet::new();
            if som.iter().all(|x| unique.insert(x)) {
                return Ok((i + 1).to_string())
            }
            som.pop_front();
        }
    }
    Ok("".to_string())
}

fn solve_1(input: &str) -> Result<String> {
    solve(input, 4)
}

fn solve_2(input: &str) -> Result<String> {
    solve(input, 14)
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

        let result = "7";
        let solve = solve_1(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }

    #[test]
    fn multi_example_first() {
        let inputs = [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", "5"),
            ("nppdvjthqldpwncqszvftbrmjlhg", "6"),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10"),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11"),
        ];
        for (input, result) in inputs {
            assert_eq!(solve_1(input).unwrap(), result);
        }
    }

    #[test]
    fn example_second() {
        let input = example_input();

        let result = "19";
        let solve = solve_2(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }

    #[test]
    fn multi_example_second() {
        let inputs = [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", "23"),
            ("nppdvjthqldpwncqszvftbrmjlhg", "23"),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "29"),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "26"),
        ];
        for (input, result) in inputs {
            assert_eq!(solve_2(input).unwrap(), result);
        }
    }
}
