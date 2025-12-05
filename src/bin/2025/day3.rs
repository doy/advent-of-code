use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<impl Iterator<Item = Vec<i64>>> {
    Ok(parse::lines(fh).map(|line: String| {
        line.bytes().map(|c| i64::from(c - b'0')).collect()
    }))
}

fn joltage(bank: &[i64], cells: usize) -> i64 {
    if cells == 1 {
        return bank.iter().copied().max().unwrap();
    }
    let (pos, next) = bank
        .iter()
        .copied()
        .enumerate()
        .take(bank.len() - cells + 1)
        .max_by_key(|(i, n)| (*n, bank.len() - *i))
        .unwrap();
    next * 10i64.pow((cells - 1).try_into().unwrap())
        + joltage(&bank[pos + 1..], cells - 1)
}

pub fn part1(banks: impl Iterator<Item = Vec<i64>>) -> Result<i64> {
    Ok(banks.map(|bank| joltage(&bank, 2)).sum())
}

pub fn part2(banks: impl Iterator<Item = Vec<i64>>) -> Result<i64> {
    Ok(banks.map(|bank| joltage(&bank, 12)).sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2025, 3).unwrap()).unwrap()).unwrap(),
        16887
    );
    assert_eq!(
        part2(parse(parse::data(2025, 3).unwrap()).unwrap()).unwrap(),
        167302518850275
    );
}
