#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

fn calculate_next(v: Vec<i64>) -> i64 {
    if v.iter().all(|n| *n == 0) {
        return 0;
    }
    calculate_next((1..v.len()).map(|i| v[i] - v[i - 1]).collect())
        + v[v.len() - 1]
}

fn calculate_prev(v: Vec<i64>) -> i64 {
    if v.iter().all(|n| *n == 0) {
        return 0;
    }
    v[0] - calculate_prev((1..v.len()).map(|i| v[i] - v[i - 1]).collect())
}

pub fn parse(fh: File) -> Result<Vec<Vec<i64>>> {
    Ok(parse::raw_lines(fh)
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect())
}

pub fn part1(report: Vec<Vec<i64>>) -> Result<i64> {
    Ok(report.into_iter().map(calculate_next).sum())
}

pub fn part2(report: Vec<Vec<i64>>) -> Result<i64> {
    Ok(report.into_iter().map(calculate_prev).sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 9).unwrap()).unwrap()).unwrap(),
        0
    );
    assert_eq!(
        part2(parse(parse::data(2023, 9).unwrap()).unwrap()).unwrap(),
        0
    );
}
