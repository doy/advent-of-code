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

fn tilt_north(map: &mut Grid<Cell>) {
    for row in map.each_row().skip(1) {
        for col in map.each_col() {
            if map[row][col] != Cell::Round {
                continue;
            }
            let mut dest_row = row;
            while dest_row > Row(0) {
                let next_row = dest_row - 1usize;
                if map[next_row][col] != Cell::Floor {
                    break;
                }
                dest_row = next_row;
            }
            if row != dest_row {
                map[dest_row][col] = Cell::Round;
                map[row][col] = Cell::Floor;
            }
        }
    }
}

fn tilt_west(map: &mut Grid<Cell>) {
    for row in map.each_row() {
        for col in map.each_col().skip(1) {
            if map[row][col] != Cell::Round {
                continue;
            }
            let mut dest_col = col;
            while dest_col > Col(0) {
                let next_col = dest_col - 1;
                if map[row][next_col] != Cell::Floor {
                    break;
                }
                dest_col = next_col;
            }
            if col != dest_col {
                map[row][dest_col] = Cell::Round;
                map[row][col] = Cell::Floor;
            }
        }
    }
}

fn tilt_south(map: &mut Grid<Cell>) {
    for row in map.each_row().rev().skip(1) {
        for col in map.each_col() {
            if map[row][col] != Cell::Round {
                continue;
            }
            let mut dest_row = row;
            while dest_row < map.rows() - 1 {
                let next_row = dest_row + 1usize;
                if map[next_row][col] != Cell::Floor {
                    break;
                }
                dest_row = next_row;
            }
            if row != dest_row {
                map[dest_row][col] = Cell::Round;
                map[row][col] = Cell::Floor;
            }
        }
    }
}

fn tilt_east(map: &mut Grid<Cell>) {
    for row in map.each_row() {
        for col in map.each_col().rev().skip(1) {
            if map[row][col] != Cell::Round {
                continue;
            }
            let mut dest_col = col;
            while dest_col < map.cols() - 1 {
                let next_col = dest_col + 1;
                if map[row][next_col] != Cell::Floor {
                    break;
                }
                dest_col = next_col;
            }
            if col != dest_col {
                map[row][dest_col] = Cell::Round;
                map[row][col] = Cell::Floor;
            }
        }
    }
}

fn weight(map: Grid<Cell>) -> usize {
    map.indexed_cells()
        .map(|(Pos(row, _), cell)| {
            if *cell == Cell::Round {
                (map.rows() - row.0).0
            } else {
                0
            }
        })
        .sum()
}

pub fn parse(fh: File) -> Result<Grid<Cell>> {
    Ok(parse::grid(parse::raw_lines(fh), |c, _| {
        c.try_into().unwrap()
    }))
}

pub fn part1(mut map: Grid<Cell>) -> Result<i64> {
    tilt_north(&mut map);
    Ok(weight(map).try_into().unwrap())
}

pub fn part2(mut map: Grid<Cell>) -> Result<i64> {
    let mut seen = HashMap::new();
    let mut i = 0;
    loop {
        i += 1;
        tilt_north(&mut map);
        tilt_west(&mut map);
        tilt_south(&mut map);
        tilt_east(&mut map);
        let seen_times: &mut Vec<_> = seen.entry(map.clone()).or_default();
        seen_times.push(i);
        if seen_times.len() > 1 {
            break;
        }
    }
    let found = seen
        .iter()
        .find_map(|(_, v)| if v.len() == 2 { Some(v) } else { None })
        .unwrap();
    let iterations =
        found[0] + ((1_000_000_000 - found[0]) % (found[1] - found[0]));
    let found_map = seen
        .into_iter()
        .find_map(|(map, v)| {
            if v.contains(&iterations) {
                Some(map)
            } else {
                None
            }
        })
        .unwrap();
    Ok(weight(found_map).try_into().unwrap())
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
