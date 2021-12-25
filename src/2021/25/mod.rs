#![allow(dead_code)]
#![allow(unused_variables)]

use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Cell {
    Down,
    Right,
    None,
}

impl Default for Cell {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map {
    grid: Grid<Cell>,
}

impl Map {
    fn step(&self) -> Self {
        self.step_east().step_south()
    }

    fn step_east(&self) -> Self {
        let mut step = self.clone();
        for ((Row(row), Col(col)), cell) in self.grid.indexed_cells() {
            if *cell == Cell::Right {
                let mut next = col + 1;
                if next >= self.grid.cols().0 {
                    next = 0;
                }
                if self.grid[Row(row)][Col(next)] == Cell::None {
                    step.grid[Row(row)][Col(next)] = Cell::Right;
                    step.grid[Row(row)][Col(col)] = Cell::None;
                }
            }
        }
        step
    }

    fn step_south(&self) -> Self {
        let mut step = self.clone();
        for ((Row(row), Col(col)), cell) in self.grid.indexed_cells() {
            if *cell == Cell::Down {
                let mut next = row + 1;
                if next >= self.grid.rows().0 {
                    next = 0;
                }
                if self.grid[Row(next)][Col(col)] == Cell::None {
                    step.grid[Row(next)][Col(col)] = Cell::Down;
                    step.grid[Row(row)][Col(col)] = Cell::None;
                }
            }
        }
        step
    }
}

pub fn parse(fh: File) -> Result<Map> {
    Ok(Map {
        grid: parse::grid(parse::lines(fh), |b| match b {
            b'v' => Cell::Down,
            b'>' => Cell::Right,
            b'.' => Cell::None,
            _ => panic!("unknown cell {}", b),
        }),
    })
}

pub fn part1(map: Map) -> Result<i64> {
    let mut prev = map;
    let mut i = 0;
    loop {
        i += 1;
        let next = prev.step();
        if next == prev {
            break;
        }
        prev = next;
    }
    Ok(i)
}

pub fn part2(_: Map) -> Result<i64> {
    todo!()
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2021, 25).unwrap()).unwrap()).unwrap(),
        482
    );
}
