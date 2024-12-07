use advent_of_code::prelude::*;
use rayon::iter::ParallelIterator as _;

fn word_horiz(grid: &Grid<u8>, row: Row, col: Col) -> String {
    let mut chars = vec![0u8; 4];
    for (offset, char) in chars.iter_mut().enumerate() {
        if col + offset < grid.cols() {
            *char = grid[row][col + offset];
        }
    }
    String::from_utf8(chars).unwrap()
}

fn word_vert(grid: &Grid<u8>, row: Row, col: Col) -> String {
    let mut chars = vec![0u8; 4];
    for offset in 0..4 {
        if row + offset < grid.rows() {
            chars[offset] = grid[row + offset][col];
        }
    }
    String::from_utf8(chars).unwrap()
}

fn word_diag(grid: &Grid<u8>, row: Row, col: Col) -> String {
    let mut chars = vec![0u8; 4];
    for offset in 0..4 {
        if row + offset < grid.rows() && col + offset < grid.cols() {
            chars[offset] = grid[row + offset][col + offset];
        }
    }
    String::from_utf8(chars).unwrap()
}

fn word_diag2(grid: &Grid<u8>, row: Row, col: Col) -> String {
    let mut chars = vec![0u8; 4];
    for offset in 0..4 {
        if row + offset < grid.rows() && col.0 >= offset {
            chars[offset] = grid[row + offset][col - offset];
        }
    }
    String::from_utf8(chars).unwrap()
}

fn word_x(grid: &Grid<u8>, row: Row, col: Col) -> Vec<u8> {
    let mut chars = vec![0u8; 5];
    if row.0 > 0 && col.0 > 0 {
        chars[0] = grid[row - 1][col - 1];
    }
    if row.0 > 0 && col + 1 < grid.cols() {
        chars[1] = grid[row - 1][col + 1];
    }
    chars[2] = grid[row][col];
    if row + 1 < grid.rows() && col.0 > 0 {
        chars[3] = grid[row + 1][col - 1];
    }
    if row + 1 < grid.rows() && col + 1 < grid.cols() {
        chars[4] = grid[row + 1][col + 1];
    }
    chars
}

pub fn parse(fh: File) -> Result<Grid<u8>> {
    Ok(parse::grid(parse::lines(fh), |c, _, _| c))
}

pub fn part1(grid: Grid<u8>) -> Result<i64> {
    Ok(grid
        .par_indexed_cells()
        .map(|((row, col), _)| {
            let mut total = 0;
            for word in [
                word_horiz(&grid, row, col),
                word_vert(&grid, row, col),
                word_diag(&grid, row, col),
                word_diag2(&grid, row, col),
            ] {
                if word == "XMAS" {
                    total += 1;
                }
                if word == "SAMX" {
                    total += 1;
                }
            }
            total
        })
        .sum())
}

pub fn part2(grid: Grid<u8>) -> Result<i64> {
    Ok(grid
        .par_indexed_cells()
        .map(|((row, col), _)| {
            let word = word_x(&grid, row, col);
            if word[2] == b'A' {
                if word[0] == b'M' && word[4] == b'S'
                    || word[0] == b'S' && word[4] == b'M'
                {
                    if word[1] == b'M' && word[3] == b'S'
                        || word[1] == b'S' && word[3] == b'M'
                    {
                        return 1;
                    }
                }
            }
            0
        })
        .sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 4).unwrap()).unwrap()).unwrap(),
        2545
    );
    assert_eq!(
        part2(parse(parse::data(2024, 4).unwrap()).unwrap()).unwrap(),
        1886
    );
}
