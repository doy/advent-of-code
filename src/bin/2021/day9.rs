use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Grid<u8>> {
    Ok(parse::digit_grid(parse::raw_lines(fh)))
}

pub fn part1(map: Grid<u8>) -> Result<u64> {
    let mut risk = 0;
    for ((row, col), pos) in map.indexed_cells() {
        if map
            .adjacent(row, col, false)
            .map(|(row, col)| map[row][col])
            .all(|n| *pos < n)
        {
            risk += 1 + u64::from(*pos);
        }
    }
    Ok(risk)
}

pub fn part2(map: Grid<u8>) -> Result<u64> {
    let mut low = vec![];
    for ((row, col), pos) in map.indexed_cells() {
        if map
            .adjacent(row, col, false)
            .map(|(row, col)| map[row][col])
            .all(|n| *pos < n)
        {
            low.push((row, col));
        }
    }

    let mut sizes = vec![];
    for (row, col) in low {
        let mut basin: Grid<bool> = Grid::default();
        basin.grow(map.rows(), map.cols());
        let mut check = vec![(row, col)];
        let mut count = 0;
        while let Some((row, col)) = check.pop() {
            if basin[row][col] || map[row][col] == 9 {
                continue;
            }

            basin[row][col] = true;
            count += 1;

            for (row, col) in basin.adjacent(row, col, false) {
                if !basin[row][col] {
                    check.push((row, col));
                }
            }
        }
        sizes.push(count);
    }
    sizes.sort_unstable();
    Ok(sizes[sizes.len() - 1]
        * sizes[sizes.len() - 2]
        * sizes[sizes.len() - 3])
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2021, 9).unwrap()).unwrap()).unwrap(),
        570
    );
    assert_eq!(
        part2(parse(parse::data(2021, 9).unwrap()).unwrap()).unwrap(),
        899392
    );
}
