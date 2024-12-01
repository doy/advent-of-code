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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum Tile {
    #[default]
    Floor,
    Vertical,
    Horizontal,
    Rising,
    Falling,
}

impl TryFrom<u8> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            b'.' => Self::Floor,
            b'|' => Self::Vertical,
            b'-' => Self::Horizontal,
            b'/' => Self::Rising,
            b'\\' => Self::Falling,
            _ => bail!("unknown tile {value}"),
        })
    }
}

pub struct Map {
    map: Grid<Tile>,
}

impl Map {
    fn count_energized_from(
        &self,
        row: Row,
        col: Col,
        direction: Direction,
    ) -> usize {
        let mut energized = HashMap::new();
        let mut rays = vec![(row, col, direction)];
        while let Some(ray) = rays.pop() {
            let (row, col, direction) = ray;
            let directions: &mut Vec<_> =
                energized.entry((row, col)).or_default();
            if directions.contains(&direction) {
                continue;
            }
            directions.push(direction);
            let next = self.next(row, col, direction);
            rays.extend(next);
        }
        energized.into_keys().count()
    }

    fn next(
        &self,
        row: Row,
        col: Col,
        direction: Direction,
    ) -> Vec<(Row, Col, Direction)> {
        let directions = match self.map[row][col] {
            Tile::Floor => vec![direction],
            Tile::Vertical => {
                if direction.horizontal() {
                    vec![Direction::Up, Direction::Down]
                } else {
                    vec![direction]
                }
            }
            Tile::Horizontal => {
                if direction.horizontal() {
                    vec![direction]
                } else {
                    vec![Direction::Left, Direction::Right]
                }
            }
            Tile::Rising => vec![match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            }],
            Tile::Falling => vec![match direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            }],
        };
        directions
            .into_iter()
            .filter_map(|direction| {
                let offsets = direction.offset();
                add_offset(
                    row,
                    col,
                    offsets.0,
                    offsets.1,
                    self.map.rows(),
                    self.map.cols(),
                )
                .map(|(row, col)| (row, col, direction))
            })
            .collect()
    }
}

pub fn parse(fh: File) -> Result<Map> {
    Ok(Map {
        map: parse::grid(parse::raw_lines(fh), |c, _, _| {
            c.try_into().unwrap()
        }),
    })
}

pub fn part1(map: Map) -> Result<i64> {
    Ok(map
        .count_energized_from(Row(0), Col(0), Direction::Right)
        .try_into()
        .unwrap())
}

pub fn part2(map: Map) -> Result<i64> {
    Ok(map
        .map
        .each_row()
        .flat_map(|row| {
            [
                (row, Col(0), Direction::Right),
                (row, map.map.cols() - 1, Direction::Left),
            ]
        })
        .chain(map.map.each_col().flat_map(|col| {
            [
                (Row(0), col, Direction::Down),
                (map.map.rows() - 1, col, Direction::Up),
            ]
        }))
        .map(|(row, col, direction)| {
            map.count_energized_from(row, col, direction)
        })
        .max()
        .unwrap()
        .try_into()
        .unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 16).unwrap()).unwrap()).unwrap(),
        7498
    );
    assert_eq!(
        part2(parse(parse::data(2023, 16).unwrap()).unwrap()).unwrap(),
        7846
    );
}
