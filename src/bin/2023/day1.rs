#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight",
    "nine",
];

pub fn parse(fh: File) -> Result<impl Iterator<Item = String>> {
    Ok(parse::lines(fh))
}

pub fn part1(lines: impl Iterator<Item = String>) -> Result<i64> {
    let mut total = 0;
    for line in lines {
        let first = line.chars().find(char::is_ascii_digit).unwrap();
        let last = line.chars().rev().find(char::is_ascii_digit).unwrap();
        let val: i64 = format!("{}{}", first, last).parse().unwrap();
        total += val;
    }
    Ok(total)
}

pub fn part2(lines: impl Iterator<Item = String>) -> Result<i64> {
    let mut total = 0;
    for line in lines {
        let mut first = 0;
        let mut last = 0;
        'c: for idx in 0..line.len() {
            let c = line.chars().nth(idx).unwrap();
            if char::is_numeric(c) {
                first = i64::from(u32::from(c) - u32::from('0'));
                break 'c;
            }
            for (i, n) in NUMBERS.iter().enumerate() {
                if line[idx..].starts_with(n) {
                    first = i64::try_from(i).unwrap();
                    break 'c;
                }
            }
        }
        'c: for idx in (0..line.len()).rev() {
            let c = line.chars().nth(idx).unwrap();
            if char::is_numeric(c) {
                last = i64::from(u32::from(c) - u32::from('0'));
                break 'c;
            }
            for (i, n) in NUMBERS.iter().enumerate() {
                if line[..=idx].ends_with(n) {
                    last = i64::try_from(i).unwrap();
                    break 'c;
                }
            }
        }
        let val: i64 = format!("{}{}", first, last).parse().unwrap();
        total += val;
    }
    Ok(total)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 1).unwrap()).unwrap()).unwrap(),
        54927
    );
    assert_eq!(
        part2(parse(parse::data(2023, 1).unwrap()).unwrap()).unwrap(),
        54581
    );
}
