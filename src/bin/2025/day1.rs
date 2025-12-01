#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

pub struct Rotation(i64);

impl std::str::FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (direction, count) = s.split_at(1);
        let direction: Direction = direction
            .parse()
            .map_err(|_| anyhow::anyhow!("parse direction"))?;
        let mut count = count.parse().context("parse count")?;
        if !direction.increasing() {
            count *= -1;
        }
        Ok(Self(count))
    }
}

pub fn parse(fh: File) -> Result<impl Iterator<Item = Rotation>> {
    Ok(parse::lines(fh))
}

pub fn part1(rotations: impl Iterator<Item = Rotation>) -> Result<i64> {
    let mut pos = 50;
    let mut zeroes = 0;
    for rotation in rotations {
        pos += rotation.0;
        pos = pos.rem_euclid(100);
        if pos == 0 {
            zeroes += 1;
        }
    }
    Ok(zeroes)
}

pub fn part2(rotations: impl Iterator<Item = Rotation>) -> Result<i64> {
    let mut pos = 50;
    let mut zeroes = 0;
    for rotation in rotations {
        pos += rotation.0;
        zeroes += (pos / 100).abs()
            + if pos <= 0 && pos - rotation.0 != 0 {
                1
            } else {
                0
            };
        pos = pos.rem_euclid(100);
    }
    Ok(zeroes)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2025, 1).unwrap()).unwrap()).unwrap(),
        1182
    );
    assert_eq!(
        part2(parse(parse::data(2025, 1).unwrap()).unwrap()).unwrap(),
        6907
    );
}
