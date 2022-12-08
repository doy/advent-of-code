#![allow(dead_code)]
#![allow(unused_variables)]

use crate::prelude::*;

pub fn parse(fh: File) -> Result<Grid<u8>> {
    Ok(parse::digit_grid(parse::lines(fh)))
}

pub fn part1(trees: Grid<u8>) -> Result<i64> {
    let mut total = 0;
    for row in (0..trees.rows().0).map(Row) {
        'tree: for col in (0..trees.cols().0).map(Col) {
            let tree = trees[row][col];

            if (0..col.0).map(Col).all(|col| trees[row][col] < tree) {
                total += 1;
                continue 'tree;
            }
            if ((col.0 + 1)..trees.cols().0)
                .map(Col)
                .all(|col| trees[row][col] < tree)
            {
                total += 1;
                continue 'tree;
            }
            if (0..row.0).map(Row).all(|row| trees[row][col] < tree) {
                total += 1;
                continue 'tree;
            }
            if ((row.0 + 1)..trees.rows().0)
                .map(Row)
                .all(|row| trees[row][col] < tree)
            {
                total += 1;
                continue 'tree;
            }
        }
    }
    Ok(total)
}

pub fn part2(trees: Grid<u8>) -> Result<i64> {
    let mut max = 0;
    for row in (0..trees.rows().0).map(Row) {
        for col in (0..trees.cols().0).map(Col) {
            let tree = trees[row][col];

            let mut seen = false;
            let left = (0..col.0)
                .map(Col)
                .rev()
                .take_while(|col| {
                    if seen {
                        return false;
                    }

                    let other = trees[row][*col];
                    if other < tree {
                        true
                    } else {
                        seen = true;
                        true
                    }
                })
                .count();

            let mut seen = false;
            let right = ((col.0 + 1)..trees.cols().0)
                .map(Col)
                .take_while(|col| {
                    if seen {
                        return false;
                    }

                    let other = trees[row][*col];
                    if other < tree {
                        true
                    } else {
                        seen = true;
                        true
                    }
                })
                .count();

            let mut seen = false;
            let up = (0..row.0)
                .map(Row)
                .rev()
                .take_while(|row| {
                    if seen {
                        return false;
                    }

                    let other = trees[*row][col];
                    if other < tree {
                        true
                    } else {
                        seen = true;
                        true
                    }
                })
                .count();

            let mut seen = false;
            let down = ((row.0 + 1)..trees.rows().0)
                .map(Row)
                .take_while(|row| {
                    if seen {
                        return false;
                    }

                    let other = trees[*row][col];
                    if other < tree {
                        true
                    } else {
                        seen = true;
                        true
                    }
                })
                .count();

            let scenic = left * right * up * down;
            if scenic > max {
                max = scenic;
            }
        }
    }
    Ok(max.try_into().unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 8).unwrap()).unwrap()).unwrap(),
        1851
    );
    assert_eq!(
        part2(parse(parse::data(2022, 8).unwrap()).unwrap()).unwrap(),
        574080
    );
}
