use advent_of_code::prelude::*;

fn word_horiz(grid: &Grid<u8>, pos: Pos) -> String {
    let mut chars = vec![0u8; 4];
    for (offset, char) in chars.iter_mut().enumerate() {
        if pos.1 + offset < grid.cols() {
            *char = grid[pos.0][pos.1 + offset];
        }
    }
    String::from_utf8(chars).unwrap()
}

fn word_vert(grid: &Grid<u8>, pos: Pos) -> String {
    let mut chars = vec![0u8; 4];
    for offset in 0..4 {
        if pos.0 + offset < grid.rows() {
            chars[offset] = grid[pos.0 + offset][pos.1];
        }
    }
    String::from_utf8(chars).unwrap()
}

fn word_diag(grid: &Grid<u8>, pos: Pos) -> String {
    let mut chars = vec![0u8; 4];
    for offset in 0..4 {
        if pos.0 + offset < grid.rows() && pos.1 + offset < grid.cols() {
            chars[offset] = grid[pos.0 + offset][pos.1 + offset];
        }
    }
    String::from_utf8(chars).unwrap()
}

fn word_diag2(grid: &Grid<u8>, pos: Pos) -> String {
    let mut chars = vec![0u8; 4];
    for offset in 0..4 {
        if pos.0 + offset < grid.rows() && pos.1.0 >= offset {
            chars[offset] = grid[pos.0 + offset][pos.1 - offset];
        }
    }
    String::from_utf8(chars).unwrap()
}

fn word_x(grid: &Grid<u8>, pos: Pos) -> Vec<u8> {
    let mut chars = vec![0u8; 5];
    if pos.0.0 > 0 && pos.1.0 > 0 {
        chars[0] = grid[pos.0 - 1usize][pos.1 - 1];
    }
    if pos.0.0 > 0 && pos.1 + 1 < grid.cols() {
        chars[1] = grid[pos.0 - 1usize][pos.1 + 1];
    }
    chars[2] = grid[pos.0][pos.1];
    if pos.0 + 1 < grid.rows() && pos.1.0 > 0 {
        chars[3] = grid[pos.0 + 1usize][pos.1 - 1];
    }
    if pos.0 + 1 < grid.rows() && pos.1 + 1 < grid.cols() {
        chars[4] = grid[pos.0 + 1usize][pos.1 + 1];
    }
    chars
}

pub fn parse(fh: File) -> Result<Grid<u8>> {
    Ok(parse::grid(parse::raw_lines(fh), |c, _| c))
}

pub fn part1(grid: Grid<u8>) -> Result<i64> {
    Ok(grid
        .par_indexed_cells()
        .map(|(pos, _)| {
            let mut total = 0;
            for word in [
                word_horiz(&grid, pos),
                word_vert(&grid, pos),
                word_diag(&grid, pos),
                word_diag2(&grid, pos),
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
        .map(|(pos, _)| {
            let word = word_x(&grid, pos);
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
