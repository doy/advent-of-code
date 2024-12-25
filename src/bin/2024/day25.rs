use advent_of_code::prelude::*;

#[derive(Debug)]
pub struct LockOrKey {
    heights: [u8; 5],
    size: u8,
}

impl LockOrKey {
    fn fits(&self, other: &Self) -> bool {
        self.heights
            .iter()
            .zip(other.heights.iter())
            .all(|(a, b)| a + b <= self.size)
    }
}

pub fn parse(fh: File) -> Result<(Vec<LockOrKey>, Vec<LockOrKey>)> {
    let mut lines = parse::raw_lines(fh).peekable();
    let mut keys = vec![];
    let mut locks = vec![];
    while lines.peek().is_some() {
        let grid = parse::grid(parse::chunk(&mut lines), |c, _| match c {
            b'#' => true,
            b'.' => false,
            _ => unreachable!(),
        });
        if grid[Row(0)][Col(0)] {
            let lock: Vec<_> = grid
                .each_col()
                .map(|col| {
                    u8::try_from(
                        grid.each_row()
                            .position(|row| !grid[row][col])
                            .unwrap_or(grid.rows().0),
                    )
                    .unwrap()
                })
                .collect();
            let lock = std::array::from_fn(|i| lock[i]);
            locks.push(LockOrKey {
                heights: lock,
                size: u8::try_from(grid.rows().0).unwrap(),
            });
        } else {
            let key: Vec<_> = grid
                .each_col()
                .map(|col| {
                    u8::try_from(
                        grid.each_row()
                            .rev()
                            .position(|row| !grid[row][col])
                            .unwrap_or(grid.rows().0),
                    )
                    .unwrap()
                })
                .collect();
            let key = std::array::from_fn(|i| key[i]);
            keys.push(LockOrKey {
                heights: key,
                size: u8::try_from(grid.rows().0).unwrap(),
            });
        }
    }
    Ok((keys, locks))
}

pub fn part1((keys, locks): (Vec<LockOrKey>, Vec<LockOrKey>)) -> Result<i64> {
    let mut total = 0;
    for key in &keys {
        for lock in &locks {
            if key.fits(lock) {
                total += 1;
            }
        }
    }
    Ok(total)
}

pub fn part2(_: (Vec<LockOrKey>, Vec<LockOrKey>)) -> Result<i64> {
    Ok(0)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 25).unwrap()).unwrap()).unwrap(),
        3291
    );
    assert_eq!(
        part2(parse(parse::data(2024, 25).unwrap()).unwrap()).unwrap(),
        0
    );
}
