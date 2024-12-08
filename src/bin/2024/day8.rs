use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Grid<Option<u8>>> {
    Ok(parse::grid(parse::raw_lines(fh), |c, _| {
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
        .filter_map(|(pos, c)| c.map(|c| (c, pos.i())))
        .fold(HashMap::<u8, Vec<IPos>>::new(), |mut acc, (c, pos)| {
            let entry = acc.entry(c).or_default();
            entry.push(pos);
            acc
        });
    let mut antinodes = HashSet::new();
    for (_, positions) in antennas {
        for (i, pos1) in positions.iter().enumerate() {
            for pos2 in &positions[i + 1..] {
                let diff = *pos1 - *pos2;
                let antinode1 = *pos1 + diff;
                let antinode2 = *pos2 - diff;
                if map.in_bounds(antinode1) {
                    antinodes.insert(antinode1);
                }
                if map.in_bounds(antinode2) {
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
        .filter_map(|(pos, c)| c.map(|c| (c, pos.i())))
        .fold(HashMap::<u8, Vec<IPos>>::new(), |mut acc, (c, pos)| {
            let entry = acc.entry(c).or_default();
            entry.push(pos);
            acc
        });
    let mut antinodes = HashSet::new();
    for (_, positions) in antennas {
        for (i, pos1) in positions.iter().enumerate() {
            for pos2 in &positions[i + 1..] {
                let diff = *pos1 - *pos2;
                let mut antinode1 = *pos1;
                while map.in_bounds(antinode1) {
                    antinodes.insert(antinode1);
                    antinode1 = antinode1 + diff;
                }
                let mut antinode2 = *pos2;
                while map.in_bounds(antinode2) {
                    antinodes.insert(antinode2);
                    antinode2 = antinode2 - diff;
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
