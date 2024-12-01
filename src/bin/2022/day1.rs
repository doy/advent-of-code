use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Vec<u64>> {
    let mut elves = vec![];
    let mut lines = parse::raw_lines(fh).peekable();
    while lines.peek().is_some() {
        let mut calories = 0;
        for line in parse::chunk(&mut lines) {
            calories += line.parse::<u64>()?;
        }
        elves.push(calories);
    }
    Ok(elves)
}

pub fn part1(elves: Vec<u64>) -> Result<u64> {
    Ok(elves.iter().copied().max().unwrap_or(0))
}

pub fn part2(mut elves: Vec<u64>) -> Result<u64> {
    elves.sort();
    Ok(elves.iter().rev().copied().take(3).sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 1).unwrap()).unwrap()).unwrap(),
        67658
    );
    assert_eq!(
        part2(parse(parse::data(2022, 1).unwrap()).unwrap()).unwrap(),
        200158
    );
}
