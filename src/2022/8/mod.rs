#![allow(dead_code)]
#![allow(unused_variables)]

use crate::prelude::*;

pub fn parse(fh: File) -> Result<Grid<u8>> {
    Ok(parse::digit_grid(parse::lines(fh)))
}

pub fn part1(trees: Grid<u8>) -> Result<i64> {
    let mut total = 0;
    for row in trees.each_row() {
        'tree: for col in trees.each_col() {
            let tree = trees[row][col];

            if trees
                .each_col()
                .take(col.0)
                .all(|col| trees[row][col] < tree)
            {
                total += 1;
                continue 'tree;
            }
            if trees
                .each_col()
                .skip(col.0 + 1)
                .all(|col| trees[row][col] < tree)
            {
                total += 1;
                continue 'tree;
            }
            if trees
                .each_row()
                .take(row.0)
                .all(|row| trees[row][col] < tree)
            {
                total += 1;
                continue 'tree;
            }
            if trees
                .each_row()
                .skip(row.0 + 1)
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
    for row in trees.each_row() {
        for col in trees.each_col() {
            let tree = trees[row][col];

            let mut seen = false;
            let left = trees
                .each_col()
                .take(col.0)
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
            let right = trees
                .each_col()
                .skip(col.0 + 1)
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
            let up = trees
                .each_row()
                .take(row.0)
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
            let down = trees
                .each_row()
                .skip(row.0 + 1)
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
