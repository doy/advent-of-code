#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum Cell {
    #[default]
    Floor,
    Cube,
    Round,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Floor => '.',
                Self::Cube => '#',
                Self::Round => 'O',
            }
        )
    }
}

impl TryFrom<u8> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            b'.' => Self::Floor,
            b'#' => Self::Cube,
            b'O' => Self::Round,
            _ => bail!("unknown tile {value}"),
        })
    }
}

fn tilt(
    mut map: Grid<Cell>,
    roll: impl Fn(Row, Col, Row, Col) -> Option<(Row, Col)>,
) -> Grid<Cell> {
    loop {
        let mut changed = false;
        for row in map.each_row() {
            for col in map.each_col() {
                let Some((next_row, next_col)) =
                    roll(row, col, map.rows(), map.cols())
                else {
                    continue;
                };
                if map[row][col] == Cell::Round
                    && map[next_row][next_col] == Cell::Floor
                {
                    map[next_row][next_col] = Cell::Round;
                    map[row][col] = Cell::Floor;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    map
}

fn north(row: Row, col: Col, _rows: Row, _cols: Col) -> Option<(Row, Col)> {
    if row > Row(0) {
        Some((row - 1, col))
    } else {
        None
    }
}

fn west(row: Row, col: Col, _rows: Row, _cols: Col) -> Option<(Row, Col)> {
    if col > Col(0) {
        Some((row, col - 1))
    } else {
        None
    }
}

fn south(row: Row, col: Col, rows: Row, _cols: Col) -> Option<(Row, Col)> {
    if row < rows - 1 {
        Some((row + 1, col))
    } else {
        None
    }
}

fn east(row: Row, col: Col, _rows: Row, cols: Col) -> Option<(Row, Col)> {
    if col < cols - 1 {
        Some((row, col + 1))
    } else {
        None
    }
}

fn weight(map: Grid<Cell>) -> usize {
    map.indexed_cells()
        .map(|((row, col), cell)| {
            if *cell == Cell::Round {
                (map.rows() - row.0).0
            } else {
                0
            }
        })
        .sum()
}

pub fn parse(fh: File) -> Result<Grid<Cell>> {
    Ok(parse::grid(parse::raw_lines(fh), |c, _, _| {
        c.try_into().unwrap()
    }))
}

pub fn part1(map: Grid<Cell>) -> Result<i64> {
    Ok(weight(tilt(map, north)).try_into().unwrap())
}

pub fn part2(mut map: Grid<Cell>) -> Result<i64> {
    let orig_map = map.clone();
    let mut seen = HashMap::new();
    let mut i = 0;
    loop {
        map = tilt(tilt(tilt(tilt(map, north), west), south), east);
        let seen_times: &mut Vec<_> = seen.entry(map.clone()).or_default();
        seen_times.push(i);
        if seen_times.len() > 1 {
            break;
        }
        i += 1;
    }
    let found = seen
        .into_iter()
        .find_map(|(_, v)| if v.len() == 2 { Some(v) } else { None })
        .unwrap();
    let iterations =
        found[0] + ((1_000_000_000 - found[0]) % (found[1] - found[0]));
    map = orig_map;
    for _ in 0..iterations {
        map = tilt(tilt(tilt(tilt(map, north), west), south), east);
    }
    Ok(weight(map).try_into().unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 14).unwrap()).unwrap()).unwrap(),
        109596
    );
    assert_eq!(
        part2(parse(parse::data(2023, 14).unwrap()).unwrap()).unwrap(),
        96105
    );
}
