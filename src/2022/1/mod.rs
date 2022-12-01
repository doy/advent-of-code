#![allow(dead_code)]
#![allow(unused_variables)]

use crate::prelude::*;

pub fn parse(fh: File) -> Result<Vec<i64>> {
    let mut elves = vec![];
    let mut cur = 0;
    for line in parse::lines(fh) {
        let line = line.trim();
        if line.is_empty() {
            elves.push(cur);
            cur = 0;
        } else {
            cur += line.parse::<i64>()?;
        }
    }
    elves.push(cur);
    Ok(elves)
}

pub fn part1(elves: Vec<i64>) -> Result<i64> {
    Ok(elves.iter().copied().max().unwrap_or(0))
}

pub fn part2(mut elves: Vec<i64>) -> Result<i64> {
    elves.sort();
    Ok(elves.iter().rev().copied().take(3).sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 1).unwrap()).unwrap()).unwrap(),
        67658
    );
    assert_eq!(
        part2(parse(parse::data(2022, 1).unwrap()).unwrap()).unwrap(),
        200158
    );
}
