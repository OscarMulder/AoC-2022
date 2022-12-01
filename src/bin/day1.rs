use std::path::Path;
use anyhow::Result;

const DAY: u32 = 1;

fn solve(input: &Path, take: usize) -> Result<u32> {
    let mut res: Vec<u32> = std::fs::read_to_string(input)?
        .split("\n\n")
        .map(|s| s.lines().map(str::parse::<u32>).sum())
        .collect::<Result<Vec<u32>, std::num::ParseIntError>>()?;
    res.sort_by(|a,b| b.cmp(a));
    Ok(res.iter().take(take).sum::<u32>())
}

fn main() {
    let path = format!("./data/{}.input", DAY);
    let input = Path::new(&path);

    let solve_first = solve(input, 1);
    match solve_first {
        Ok(res) => println!("Day {}, first puzzle: {}", DAY, res),
        Err(e) => println!("{:?}", e),
    }

    let solve_second = solve(input, 3);
    match solve_second {
        Ok(res) => println!("Day {}, second puzzle: {}", DAY, res),
        Err(e) => println!("{:?}", e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_first() {
        let path = format!("./data/{}.example", DAY);
        let input = Path::new(&path);

        let result = 24000;
        let solve = solve(input, 1);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }

    #[test]
    fn example_second() {
        let path = format!("./data/{}.example", DAY);
        let input = Path::new(&path);

        let result = 45000;
        let solve = solve(input, 3);
        
        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }
}
