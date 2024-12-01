use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Grid<bool>> {
    Ok(parse::grid(parse::raw_lines(fh), |c, _, _| c == b'#'))
}

pub fn part1(mut map: Grid<bool>) -> Result<i64> {
    let mut expand_rows = vec![];
    for row in map.each_row() {
        if map[row].iter().all(|b| !b) {
            expand_rows.push(row);
        }
    }
    let mut expand_cols = vec![];
    for col in map.each_col() {
        if map.each_row().map(|row| map[row][col]).all(|b| !b) {
            expand_cols.push(col);
        }
    }
    for row in expand_rows.into_iter().rev() {
        map.insert_row(row);
    }
    for col in expand_cols.into_iter().rev() {
        map.insert_col(col);
    }

    let galaxies: HashSet<(Row, Col)> = map
        .indexed_cells()
        .filter_map(|(pos, galaxy)| if *galaxy { Some(pos) } else { None })
        .collect();

    let mut total = 0;
    for a in &galaxies {
        for b in &galaxies {
            total += a.0.abs_diff(b.0).0 + a.1.abs_diff(b.1).0
        }
    }

    Ok((total / 2).try_into().unwrap())
}

pub fn part2(map: Grid<bool>) -> Result<i64> {
    let mut expand_rows = vec![];
    for row in map.each_row() {
        if map[row].iter().all(|b| !b) {
            expand_rows.push(row);
        }
    }
    let mut expand_cols = vec![];
    for col in map.each_col() {
        if map.each_row().map(|row| map[row][col]).all(|b| !b) {
            expand_cols.push(col);
        }
    }

    let galaxies: HashSet<(Row, Col)> = map
        .indexed_cells()
        .filter_map(|(pos, galaxy)| if *galaxy { Some(pos) } else { None })
        .collect();

    let mut total = 0;
    for a in &galaxies {
        for b in &galaxies {
            let expanded_rows = expand_rows
                .iter()
                .filter(|row| (a.0.min(b.0)..a.0.max(b.0)).contains(row))
                .count();
            let expanded_cols = expand_cols
                .iter()
                .filter(|col| (a.1.min(b.1)..a.1.max(b.1)).contains(col))
                .count();
            total += a.0.abs_diff(b.0).0
                + a.1.abs_diff(b.1).0
                + 999999 * (expanded_rows + expanded_cols);
        }
    }

    Ok((total / 2).try_into().unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 11).unwrap()).unwrap()).unwrap(),
        10033566
    );
    assert_eq!(
        part2(parse(parse::data(2023, 11).unwrap()).unwrap()).unwrap(),
        560822911938
    );
}
