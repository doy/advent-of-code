#![allow(dead_code)]
#![allow(unused_variables)]

use std::str::FromStr;

use advent_of_code::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Red(i64),
    Green(i64),
    Blue(i64),
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        match parts[1] {
            "red" => Ok(Self::Red(parts[0].parse()?)),
            "green" => Ok(Self::Green(parts[0].parse()?)),
            "blue" => Ok(Self::Blue(parts[0].parse()?)),
            _ => Err(anyhow::anyhow!("failed to parse {}", parts[1])),
        }
    }
}

pub fn parse(fh: File) -> Result<Vec<Vec<Vec<Color>>>> {
    parse::raw_lines(fh)
        .map(|line| {
            let line = line.split(": ").nth(1).unwrap();
            line.split("; ")
                .map(|draw| {
                    draw.split(", ")
                        .map(|color| color.parse::<Color>())
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<Vec<_>>, _>>()
        })
        .collect()
}

pub fn part1(games: Vec<Vec<Vec<Color>>>) -> Result<i64> {
    let mut total = 0;
    'game: for (i, game) in games.into_iter().enumerate() {
        for draw in game {
            for color in draw {
                match color {
                    Color::Red(n) => {
                        if n > 12 {
                            continue 'game;
                        }
                    }
                    Color::Green(n) => {
                        if n > 13 {
                            continue 'game;
                        }
                    }
                    Color::Blue(n) => {
                        if n > 14 {
                            continue 'game;
                        }
                    }
                }
            }
        }
        total += i64::try_from(i).unwrap() + 1;
    }
    Ok(total)
}

pub fn part2(games: Vec<Vec<Vec<Color>>>) -> Result<i64> {
    let mut total = 0;
    for game in games {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for draw in game {
            for color in draw {
                match color {
                    Color::Red(n) => {
                        if n > min_red {
                            min_red = n;
                        }
                    }
                    Color::Green(n) => {
                        if n > min_green {
                            min_green = n;
                        }
                    }
                    Color::Blue(n) => {
                        if n > min_blue {
                            min_blue = n;
                        }
                    }
                }
            }
        }
        total += min_red * min_green * min_blue;
    }
    Ok(total)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 2).unwrap()).unwrap()).unwrap(),
        2617
    );
    assert_eq!(
        part2(parse(parse::data(2023, 2).unwrap()).unwrap()).unwrap(),
        59795
    );
}
