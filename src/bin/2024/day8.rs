use advent_of_code::prelude::*;

fn in_bounds<T: Clone + Eq + std::hash::Hash>(
    grid: &Grid<T>,
    pos: (IRow, ICol),
) -> bool {
    pos.0 >= IRow(0)
        && pos.0 < IRow(grid.rows().0.try_into().unwrap())
        && pos.1 >= ICol(0)
        && pos.1 < ICol(grid.cols().0.try_into().unwrap())
}

pub fn parse(fh: File) -> Result<Grid<Option<u8>>> {
    Ok(parse::grid(parse::raw_lines(fh), |c, _, _| {
        if c == b'.' {
            None
        } else {
            Some(c)
        }
    }))
}

pub fn part1(map: Grid<Option<u8>>) -> Result<i64> {
    let antennas = map
        .indexed_cells()
        .filter_map(|((row, col), c)| {
            c.map(|c| {
                (
                    c,
                    (
                        IRow(row.0.try_into().unwrap()),
                        ICol(col.0.try_into().unwrap()),
                    ),
                )
            })
        })
        .fold(
            HashMap::<u8, Vec<(IRow, ICol)>>::new(),
            |mut acc, (c, (row, col))| {
                let entry = acc.entry(c).or_default();
                entry.push((row, col));
                acc
            },
        );
    let mut antinodes = HashSet::new();
    for (_, positions) in antennas {
        for (i, pos1) in positions.iter().enumerate() {
            for pos2 in &positions[i + 1..] {
                let diff = (pos1.0 - pos2.0 .0, pos1.1 - pos2.1 .0);
                let antinode1 = (pos1.0 + diff.0, pos1.1 + diff.1);
                let antinode2 = (pos2.0 - diff.0 .0, pos2.1 - diff.1 .0);
                if in_bounds(&map, antinode1) {
                    antinodes.insert(antinode1);
                }
                if in_bounds(&map, antinode2) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }
    Ok(antinodes.len().try_into().unwrap())
}

pub fn part2(map: Grid<Option<u8>>) -> Result<i64> {
    let antennas = map
        .indexed_cells()
        .filter_map(|((row, col), c)| {
            c.map(|c| {
                (
                    c,
                    (
                        IRow(row.0.try_into().unwrap()),
                        ICol(col.0.try_into().unwrap()),
                    ),
                )
            })
        })
        .fold(
            HashMap::<u8, Vec<(IRow, ICol)>>::new(),
            |mut acc, (c, (row, col))| {
                let entry = acc.entry(c).or_default();
                entry.push((row, col));
                acc
            },
        );
    let mut antinodes = HashSet::new();
    for (_, positions) in antennas {
        for (i, pos1) in positions.iter().enumerate() {
            for pos2 in &positions[i + 1..] {
                let diff = (pos1.0 - pos2.0 .0, pos1.1 - pos2.1 .0);
                let mut antinode1 = (pos1.0, pos1.1);
                while in_bounds(&map, antinode1) {
                    antinodes.insert(antinode1);
                    antinode1 = (antinode1.0 + diff.0, antinode1.1 + diff.1);
                }
                let mut antinode2 = (pos2.0, pos2.1);
                while in_bounds(&map, antinode2) {
                    antinodes.insert(antinode2);
                    antinode2 =
                        (antinode2.0 - diff.0 .0, antinode2.1 - diff.1 .0);
                }
            }
        }
    }
    Ok(antinodes.len().try_into().unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 8).unwrap()).unwrap()).unwrap(),
        376
    );
    assert_eq!(
        part2(parse(parse::data(2024, 8).unwrap()).unwrap()).unwrap(),
        1352
    );
}
