use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Grid<u8>> {
    Ok(parse::digit_grid(parse::raw_lines(fh)))
}

pub fn part1(map: Grid<u8>) -> Result<u64> {
    let mut risk = 0;
    for (pos, c) in map.indexed_cells() {
        if map.adjacent(pos, false).map(|pos| map[pos]).all(|n| *c < n) {
            risk += 1 + u64::from(*c);
        }
    }
    Ok(risk)
}

pub fn part2(map: Grid<u8>) -> Result<u64> {
    let mut low = vec![];
    for (pos, c) in map.indexed_cells() {
        if map.adjacent(pos, false).map(|pos| map[pos]).all(|n| *c < n) {
            low.push(pos);
        }
    }

    let mut sizes = vec![];
    for pos in low {
        let mut basin: Grid<bool> = Grid::default();
        basin.grow(map.size());
        let mut check = vec![pos];
        let mut count = 0;
        while let Some(pos) = check.pop() {
            if basin[pos] || map[pos] == 9 {
                continue;
            }

            basin[pos] = true;
            count += 1;

            for pos in basin.adjacent(pos, false) {
                if !basin[pos] {
                    check.push(pos);
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
