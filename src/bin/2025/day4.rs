use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Grid<bool>> {
    Ok(parse::bool_grid(parse::raw_lines(fh), b'@', b'.'))
}

pub fn part1(map: Grid<bool>) -> Result<i64> {
    Ok(map
        .indexed_cells()
        .map(|(pos, cell)| {
            *cell
                && map.adjacent(pos, true).filter(|pos| map[*pos]).count() < 4
        })
        .filter(|b| *b)
        .count()
        .try_into()
        .unwrap())
}

pub fn part2(mut map: Grid<bool>) -> Result<i64> {
    let mut removed = 0;
    loop {
        let to_remove: Vec<_> = map
            .indexed_cells()
            .filter_map(|(pos, cell)| {
                (*cell
                    && map
                        .adjacent(pos, true)
                        .filter(|pos| map[*pos])
                        .count()
                        < 4)
                .then_some(pos)
            })
            .collect();
        if to_remove.is_empty() {
            return Ok(removed);
        } else {
            removed += i64::try_from(to_remove.len()).unwrap();
            for pos in to_remove {
                map[pos] = false;
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2025, 4).unwrap()).unwrap()).unwrap(),
        1370
    );
    assert_eq!(
        part2(parse(parse::data(2025, 4).unwrap()).unwrap()).unwrap(),
        8437
    );
}
