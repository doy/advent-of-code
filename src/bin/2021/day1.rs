use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Vec<i64>> {
    Ok(parse::lines(fh).collect())
}

pub fn part1(ints: Vec<i64>) -> Result<usize> {
    Ok(ints
        .windows(2)
        .map(|a| a[1] - a[0])
        .filter(|x| *x > 0)
        .count())
}

pub fn part2(ints: Vec<i64>) -> Result<usize> {
    Ok(ints
        .windows(3)
        .map(|a| a[0] + a[1] + a[2])
        .collect::<Vec<_>>()
        .windows(2)
        .map(|a| a[1] - a[0])
        .filter(|x| *x > 0)
        .count())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2021, 1).unwrap()).unwrap()).unwrap(),
        1602
    );
    assert_eq!(
        part2(parse(parse::data(2021, 1).unwrap()).unwrap()).unwrap(),
        1633
    );
}
