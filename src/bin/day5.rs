use std::{path::Path, str::{FromStr, from_utf8}};
use anyhow::Result;

const DAY: u32 = 5;

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = std::num::ParseIntError;

    fn from_str(move_line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = move_line.split(" ").collect();
        let amount: usize = parts[1].parse()?;
        let from: usize = parts[3].parse()?;
        let to: usize = parts[5].parse()?;
        Ok(Move {
            amount,
            from,
            to,
        })
    }
}

fn get_items_part1(stack: &mut Vec<char>, len: usize) -> Vec<char> {
    let items: Vec<char> = stack.drain(len..).rev().collect();
    return items;
}

fn get_items_part2(stack: &mut Vec<char>, len: usize) -> Vec<char> {
    let items: Vec<char> = stack.drain(len..).collect();
    return items;
}

fn solve(input: &str, get_items: fn(stack: &mut Vec<char>, len: usize) -> Vec<char>) -> Result<String> {
    let splits: Vec<&str> = input.split("\n\n").collect();
    let pre_stacks = splits[0];
    let pre_moves = splits[1];
    let nr_stacks = (pre_stacks.lines().take(1).collect::<String>().len() + 1) / 4;
    let moves: Vec<Move> = pre_moves.lines().map(|l| l.parse().unwrap()).collect();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); nr_stacks];

    pre_stacks.lines().rev().skip(1).for_each(|l| {
        let parts: Vec<&str> = l.as_bytes().chunks(4).map(|l| from_utf8(l).unwrap()).collect();
        parts.iter().enumerate().for_each(|(i, &p)| {
            let c: char = p.chars().nth(1).unwrap();
            if c != ' ' {
                stacks[i].push(c);
            }
        })
    });
    moves.iter().for_each(|m| {
        let new_len = stacks[m.from - 1].len() - m.amount;
        let mut items = get_items(&mut stacks[m.from - 1], new_len);
        stacks[m.to - 1].append(&mut items);
    });
    let res: String = stacks.iter_mut().map(|s| s.pop().unwrap()).collect();
    Ok(res)
}

fn solve_1(input: &str) -> Result<String> {
    solve(input, get_items_part1)
}

fn solve_2(input: &str) -> Result<String> {
    solve(input, get_items_part2)
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

        let result = "CMZ";
        let solve = solve_1(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }

    #[test]
    fn example_second() {
        let input = example_input();

        let result = "MCD";
        let solve = solve_2(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }
}
