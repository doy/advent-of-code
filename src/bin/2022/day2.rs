#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(self) -> u64 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

impl std::str::FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => return Err(()),
        })
    }
}

#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn outcome(self, other: Self) -> Outcome {
        match self {
            Self::Rock => match other {
                Self::Rock => Outcome::Draw,
                Self::Paper => Outcome::Lose,
                Self::Scissors => Outcome::Win,
            },
            Self::Paper => match other {
                Self::Rock => Outcome::Win,
                Self::Paper => Outcome::Draw,
                Self::Scissors => Outcome::Lose,
            },
            Self::Scissors => match other {
                Self::Rock => Outcome::Lose,
                Self::Paper => Outcome::Win,
                Self::Scissors => Outcome::Draw,
            },
        }
    }

    fn shape_for(self, outcome: Outcome) -> Self {
        if Self::Rock.outcome(self) == outcome {
            return Self::Rock;
        }
        if Self::Paper.outcome(self) == outcome {
            return Self::Paper;
        }
        if Self::Scissors.outcome(self) == outcome {
            return Self::Scissors;
        }
        unreachable!()
    }
}

impl std::str::FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => return Err(()),
        })
    }
}

pub fn parse(fh: File) -> Result<impl Iterator<Item = String>> {
    Ok(parse::raw_lines(fh))
}

pub fn part1(lines: impl Iterator<Item = String>) -> Result<u64> {
    Ok(lines
        .map(|line| {
            let mut parts = line.split(' ');
            let them: Shape = parts.next().unwrap().parse().unwrap();
            let me: Shape = parts.next().unwrap().parse().unwrap();
            me.score() + me.outcome(them).score()
        })
        .sum())
}

pub fn part2(lines: impl Iterator<Item = String>) -> Result<u64> {
    Ok(lines
        .map(|line| {
            let mut parts = line.split(' ');
            let them: Shape = parts.next().unwrap().parse().unwrap();
            let outcome: Outcome = parts.next().unwrap().parse().unwrap();
            let me = them.shape_for(outcome);
            me.score() + outcome.score()
        })
        .sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 2).unwrap()).unwrap()).unwrap(),
        13565
    );
    assert_eq!(
        part2(parse(parse::data(2022, 2).unwrap()).unwrap()).unwrap(),
        12424
    );
}
