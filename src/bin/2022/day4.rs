use advent_of_code::prelude::*;

pub struct Pair {
    first: (i64, i64),
    second: (i64, i64),
}

impl Pair {
    fn contains(&self) -> bool {
        range_contains(self.first, self.second)
            || range_contains(self.second, self.first)
    }

    fn overlaps(&self) -> bool {
        range_overlaps(self.first, self.second)
            || range_overlaps(self.second, self.first)
    }
}

fn range_contains(a: (i64, i64), b: (i64, i64)) -> bool {
    (a.0..=a.1).contains(&b.0) && (a.0..=a.1).contains(&b.1)
}

fn range_overlaps(a: (i64, i64), b: (i64, i64)) -> bool {
    (a.0..=a.1).contains(&b.0) || (a.0..=a.1).contains(&b.1)
}

pub fn parse(fh: File) -> Result<impl Iterator<Item = Pair>> {
    Ok(parse::raw_lines(fh).map(|line| {
        let mut parts = line.split(',');
        let first = parts.next().unwrap();
        let mut first_parts = first.split('-');
        let second = parts.next().unwrap();
        let mut second_parts = second.split('-');
        Pair {
            first: (
                first_parts.next().unwrap().parse().unwrap(),
                first_parts.next().unwrap().parse().unwrap(),
            ),
            second: (
                second_parts.next().unwrap().parse().unwrap(),
                second_parts.next().unwrap().parse().unwrap(),
            ),
        }
    }))
}

pub fn part1(pairs: impl Iterator<Item = Pair>) -> Result<usize> {
    Ok(pairs.filter(|pair| pair.contains()).count())
}

pub fn part2(pairs: impl Iterator<Item = Pair>) -> Result<usize> {
    Ok(pairs.filter(|pair| pair.overlaps()).count())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 4).unwrap()).unwrap()).unwrap(),
        515
    );
    assert_eq!(
        part2(parse(parse::data(2022, 4).unwrap()).unwrap()).unwrap(),
        883
    );
}
