use advent_of_code::prelude::*;

pub struct Pantry {
    fresh: Vec<std::ops::RangeInclusive<i64>>,
    ingredients: Vec<i64>,
}

pub fn parse(fh: File) -> Result<Pantry> {
    let mut lines = parse::raw_lines(fh);
    let fresh = parse::chunk(&mut lines)
        .map(|range_str| {
            let mut parts = range_str.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            start..=end
        })
        .collect();
    let ingredients = parse::chunk(&mut lines)
        .map(|s| s.parse().unwrap())
        .collect();
    Ok(Pantry { fresh, ingredients })
}

pub fn part1(pantry: Pantry) -> Result<i64> {
    Ok(pantry
        .ingredients
        .iter()
        .filter(|i| pantry.fresh.iter().any(|r| r.contains(i)))
        .count()
        .try_into()
        .unwrap())
}

pub fn part2(pantry: Pantry) -> Result<i64> {
    let mut fresh = pantry.fresh;
    fresh.sort_by_key(|r| (*r.start(), *r.end()));
    let mut fresh: VecDeque<std::ops::RangeInclusive<i64>> = fresh.into();
    let mut merged = vec![];
    while let Some(mut next) = fresh.pop_front() {
        while let Some(head) = fresh.pop_front() {
            if head.start() <= next.end() {
                next = *next.start()..=(*head.end()).max(*next.end());
            } else {
                fresh.push_front(head);
                break;
            }
        }
        merged.push(next)
    }
    Ok(merged.into_iter().map(|r| r.end() - r.start() + 1).sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2025, 5).unwrap()).unwrap()).unwrap(),
        643
    );
    assert_eq!(
        part2(parse(parse::data(2025, 5).unwrap()).unwrap()).unwrap(),
        342018167474526
    );
}
