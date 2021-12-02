use anyhow::Context as _;
use std::io::BufRead as _;

pub fn part1() -> anyhow::Result<i64> {
    let f = data!()?;
    let f = std::io::BufReader::new(f);
    let mut horizontal = 0;
    let mut vertical = 0;
    for line in f.lines() {
        let line = line?;
        if let Some(n) = line.strip_prefix("forward ") {
            horizontal += n.parse::<i64>().unwrap();
        } else if let Some(n) = line.strip_prefix("down ") {
            vertical += n.parse::<i64>().unwrap();
        } else if let Some(n) = line.strip_prefix("up ") {
            vertical -= n.parse::<i64>().unwrap();
        }
    }
    Ok(horizontal * vertical)
}

pub fn part2() -> anyhow::Result<i64> {
    let f = data!()?;
    let f = std::io::BufReader::new(f);
    let mut aim = 0;
    let mut horizontal = 0;
    let mut vertical = 0;
    for line in f.lines() {
        let line = line?;
        if let Some(n) = line.strip_prefix("forward ") {
            let x = n.parse::<i64>().unwrap();
            horizontal += x;
            vertical += aim * x;
        } else if let Some(n) = line.strip_prefix("down ") {
            aim += n.parse::<i64>().unwrap();
        } else if let Some(n) = line.strip_prefix("up ") {
            aim -= n.parse::<i64>().unwrap();
        }
    }
    Ok(horizontal * vertical)
}
