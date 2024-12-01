use advent_of_code::prelude::*;

enum Mirror {
    Horizontal(usize),
    Vertical(usize),
}

impl Mirror {
    fn score(&self) -> usize {
        match self {
            Self::Horizontal(n) => 100 * n,
            Self::Vertical(n) => *n,
        }
    }
}

fn find_mirror(pattern: &Grid<bool>, smudges: usize) -> Mirror {
    'mirror: for row in pattern.each_row().skip(1) {
        let mut found_smudges = 0;
        for offset in 0..row.min(pattern.rows() - row.0).0 {
            let a = pattern.row_vec(row + offset);
            let b = pattern.row_vec(row - offset - 1);
            found_smudges += a
                .into_iter()
                .zip(b.into_iter())
                .filter(|(a, b)| a != b)
                .count();
            if found_smudges > smudges {
                continue 'mirror;
            }
        }
        if found_smudges != smudges {
            continue 'mirror;
        }
        return Mirror::Horizontal(row.0);
    }

    'mirror: for col in pattern.each_col().skip(1) {
        let mut found_smudges = 0;
        for offset in 0..col.min(pattern.cols() - col.0).0 {
            let a = pattern.col_vec(col + offset);
            let b = pattern.col_vec(col - offset - 1);
            found_smudges += a
                .into_iter()
                .zip(b.into_iter())
                .filter(|(a, b)| a != b)
                .count();
            if found_smudges > smudges {
                continue 'mirror;
            }
        }
        if found_smudges != smudges {
            continue 'mirror;
        }
        return Mirror::Vertical(col.0);
    }

    unreachable!()
}

pub fn parse(fh: File) -> Result<Vec<Grid<bool>>> {
    let mut lines = parse::raw_lines(fh).peekable();
    let mut grids = vec![];
    while lines.peek().is_some() {
        grids
            .push(parse::grid(parse::chunk(&mut lines), |c, _, _| c == b'#'));
    }
    Ok(grids)
}

pub fn part1(patterns: Vec<Grid<bool>>) -> Result<i64> {
    Ok(patterns
        .into_iter()
        .map(|pattern| find_mirror(&pattern, 0).score())
        .sum::<usize>()
        .try_into()
        .unwrap())
}

pub fn part2(patterns: Vec<Grid<bool>>) -> Result<i64> {
    Ok(patterns
        .into_iter()
        .map(|pattern| find_mirror(&pattern, 1).score())
        .sum::<usize>()
        .try_into()
        .unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 13).unwrap()).unwrap()).unwrap(),
        37975
    );
    assert_eq!(
        part2(parse(parse::data(2023, 13).unwrap()).unwrap()).unwrap(),
        32497
    );
}
