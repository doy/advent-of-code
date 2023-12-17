#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

fn add_offset(
    row: Row,
    col: Col,
    row_offset: IRow,
    col_offset: ICol,
    max_row: Row,
    max_col: Col,
) -> Option<(Row, Col)> {
    if let (Some(row), Some(col)) = (
        row.0.checked_add_signed(row_offset.0),
        col.0.checked_add_signed(col_offset.0),
    ) {
        let row = Row(row);
        let col = Col(col);
        if row < max_row && col < max_col {
            return Some((row, col));
        }
    }
    None
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_pos(row: Row, col: Col, new_row: Row, new_col: Col) -> Self {
        if row.abs_diff(new_row) == Row(1) && col.abs_diff(new_col) == Col(0)
        {
            if row > new_row {
                Self::Up
            } else {
                Self::Down
            }
        } else if col.abs_diff(new_col) == Col(1)
            && row.abs_diff(new_row) == Row(0)
        {
            if col > new_col {
                Self::Left
            } else {
                Self::Right
            }
        } else {
            panic!("invalid direction ({row:?}, {col:?}) -> ({new_row:?}, {new_col:?})")
        }
    }

    fn horizontal(&self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }

    fn increasing(&self) -> bool {
        matches!(self, Self::Down | Self::Right)
    }

    fn turns(&self) -> [Self; 2] {
        match self {
            Self::Up | Self::Down => [Self::Left, Self::Right],
            Self::Left | Self::Right => [Self::Up, Self::Down],
        }
    }

    fn offset(&self) -> (IRow, ICol) {
        match self {
            Self::Up => (IRow(-1), ICol(0)),
            Self::Down => (IRow(1), ICol(0)),
            Self::Left => (IRow(0), ICol(-1)),
            Self::Right => (IRow(0), ICol(1)),
        }
    }
}

#[derive(Debug)]
pub struct Crucible {
    map: Grid<u8>,
}

impl
    advent_of_code::graph::Graph<
        (Row, Col, Option<Direction>, u8),
        (Row, Col),
    > for Crucible
{
    type Edges = Vec<(Row, Col)>;

    fn edges(&self, v: (Row, Col, Option<Direction>, u8)) -> Self::Edges {
        let (row, col, direction, length) = v;

        if let Some(direction) = direction {
            let mut edges: Vec<_> = direction
                .turns()
                .into_iter()
                .filter_map(|direction| {
                    let offset = direction.offset();
                    add_offset(
                        row,
                        col,
                        offset.0,
                        offset.1,
                        self.map.rows(),
                        self.map.cols(),
                    )
                })
                .collect();
            if length + 1 < 3 {
                let offset = direction.offset();
                if let Some((row, col)) = add_offset(
                    row,
                    col,
                    offset.0,
                    offset.1,
                    self.map.rows(),
                    self.map.cols(),
                ) {
                    edges.push((row, col));
                }
            }
            edges
        } else {
            [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
            .into_iter()
            .filter_map(|direction| {
                let offset = direction.offset();
                add_offset(
                    row,
                    col,
                    offset.0,
                    offset.1,
                    self.map.rows(),
                    self.map.cols(),
                )
            })
            .collect()
        }
    }

    fn edge(
        &self,
        v: (Row, Col, Option<Direction>, u8),
        e: (Row, Col),
    ) -> ((Row, Col, Option<Direction>, u8), u64) {
        let (row, col, direction, length) = v;
        let (new_row, new_col) = e;
        let new_direction = Direction::from_pos(row, col, new_row, new_col);

        (
            (
                new_row,
                new_col,
                Some(new_direction),
                if direction == Some(new_direction) {
                    length + 1
                } else {
                    0
                },
            ),
            u64::from(self.map[new_row][new_col]),
        )
    }
}

#[derive(Debug)]
pub struct UltraCrucible {
    map: Grid<u8>,
}

impl
    advent_of_code::graph::Graph<
        (Row, Col, Option<Direction>, u8),
        (Row, Col),
    > for UltraCrucible
{
    type Edges = Vec<(Row, Col)>;

    fn edges(&self, v: (Row, Col, Option<Direction>, u8)) -> Self::Edges {
        let (row, col, direction, length) = v;

        if let Some(direction) = direction {
            let mut edges = vec![];
            if length + 1 >= 4 {
                edges.extend(direction.turns().into_iter().filter_map(
                    |direction| {
                        let offset = direction.offset();
                        add_offset(
                            row,
                            col,
                            offset.0,
                            offset.1,
                            self.map.rows(),
                            self.map.cols(),
                        )
                    },
                ));
            }
            if length + 1 < 10 {
                let offset = direction.offset();
                if let Some((row, col)) = add_offset(
                    row,
                    col,
                    offset.0,
                    offset.1,
                    self.map.rows(),
                    self.map.cols(),
                ) {
                    edges.push((row, col));
                }
            }
            edges
        } else {
            [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
            .into_iter()
            .filter_map(|direction| {
                let offset = direction.offset();
                add_offset(
                    row,
                    col,
                    offset.0,
                    offset.1,
                    self.map.rows(),
                    self.map.cols(),
                )
            })
            .collect()
        }
    }

    fn edge(
        &self,
        v: (Row, Col, Option<Direction>, u8),
        e: (Row, Col),
    ) -> ((Row, Col, Option<Direction>, u8), u64) {
        let (row, col, direction, length) = v;
        let (new_row, new_col) = e;
        let new_direction = Direction::from_pos(row, col, new_row, new_col);

        (
            (
                new_row,
                new_col,
                Some(new_direction),
                if direction == Some(new_direction) {
                    length + 1
                } else {
                    0
                },
            ),
            u64::from(self.map[new_row][new_col]),
        )
    }
}

pub fn parse(fh: File) -> Result<Grid<u8>> {
    Ok(parse::digit_grid(parse::raw_lines(fh)))
}

pub fn part1(map: Grid<u8>) -> Result<i64> {
    let crucible = Crucible { map };
    let (weight, shortest_path) =
        crucible.dijkstra((Row(0), Col(0), None, 0), |(row, col, _, _)| {
            row == crucible.map.rows() - 1 && col == crucible.map.cols() - 1
        });
    Ok(weight.try_into().unwrap())
}

pub fn part2(map: Grid<u8>) -> Result<i64> {
    let crucible = UltraCrucible { map };
    let (weight, shortest_path) = crucible.dijkstra(
        (Row(0), Col(0), None, 0),
        |(row, col, _, length)| {
            row == crucible.map.rows() - 1
                && col == crucible.map.cols() - 1
                && length + 1 >= 4
        },
    );
    Ok(weight.try_into().unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 17).unwrap()).unwrap()).unwrap(),
        1076
    );
    assert_eq!(
        part2(parse(parse::data(2023, 17).unwrap()).unwrap()).unwrap(),
        1219
    );
}
