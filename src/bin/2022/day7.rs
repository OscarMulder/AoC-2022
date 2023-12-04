use std::collections::HashMap;
use std::{path::Path, collections::BTreeMap};
use anyhow::Result;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::*;

const DAY: u32 = 7;

enum Cmd {
    CdOut,
    CdRoot,
    Cd(String),
    Ls,
    File(String, u32),
    Dir(String),
}

fn parse_ls(input: &str) -> IResult<&str, Cmd> {
    let (input, _) = tag("$ ls")(input)?;
    Ok((input, Cmd::Ls))
}

fn parse_line(input: &str) -> IResult<&str, Cmd> {
    let (input, cmd) = branch::alt((parse_ls_file, parse_ls_dir, parse_cd, parse_ls))(input)?;
    Ok((input, cmd))
}

fn parse_ls_file(input: &str) -> IResult<&str, Cmd> {
    let (input, (size, _, name)) = sequence::tuple((character::complete::u32, character::complete::space1, multi::many1(character::complete::anychar)))(input)?;
    Ok((input, Cmd::File(name.iter().collect(), size)))
}

fn parse_ls_dir(input: &str) -> IResult<&str, Cmd> {
    let (input, (_, _, name)) = sequence::tuple((tag("dir"), character::complete::space1, multi::many1(character::complete::anychar)))(input)?;
    Ok((input, Cmd::Dir(name.iter().collect())))
}

fn get_path(stack: Vec<String>) -> String {
    let str: String = stack.into_iter().collect();
    str
}

fn parse_cd(input: &str) -> IResult<&str, Cmd> {
    let dir_out = tag("$ cd ..");
    let dir_root = tag("$ cd /");
    let (input, dir) = combinator::opt(dir_out)(input)?;
    if let Some(_) = dir {
        return Ok((input, Cmd::CdOut))
    }
    let (input, dir) = combinator::opt(dir_root)(input)?;
    if let Some(_) = dir {
        return Ok((input, Cmd::CdRoot))
    }
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = multi::many1(character::complete::anychar)(input)?;
    Ok((input, Cmd::Cd(dir.iter().collect())))
}

fn parse(input: &str) -> HashMap<String, u32> {
    let mut filesystem: BTreeMap<String, u32> = BTreeMap::new();
    let mut dirs: Vec<String> = vec![];
    let mut stack: Vec<String> = vec![];
    stack.push("/".to_string());
    dirs.push("/".to_string());
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let (_, cmd) = parse_line(line).unwrap();
        match cmd {
            Cmd::CdOut => { stack.pop(); },
            Cmd::CdRoot => { stack.drain(1..); },
            Cmd::Cd(name) => { stack.push(format!("{}/", name)); },
            Cmd::Ls => {},
            Cmd::File(name, size) => {
                let pwd = get_path(stack.clone());
                filesystem.insert(format!("{}{}", pwd, name), size);
            },
            Cmd::Dir(name) => {
                let pwd = get_path(stack.clone());
                dirs.push(format!("{}{}/", pwd, name));
            },
        };
    };
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    dirs.iter().for_each(|dir| {
        for (_,v) in filesystem.range(dir.to_string()..).take_while(|(k, _)| k.starts_with(dir)) {
            *dir_sizes.entry(dir.to_string()).or_insert(0) += v;
        }
    });
    dir_sizes
}

fn solve(input: &str) -> Result<String> {
    let mut dir_sizes = parse(input);
    let sorted: BTreeMap<u32, String> = dir_sizes.iter().map(|(k,&v)| (v,k.clone())).collect();
    let unused_space = 70_000_000 - *dir_sizes.entry("/".to_string()).or_default();
    let to_find = 30_000_000 - unused_space;
    let (res, _) = sorted.iter().find(|(k, _)| **k >= to_find).unwrap();
    Ok(res.to_string())
}

fn solve_1(input: &str) -> Result<String> {
    let dir_sizes = parse(input);
    let res: u64 = dir_sizes.iter().filter(|(_, &size)| size <= 100_000).map(|(_, size)| *size as u64).sum();
    Ok(res.to_string())
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

        let result = "95437";
        let solve = solve_1(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }

    #[test]
    fn example_second() {
        let input = example_input();

        let result = "24933642";
        let solve = solve_2(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }
}
