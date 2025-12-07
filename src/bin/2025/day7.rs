use advent_of_code::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum Cell {
    Splitter,
    Beam(i64),
    #[default]
    None,
}

pub fn parse(fh: File) -> Result<Grid<Cell>> {
    Ok(parse::grid(parse::raw_lines(fh), |c, _| match c {
        b'^' => Cell::Splitter,
        b'S' => Cell::Beam(1),
        b'.' => Cell::None,
        _ => unreachable!(),
    }))
}

pub fn part1(mut map: Grid<Cell>) -> Result<i64> {
    let mut total = 0;
    for row in map.each_row() {
        for col in map.each_col() {
            let pos = Pos(row, col);
            let cell = map[pos];
            if let Cell::Beam(_) = cell {
                let next = pos + Pos(Row(1), Col(0));
                if map.in_bounds(next.i()) {
                    if map[next] == Cell::Splitter {
                        map[next - Pos(Row(0), Col(1))] = Cell::Beam(1);
                        map[next + Pos(Row(0), Col(1))] = Cell::Beam(1);
                        total += 1;
                    } else {
                        map[next] = Cell::Beam(1);
                    }
                }
            }
        }
    }
    Ok(total)
}

pub fn part2(mut map: Grid<Cell>) -> Result<i64> {
    for row in map.each_row() {
        for col in map.each_col() {
            let pos = Pos(row, col);
            let cell = map[pos];
            if let Cell::Beam(histories) = cell {
                let next = pos + Pos(Row(1), Col(0));
                if map.in_bounds(next.i()) {
                    if map[next] == Cell::Splitter {
                        let left = next - Pos(Row(0), Col(1));
                        let existing_histories =
                            if let Cell::Beam(histories) = map[left] {
                                histories
                            } else {
                                0
                            };
                        map[left] =
                            Cell::Beam(histories + existing_histories);

                        let right = next + Pos(Row(0), Col(1));
                        let existing_histories =
                            if let Cell::Beam(histories) = map[right] {
                                histories
                            } else {
                                0
                            };
                        map[right] =
                            Cell::Beam(histories + existing_histories);
                    } else {
                        let existing_histories =
                            if let Cell::Beam(histories) = map[next] {
                                histories
                            } else {
                                0
                            };
                        map[next] =
                            Cell::Beam(histories + existing_histories);
                    }
                }
            }
        }
    }
    Ok(map[map.rows() - 1usize]
        .iter()
        .copied()
        .map(|cell| match cell {
            Cell::Beam(histories) => histories,
            _ => 0,
        })
        .sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2025, 7).unwrap()).unwrap()).unwrap(),
        1609
    );
    assert_eq!(
        part2(parse(parse::data(2025, 7).unwrap()).unwrap()).unwrap(),
        12472142047197
    );
}
