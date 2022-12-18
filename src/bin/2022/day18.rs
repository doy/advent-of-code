#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Vec<(i64, i64, i64)>> {
    Ok(parse::raw_lines(fh)
        .map(|s| {
            let mut parts = s.split(',');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect())
}

pub fn part1(points: Vec<(i64, i64, i64)>) -> Result<i64> {
    let mut total = 0;
    for p in &points {
        let mut area = 6;
        for neighbor in [
            (p.0 - 1, p.1, p.2),
            (p.0 + 1, p.1, p.2),
            (p.0, p.1 - 1, p.2),
            (p.0, p.1 + 1, p.2),
            (p.0, p.1, p.2 - 1),
            (p.0, p.1, p.2 + 1),
        ] {
            if points.contains(&neighbor) {
                area -= 1;
            }
        }
        total += area;
    }
    Ok(total)
}

pub fn part2(points: Vec<(i64, i64, i64)>) -> Result<i64> {
    let mut all_neighbors = HashSet::new();
    for p in &points {
        for neighbor in [
            (p.0 - 1, p.1, p.2),
            (p.0 + 1, p.1, p.2),
            (p.0, p.1 - 1, p.2),
            (p.0, p.1 + 1, p.2),
            (p.0, p.1, p.2 - 1),
            (p.0, p.1, p.2 + 1),
        ] {
            if !points.contains(&neighbor) {
                all_neighbors.insert(neighbor);
            }
        }
    }

    let bounds = (
        (points.iter().map(|p| p.0).min().unwrap() - 1)
            ..=(points.iter().map(|p| p.0).max().unwrap() + 1),
        (points.iter().map(|p| p.1).min().unwrap() - 1)
            ..=(points.iter().map(|p| p.1).max().unwrap() + 1),
        (points.iter().map(|p| p.2).min().unwrap() - 1)
            ..=(points.iter().map(|p| p.2).max().unwrap() + 1),
    );
    let mut visited = HashSet::new();
    let mut to_visit =
        vec![(*bounds.0.start(), *bounds.1.start(), *bounds.2.start())];
    while let Some(p) = to_visit.pop() {
        visited.insert(p);
        let neighbors = [
            (p.0 - 1, p.1, p.2),
            (p.0 + 1, p.1, p.2),
            (p.0, p.1 - 1, p.2),
            (p.0, p.1 + 1, p.2),
            (p.0, p.1, p.2 - 1),
            (p.0, p.1, p.2 + 1),
        ];
        to_visit.extend(
            neighbors
                .iter()
                .filter(|p| in_bounds(**p, &bounds))
                .filter(|p| !visited.contains(p))
                .filter(|p| !points.contains(p)),
        )
    }

    let mut total = 0;
    for p in &points {
        let mut area = 6;
        for neighbor in [
            (p.0 - 1, p.1, p.2),
            (p.0 + 1, p.1, p.2),
            (p.0, p.1 - 1, p.2),
            (p.0, p.1 + 1, p.2),
            (p.0, p.1, p.2 - 1),
            (p.0, p.1, p.2 + 1),
        ] {
            if points.contains(&neighbor) || !visited.contains(&neighbor) {
                area -= 1;
            }
        }
        total += area;
    }
    Ok(total)
}

fn in_bounds(
    p: (i64, i64, i64),
    bounds: &(
        std::ops::RangeInclusive<i64>,
        std::ops::RangeInclusive<i64>,
        std::ops::RangeInclusive<i64>,
    ),
) -> bool {
    bounds.0.contains(&p.0)
        && bounds.1.contains(&p.1)
        && bounds.2.contains(&p.2)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 18).unwrap()).unwrap()).unwrap(),
        4608
    );
    assert_eq!(
        part2(parse(parse::data(2022, 18).unwrap()).unwrap()).unwrap(),
        2652
    );
}
