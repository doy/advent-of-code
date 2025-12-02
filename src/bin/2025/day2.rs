use advent_of_code::prelude::*;

pub fn parse(
    fh: File,
) -> Result<impl Iterator<Item = std::ops::RangeInclusive<i64>>> {
    Ok(parse::split::<_, String>(fh, b',').map(|range| {
        let mut parts = range.split('-');
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        start..=end
    }))
}

fn is_silly(i: i64) -> bool {
    let digits = i.ilog10() + 1;
    if !digits.is_multiple_of(2) {
        return false;
    }
    let q = 10i64.pow(digits / 2);
    i / q == i % q
}

fn is_extra_silly(i: i64) -> bool {
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

pub fn part1(
    ranges: impl Iterator<Item = std::ops::RangeInclusive<i64>>,
) -> Result<i64> {
    let mut total = 0;
    for range in ranges {
        for i in range {
            if is_silly(i) {
                total += i;
            }
        }
    }
    Ok(total)
}

pub fn part2(
    ranges: impl Iterator<Item = std::ops::RangeInclusive<i64>>,
) -> Result<i64> {
    let mut total = 0;
    for range in ranges {
        for i in range {
            if is_extra_silly(i) {
                total += i;
            }
        }
    }
    Ok(total)
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
