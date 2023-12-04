use std::path::Path;
use anyhow::Result;

const DAY: u32 = 8;

fn solve(input: &str) -> Result<String> {
    let mut trees:Vec<Vec<(u8, usize, usize, usize, usize)>> = input.lines()
        .map(|l| {
            l.chars().map(|c| (c as u8 - '0' as u8, 0, 0, 0, 0)).collect()
        })
        .collect();
    for y in 0..trees.len() {
        for x in 0..trees.len() {
            // right
            let higher = (0..x).rev().find(|&fx| trees[y][fx].0 >= trees[y][x].0);
            if let Some(higher) = higher {
                trees[y][x].1 += x - higher;
            } else {
                trees[y][x].1 += x;
            }
        }
        for x in (0..trees.len()).rev() {
            // left
            let higher = (x+1..trees.len()).find(|&fx| trees[y][fx].0 >= trees[y][x].0);
            if let Some(higher) = higher {
                trees[y][x].2 += higher - x;
            } else {
                trees[y][x].2 += trees.len() - 1 - x;
            }
        }
    }
    for x in 0..trees.len() {
        for y in 0..trees.len() {
            // up
            let higher = (0..y).rev().find(|&fy| trees[fy][x].0 >= trees[y][x].0);
            if let Some(higher) = higher {
                trees[y][x].3 += y - higher;
            } else {
                trees[y][x].3 += y;
            }
        }
        for y in (0..trees.len()).rev() {
            // down
            let higher = (y+1..trees.len()).find(|&fy| trees[fy][x].0 >= trees[y][x].0);
            if let Some(higher) = higher {
                trees[y][x].4 += higher - y;
            } else {
                trees[y][x].4 += trees.len() - 1 - y;
            }
        }
    }
    let count = trees.iter().flat_map(|row| row.iter().map(|t| t.1 * t.2 * t.3 * t.4).max()).max();
    Ok(count.unwrap().to_string())
}

fn solve_1(input: &str) -> Result<String> {
    let mut trees:Vec<Vec<(u8, bool)>> = input.lines()
        .map(|l| {
            l.chars().map(|c| (c as u8, false)).collect()
        })
        .collect();
    for y in 0..trees.len() {
        let mut highest = 0;
        for x in 0..trees.len() {
            if x == 0 {
                highest = trees[y][x].0;
                trees[y][x].1 = true;
                continue;
            }
            if trees[y][x].0 > highest {
                highest = trees[y][x].0;
                trees[y][x].1 = true;
            }
        }
        for x in (0..trees.len()).rev() {
            if x == trees.len() - 1 {
                highest = trees[y][x].0;
                trees[y][x].1 = true;
                continue;
            }
            if trees[y][x].0 > highest {
                highest = trees[y][x].0;
                trees[y][x].1 = true;
            }
        }
    }
    for x in 0..trees.len() {
        let mut highest = 0;
        for y in 0..trees.len() {
            if y == 0 {
                highest = trees[y][x].0;
                trees[y][x].1 = true;
                continue;
            }
            if trees[y][x].0 > highest {
                highest = trees[y][x].0;
                trees[y][x].1 = true;
            }
        }
        for y in (0..trees.len()).rev() {
            if y == trees.len() - 1 {
                highest = trees[y][x].0;
                trees[y][x].1 = true;
                continue;
            }
            if trees[y][x].0 > highest {
                highest = trees[y][x].0;
                trees[y][x].1 = true;
            }
        }
    }
    let count = trees.iter().flat_map(|row| row.iter().filter(|t| t.1).map(|t| t.1).collect::<Vec<bool>>()).count();
    Ok(count.to_string())
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

        let result = "21";
        let solve = solve_1(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }

    #[test]
    fn example_second() {
        let input = example_input();

        let result = "8";
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
