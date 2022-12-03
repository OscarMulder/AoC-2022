use std::{path::Path, collections::HashSet};
use anyhow::Result;

const DAY: u32 = 3;

fn solve_1(input: &str) -> Result<String> {
    let res: u32 = input.lines()
        .map(|l| {
            let len = l.len();
            let first = l[..(len / 2 + 1)].to_string();
            let second = l[(len / 2)..].to_string();
            let duplicate = first.chars().find(|c| second.chars().find(|ch| ch == c) != None);
            if let Some(dup) = duplicate {
                if dup.is_ascii_lowercase() {
                    return dup as u32 - 'a' as u32 + 1;
                } else {
                    return dup as u32 - 'A' as u32 + 27;
                }
            }
            0
        })
        .sum();
    Ok(res.to_string())
}

fn solve_2(input: &str) -> Result<String> {
    let res: u32 = input.lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            let set: HashSet<char> = group[0].chars().filter(|c| group[1].chars().find(|ch| ch == c) != None).collect();
            let duplicate = group[2].chars().find(|c| set.contains(c));
            if let Some(dup) = duplicate {
                if dup.is_ascii_lowercase() {
                    return dup as u32 - 'a' as u32 + 1;
                } else {
                    return dup as u32 - 'A' as u32 + 27;
                }
            }
            0
        })
        .sum();
    Ok(res.to_string())
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

        let result = "157";
        let solve = solve_1(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }

    #[test]
    fn example_second() {
        let input = example_input();

        let result = "70";
        let solve = solve_2(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }
}
