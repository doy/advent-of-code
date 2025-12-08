use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Vec<std::ops::RangeInclusive<i64>>> {
    Ok(parse::split::<_, String>(fh, b',')
        .map(|range| {
            let mut parts = range.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            start..=end
        })
        .collect())
}

fn is_silly(i: &i64) -> bool {
    let digits = i.ilog10() + 1;
    if !digits.is_multiple_of(2) {
        return false;
    }
    let q = 10i64.pow(digits / 2);
    i / q == i % q
}

fn is_extra_silly(i: &i64) -> bool {
    let digits = i.ilog10() + 1;
    for n in 2..=digits {
        if digits.is_multiple_of(n) {
            let q = 10i64.pow(digits / n);
            if (1..n).all(|c| (i / q.pow(c)) % q == i % q) {
                return true;
            }
        }
    }
    false
}

pub fn part1(ranges: Vec<std::ops::RangeInclusive<i64>>) -> Result<i64> {
    Ok(ranges
        .into_par_iter()
        .map(|range| range.into_iter().filter(is_silly).sum::<i64>())
        .sum())
}

pub fn part2(ranges: Vec<std::ops::RangeInclusive<i64>>) -> Result<i64> {
    Ok(ranges
        .into_par_iter()
        .map(|range| range.into_iter().filter(is_extra_silly).sum::<i64>())
        .sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2025, 2).unwrap()).unwrap()).unwrap(),
        28844599675
    );
    assert_eq!(
        part2(parse(parse::data(2025, 2).unwrap()).unwrap()).unwrap(),
        48778605167
    );
}
