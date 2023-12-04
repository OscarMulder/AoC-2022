use std::{path::Path, fmt};
use std::fmt::Write;
use anyhow::Result;
use pathfinding::prelude;

const DAY: u32 = 12;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    pub grid: Vec<Vec<u32>>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::from("");
        let size = self.size();
        for it in self.grid.iter() {
            for (i, num) in it.iter().enumerate() {
                if i == (size - 1) {
                    write!(str, "{:>2}\n", num).unwrap()
                } else {
                    write!(str, "{:>2} ", num).unwrap()
                }
            }
        }
        write!(f, "{}", str)
    }
}

impl Node {
    pub fn move_left(&mut self) -> Result<(), ()> {
        let (y, x) = self.find(0);
        if x == 0 {
            return Err(());
        }
        self.grid[y].swap(x, x - 1);
        Ok(())
    }

    pub fn move_right(&mut self) -> Result<(), ()> {
        let size = self.size();
        let (y, x) = self.find(0);
        if x == size - 1 {
            return Err(());
        }
        self.grid[y].swap(x, x + 1);
        Ok(())
    }

    pub fn move_up(&mut self) -> Result<(), ()> {
        let (y, x) = self.find(0);
        if y == 0 {
            return Err(());
        }
        let tmp = self.grid[y - 1][x];
        self.grid[y - 1][x] = self.grid[y][x];
        self.grid[y][x] = tmp;
        Ok(())
    }

    pub fn move_down(&mut self) -> Result<(), ()> {
        let size = self.size();
        let (y, x) = self.find(0);
        if y == size - 1 {
            return Err(());
        }
        let tmp = self.grid[y + 1][x];
        self.grid[y + 1][x] = self.grid[y][x];
        self.grid[y][x] = tmp;
        Ok(())
    }


    fn successors(&self) -> Vec<(Node, u32)> {
        //! returns the (max 4) possible Node's for UP, DOWN, LEFT, RIGHT
        //! including their cost values (always current + 1)

        let mut children: Vec<(Node, u32)> = vec![];
        let (y, x) = self.find(0);
        let size = self.size();
        if y > 0 {
            let mut up = self.clone();
            if let Ok(_) = up.move_up() {
                children.push((up, 1));
            }
        }
        if y < size - 1 {
            let mut down = self.clone();
            if let Ok(_) = down.move_down() {
                children.push((down, 1));
            }
        }
        if x < size - 1 {
            let mut right = self.clone();
            if let Ok(_) = right.move_right() {
                children.push((right, 1));
            }
        }
        if x > 0 {
            let mut left = self.clone();
            if let Ok(_) = left.move_left() {
                children.push((left, 1));
            }
        }
        children
    }

    pub fn size(&self) -> usize {
        self.grid.len()
    }

    fn man_distance(&self, goal: &Node) -> u32 {
        // manhattan distance first, so the sum of the distance from each number in this to the
        // correct position in goal
        let size: usize = self.grid.len();
        let mut manhattan_sum: u32 = 0;
        for y in 0..size {
            for x in 0..size {
                let (gy, gx) = self.find(goal.grid[y][x]);
                manhattan_sum +=
                    ((gy as i32 - y as i32).abs() + (gx as i32 - x as i32).abs()) as u32;
            }
        }
        return manhattan_sum;
    }
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

        let result = "-";
        let solve = solve_1(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }

    #[test]
    fn multi_example_first() {
        let inputs = [
            ("", "-"),
            ("", "-"),
            ("", "-"),
            ("", "-"),
        ];
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
        let inputs = [
            ("", "-"),
            ("", "-"),
            ("", "-"),
            ("", "-"),
        ];
        for (input, result) in inputs {
            assert_eq!(solve_2(input).unwrap(), result);
        }
    }
}
