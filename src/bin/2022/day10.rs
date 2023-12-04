use anyhow::Result;
use std::path::Path;

const DAY: u32 = 10;

// fn solve(input: &str) -> Result<String> {}

fn solve_1(input: &str) -> Result<String> {
    let instructions = input.lines();
    let mut cycles = vec![1];
    for line in instructions {
        let mut parts = line.split_whitespace();
        let instr = parts.next();
        let v = parts.next();
        match instr.unwrap() {
            "addx" => {
                let v: i32 = v.unwrap().parse().unwrap();
                let current_x = cycles.last().unwrap().clone();
                cycles.push(current_x);
                cycles.push(current_x + v);
            }
            "noop" => cycles.push(cycles.last().unwrap().clone()),
            _ => panic!("I Die."),
        }
    }
    let result: i32 = [
        20 * cycles[19],
        60 * cycles[59],
        100 * cycles[99],
        140 * cycles[139],
        180 * cycles[179],
        220 * cycles[219],
    ]
    .iter()
    .sum();
    Ok(result.to_string())
}

fn solve_2(input: &str) -> Result<String> {
    let instructions = input.lines();
    let mut cycles = vec![1];
    for line in instructions {
        let mut parts = line.split_whitespace();
        let instr = parts.next();
        let v = parts.next();
        match instr.unwrap() {
            "addx" => {
                let v: i32 = v.unwrap().parse().unwrap();
                let current_x = cycles.last().unwrap().clone();
                cycles.push(current_x);
                cycles.push(current_x + v);
            }
            "noop" => cycles.push(cycles.last().unwrap().clone()),
            _ => panic!("I Die."),
        }
    }
    let crt = cycles
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let mut pixel: String;
            let pos: i32 = i as i32 % 40;
            let pix = x % 40;
            if pos >= pix - 1 && pos as i32 <= pix + 1 {
                pixel = "#".into();
            } else {
                pixel = ".".into();
            }
            if i != 0 && (i + 1) % 40 == 0 {
                pixel += "\n".into();
            }
            pixel
        })
        .collect::<String>();
    print!("{}", crt);
    return Ok("".to_string());
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

        let result = "13140";
        let solve = solve_1(&input);

        assert!(solve.is_ok());
        assert_eq!(solve.unwrap(), result);
    }

    #[test]
    fn multi_example_first() {
        let inputs = [("", "-"), ("", "-"), ("", "-"), ("", "-")];
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
        let inputs = [("", "-"), ("", "-"), ("", "-"), ("", "-")];
        for (input, result) in inputs {
            assert_eq!(solve_2(input).unwrap(), result);
        }
    }
}
