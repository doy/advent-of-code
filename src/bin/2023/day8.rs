#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

pub struct Network {
    directions: Vec<Direction>,
    graph: HashMap<String, (String, String)>,
}

pub fn parse(fh: File) -> Result<Network> {
    let mut lines = parse::raw_lines(fh);
    let directions = lines.next().unwrap();
    lines.next().unwrap();
    Ok(Network {
        directions: directions
            .chars()
            .map(|c| {
                if c == 'L' {
                    Direction::Left
                } else {
                    Direction::Right
                }
            })
            .collect(),
        graph: lines
            .map(|line| {
                let cap = regex_captures!(r"(\w+) = \((\w+), (\w+)\)", &line)
                    .unwrap();
                (cap[1].to_string(), (cap[2].to_string(), cap[3].to_string()))
            })
            .collect(),
    })
}

pub fn part1(network: Network) -> Result<i64> {
    let mut vertex = "AAA".to_string();
    let mut distance = 0;

    while vertex != "ZZZ" {
        let next = network.graph[&vertex].clone();
        vertex = match network.directions[distance % network.directions.len()]
        {
            Direction::Left => next.0,
            Direction::Right => next.1,
        };
        distance += 1;
    }

    Ok(distance.try_into().unwrap())
}

pub fn part2(network: Network) -> Result<i64> {
    todo!()
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 8).unwrap()).unwrap()).unwrap(),
        11309
    );
    assert_eq!(
        part2(parse(parse::data(2023, 8).unwrap()).unwrap()).unwrap(),
        0
    );
}
