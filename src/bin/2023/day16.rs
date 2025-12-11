use advent_of_code::prelude::*;

fn add_offset(pos: Pos, offset: IPos, max: Size) -> Option<Pos> {
    if let (Some(row), Some(col)) = (
        pos.0.0.checked_add_signed(offset.0.0),
        pos.1.0.checked_add_signed(offset.1.0),
    ) {
        let row = Row(row);
        let col = Col(col);
        if row < max.0 && col < max.1 {
            return Some(Pos(row, col));
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
    fn count_energized_from(&self, pos: Pos, direction: Direction) -> usize {
        let mut energized = HashMap::new();
        let mut rays = vec![(pos, direction)];
        while let Some(ray) = rays.pop() {
            let (pos, direction) = ray;
            let directions: &mut Vec<_> = energized.entry(pos).or_default();
            if directions.contains(&direction) {
                continue;
            }
            directions.push(direction);
            let next = self.next(pos, direction);
            rays.extend(next);
        }
        energized.into_keys().count()
    }

    fn next(&self, pos: Pos, direction: Direction) -> Vec<(Pos, Direction)> {
        let directions = match self.map[pos] {
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
                add_offset(pos, offsets, self.map.size())
                    .map(|pos| (pos, direction))
            })
            .collect()
    }
}

pub fn parse(fh: File) -> Result<Map> {
    Ok(Map {
        map: parse::grid(parse::raw_lines(fh), |c, _| c.try_into().unwrap()),
    })
}

pub fn part1(map: Map) -> Result<i64> {
    Ok(map
        .count_energized_from(Pos(Row(0), Col(0)), Direction::Right)
        .try_into()
        .unwrap())
}

pub fn part2(map: Map) -> Result<i64> {
    Ok(map
        .map
        .each_row()
        .flat_map(|row| {
            [
                (Pos(row, Col(0)), Direction::Right),
                (Pos(row, map.map.cols() - 1), Direction::Left),
            ]
        })
        .chain(map.map.each_col().flat_map(|col| {
            [
                (Pos(Row(0), col), Direction::Down),
                (Pos(map.map.rows() - 1, col), Direction::Up),
            ]
        }))
        .map(|(pos, direction)| map.count_energized_from(pos, direction))
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
