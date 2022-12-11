use std::path::Path;
use anyhow::Result;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::*;

const DAY: u32 = 7;

struct Dir {
    name: String,
    files: Vec<Box<File>>,
    children: Vec<Box<Dir>>,
}

impl Dir  {
    fn new(name: String) -> Self {
        Dir {
            name,
            files: vec![],
            children: vec![],
        }
    }
}

struct File {
    name: String,
    size: u32,
}

enum Cmd {
    CdOut,
    CdRoot,
    Cd(String),
    Ls,
}

fn parse_ls(input: &str) -> IResult<&str, Cmd> {
    let (input, _) = tag("ls")(input)?;
    Ok((input, Cmd::Ls))
}

fn parse_cmd(input: &str) -> IResult<&str, Cmd> {
    let (input, (_, cmd)) = sequence::tuple((tag("$ "), branch::alt((parse_cd, parse_ls))))(input)?;
    Ok((input, cmd))
}

fn parse_cd(input: &str) -> IResult<&str, Cmd> {
    let dir_out = tag("..");
    let dir_root = tag("/");
    let (input, dir) = tag("cd ")(input)?;
    let (input, dir) = combinator::opt(dir_out)(input)?;
    if let Some(dir) = dir {
        return Ok((input, Cmd::CdOut))
    }
    let (input, dir) = combinator::opt(dir_root)(input)?;
    if let Some(dir) = dir {
        return Ok((input, Cmd::CdRoot))
    }
    let (input, dir) = multi::many1(character::complete::anychar)(input)?;
    Ok((input, Cmd::Cd(dir.iter().collect())))
}

fn parse_ls_file(input: &str) -> IResult<&str, File> {
    let (input, (size, _, name)) = sequence::tuple((character::complete::u32, character::complete::space1, multi::many1(character::complete::anychar)))(input)?;
    Ok((input, File { size, name: name.iter().collect()}))
}

fn parse_ls_dir(input: &str) -> IResult<&str, Dir> {
    let (input, (_, _, name)) = sequence::tuple((tag("dir"), character::complete::space1, multi::many1(character::complete::anychar)))(input)?;
    Ok((input, Dir::new(name.iter().collect())))
}

fn parse_ls_output_line<'a>(input: &'a str) -> IResult<&'a str, Dir> {
    let mut current_dir = Dir::new("curr".to_string());
    let (input, file) = combinator::opt(parse_ls_file)(input)?;
    if let Some(file) = file {
        current_dir.files.push(Box::new(file));
    }
    let (input, dir) = combinator::opt(parse_ls_dir)(input)?;
    if let Some(dir) = dir {
        current_dir.children.push(Box::new(dir));
    }
    Ok((input, current_dir))
}

fn parse_ls_output<'a,'b>(input: &'a str) -> IResult<&'a str, Dir> {
    let (input, (dirs, _)) = multi::many_till(parse_ls_output_line, parse_cmd)(input)?;
    let mut current_dir = Dir::new("curr".to_string());
    combine_dirs(&mut current_dir, dirs);
    return Ok((input, current_dir));
}

fn combine_dirs(current_dir: &mut Dir, dirs: Vec<Dir>) -> () {
    for mut dir in dirs {
        current_dir.files.append(&mut dir.files);
        current_dir.children.append(&mut dir.children);
    };
}

fn parse(input: &str) -> IResult<&str, Dir> {
    let root_dir = Dir { name: "/".to_string(), files: vec![], children: vec![]};
    let mut stack: Vec<Dir> = vec![];
    stack.push(root_dir);
    let (input, cmd) = parse_cmd(input)?;
    let input = match cmd {
        Cmd::CdOut => { stack.pop(); input },
        Cmd::CdRoot => { 
            stack.drain(1..);
            input
        },
        Cmd::Cd(name) => {
            let current = stack.pop().unwrap();
            // let new = current.children.iter().find(|c| c.name == name).unwrap();
            // let new = stack.iter().reduce(|current, dir| {
            //     current.children.iter().find(|c| c.name == dir.name).unwrap()
            // });
            stack.push(current);
            // stack.push(new);
            input
        },
        Cmd::Ls => {
            let mut current = stack.pop().unwrap();
            let (input, dir) = parse_ls_output(input)?;
            combine_dirs(&mut current, vec![dir]);
            stack.push(current);
            input
        },
    };
    stack.drain(1..);
    let root_dir = stack.pop().unwrap();
    Ok((input, root_dir))
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

        let result = "95437";
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

}
