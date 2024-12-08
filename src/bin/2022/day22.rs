#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Default)]
enum Tile {
    Open,
    Wall,
    #[default]
    Noop,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Step {
    Forward(usize),
    Left,
    Right,
}

fn direction_value(direction: Direction) -> usize {
    match direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    }
}

#[derive(Debug)]
pub struct Map {
    grid: Grid<Tile>,
    path: Vec<Step>,
}

impl Map {
    fn step(&self, pos: (Row, Col), facing: Direction) -> (Row, Col) {
        let (row_diff, col_diff, sub) = match facing {
            Direction::Right => (0, 1, false),
            Direction::Down => (1, 0, false),
            Direction::Left => (0, 1, true),
            Direction::Up => (1, 0, true),
        };
        let rows = self.grid.rows().0;
        let cols = self.grid.cols().0;
        let mut new_pos = pos;
        new_pos = (
            if sub {
                rows + new_pos.0 - row_diff
            } else {
                rows + new_pos.0 + row_diff
            } % rows,
            if sub {
                cols + new_pos.1 - col_diff
            } else {
                cols + new_pos.1 + col_diff
            } % cols,
        );
        while matches!(self.grid[new_pos.0][new_pos.1], Tile::Noop) {
            new_pos = (
                if sub {
                    rows + new_pos.0 - row_diff
                } else {
                    rows + new_pos.0 + row_diff
                } % rows,
                if sub {
                    cols + new_pos.1 - col_diff
                } else {
                    cols + new_pos.1 + col_diff
                } % cols,
            );
        }
        if matches!(self.grid[new_pos.0][new_pos.1], Tile::Wall) {
            pos
        } else {
            new_pos
        }
    }
}

pub fn parse(fh: File) -> Result<Map> {
    let mut lines = parse::raw_lines(fh);
    let grid = parse::grid(parse::chunk(&mut lines), |c, _| match c {
        b' ' => Tile::Noop,
        b'.' => Tile::Open,
        b'#' => Tile::Wall,
        _ => panic!("invalid map tile {}", c),
    });

    let path_str = lines.next().unwrap();
    let mut path_str = &path_str[..];
    let mut path = vec![];
    while let Some(first) = path_str.chars().next() {
        match first {
            'R' => {
                path.push(Step::Right);
                path_str = &path_str[1..];
            }
            'L' => {
                path.push(Step::Left);
                path_str = &path_str[1..];
            }
            '0'..='9' => {
                let prefix_len =
                    path_str.chars().take_while(char::is_ascii_digit).count();
                path.push(Step::Forward(
                    path_str[0..prefix_len].parse().unwrap(),
                ));
                path_str = &path_str[prefix_len..];
            }
            _ => panic!("invalid path char {}", first),
        }
    }

    Ok(Map { grid, path })
}

pub fn part1(map: Map) -> Result<usize> {
    let mut pos = (Row(0), Col(0));
    let mut facing = Direction::Right;
    for step in &map.path {
        match step {
            Step::Left => facing = facing.turn_left(),
            Step::Right => facing = facing.turn_right(),
            Step::Forward(n) => {
                for _ in 0..*n {
                    pos = map.step(pos, facing);
                }
            }
        }
    }
    Ok((pos.0 .0 + 1) * 1000 + (pos.1 .0 + 1) * 4 + direction_value(facing))
}

pub fn part2(map: Map) -> Result<usize> {
    todo!()
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 22).unwrap()).unwrap()).unwrap(),
        95358
    );
    // assert_eq!(
    //     part2(parse(parse::data(2022, 22).unwrap()).unwrap()).unwrap(),
    //     0
    // );
}
