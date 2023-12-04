use anyhow::Result;
use nom::InputLength;
use std::{collections::HashSet, path::Path};

const DAY: u32 = 9;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }

    fn up(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    fn down(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    fn right(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }

    fn left(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }

    fn diag_up_left(&self) -> Self {
        Self::new(self.x - 1, self.y + 1)
    }

    fn diag_down_left(&self) -> Self {
        Self::new(self.x - 1, self.y - 1)
    }

    fn diag_up_right(&self) -> Self {
        Self::new(self.x + 1, self.y + 1)
    }

    fn diag_down_right(&self) -> Self {
        Self::new(self.x + 1, self.y - 1)
    }

    fn follow(&self, new_head: &Self) -> Self {
        let diff_x: i32 = new_head.x - self.x;
        let diff_y: i32 = new_head.y - self.y;
        // dbg!(diff_x, diff_y);
        match (diff_x, diff_y) {
            (0, -2) => self.down(),
            (0, 2) => self.up(),
            (-2, 0) => self.left(),
            (2, 0) => self.right(),
            (1, -2) => self.diag_down_right(),
            (1, 2) => self.diag_up_right(),
            (-2, 1) => self.diag_up_left(),
            (2, 1) => self.diag_up_right(),
            (-1, -2) => self.diag_down_left(),
            (-1, 2) => self.diag_up_left(),
            (-2, -1) => self.diag_down_left(),
            (2, -1) => self.diag_down_right(),
            (-2, -2) => self.diag_down_left(),
            (-2, 2) => self.diag_up_left(),
            (2, -2) => self.diag_down_right(),
            (2, 2) => self.diag_up_right(),
            _ => self.clone(),
        }
    }
}

fn solve(input: &str) -> Result<String> {
    let moves: Option<Vec<(&str, &str)>> = input.lines().map(|s| s.split_once(" ")).collect();
    let moves = moves.unwrap();
    let mut head_pos: Vec<Coord> = Vec::new();
    let mut tail_pos: Vec<Coord> = Vec::new();
    let start = Coord::new(0, 0);
    head_pos.push(start.clone());
    tail_pos.push(start.clone());
    for m in moves {
        let count: i32 = m.1.parse().unwrap();
        for _ in 0..count {
            match m.0 {
                "U" => {
                    let new_head = head_pos.last().unwrap().up();
                    tail_pos.push(tail_pos.last().unwrap().follow(&new_head));
                    head_pos.push(new_head);
                }
                "D" => {
                    let new_head = head_pos.last().unwrap().down();
                    tail_pos.push(tail_pos.last().unwrap().follow(&new_head));
                    head_pos.push(new_head);
                }
                "R" => {
                    let new_head = head_pos.last().unwrap().right();
                    tail_pos.push(tail_pos.last().unwrap().follow(&new_head));
                    head_pos.push(new_head);
                }
                "L" => {
                    let new_head = head_pos.last().unwrap().left();
                    tail_pos.push(tail_pos.last().unwrap().follow(&new_head));
                    head_pos.push(new_head);
                }
                _ => {
                    panic!("I die.")
                }
            }
        }
    }
    let set: HashSet<(i32, i32)> = HashSet::from_iter(tail_pos.iter().map(|pos| (pos.x, pos.y)));
    Ok(format!("{}", set.len()).to_string())
}

fn solve_1(input: &str) -> Result<String> {
    solve(input)
}

fn solve_2(input: &str) -> Result<String> {
    let moves: Option<Vec<(&str, &str)>> = input.lines().map(|s| s.split_once(" ")).collect();
    let moves = moves.unwrap();
    let mut tails: Vec<Vec<Coord>> = vec![vec![Coord::new(0, 0)]; 10];
    for m in moves {
        let count: i32 = m.1.parse().unwrap();
        for _ in 0..count {
            match m.0 {
                "U" => {
                    let new_head = tails[0].last().unwrap().up();
                    tails[0].push(new_head);
                }
                "D" => {
                    let new_head = tails[0].last().unwrap().down();
                    tails[0].push(new_head);
                }
                "R" => {
                    let new_head = tails[0].last().unwrap().right();
                    tails[0].push(new_head);
                }
                "L" => {
                    let new_head = tails[0].last().unwrap().left();
                    tails[0].push(new_head);
                }
                _ => {
                    panic!("I die.")
                }
            }
            for i in 1..10 {
                let new_head = &tails[i - 1].last().unwrap().clone();
                let new_tail = tails[i].last().unwrap().follow(new_head);
                tails[i].push(new_tail);
            }
        }
    }
    let set: HashSet<(i32, i32)> = HashSet::from_iter(tails[9].iter().map(|pos| (pos.x, pos.y)));
    Ok(format!("{}", set.len()).to_string())
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

    fn example2_input() -> String {
        let path = format!("./data/{}.example2", DAY);
        let input_path = Path::new(&path);
        let input = std::fs::read_to_string(input_path).unwrap();
        input
    }

    #[test]
    fn example_first() {
        let input = example_input();

        let result = "13";
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

    #[test]
    fn example2_second() {
        let input = example2_input();

        let result = "36";
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
