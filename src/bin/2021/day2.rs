use advent_of_code::prelude::*;

pub enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

pub fn parse(fh: File) -> Result<impl Iterator<Item = Command>> {
    Ok(parse::raw_lines(fh).map(|line| {
        if let Some(n) = line.strip_prefix("forward ") {
            Command::Forward(n.parse().unwrap())
        } else if let Some(n) = line.strip_prefix("down ") {
            Command::Down(n.parse().unwrap())
        } else if let Some(n) = line.strip_prefix("up ") {
            Command::Up(n.parse().unwrap())
        } else {
            panic!("couldn't parse line: {}", line);
        }
    }))
}

pub fn part1(commands: impl Iterator<Item = Command>) -> Result<i64> {
    let mut horizontal = 0;
    let mut vertical = 0;
    for command in commands {
        match command {
            Command::Forward(n) => {
                horizontal += n;
            }
            Command::Down(n) => {
                vertical += n;
            }
            Command::Up(n) => {
                vertical -= n;
            }
        }
    }
    Ok(horizontal * vertical)
}

pub fn part2(commands: impl Iterator<Item = Command>) -> Result<i64> {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut vertical = 0;
    for command in commands {
        match command {
            Command::Forward(n) => {
                horizontal += n;
                vertical += aim * n;
            }
            Command::Down(n) => {
                aim += n;
            }
            Command::Up(n) => {
                aim -= n;
            }
        }
    }
    Ok(horizontal * vertical)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2021, 2).unwrap()).unwrap()).unwrap(),
        1694130
    );
    assert_eq!(
        part2(parse(parse::data(2021, 2).unwrap()).unwrap()).unwrap(),
        1698850445
    );
}
