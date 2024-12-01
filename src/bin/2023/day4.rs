use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Vec<(HashSet<i64>, HashSet<i64>)>> {
    Ok(parse::raw_lines(fh)
        .map(|line| {
            let line = line.split(": ").nth(1).unwrap();
            let mut parts = line.split(" | ");
            let winning = parts.next().unwrap();
            let ours = parts.next().unwrap();
            (
                winning
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
                ours.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            )
        })
        .collect())
}

pub fn part1(cards: Vec<(HashSet<i64>, HashSet<i64>)>) -> Result<i64> {
    let mut total = 0;
    for (winning, ours) in cards {
        let matches = (&ours & &winning).len();
        if matches > 0 {
            total += 2i64.pow((matches - 1).try_into().unwrap());
        }
    }
    Ok(total)
}

pub fn part2(cards: Vec<(HashSet<i64>, HashSet<i64>)>) -> Result<i64> {
    let mut counts = vec![1; cards.len()];
    for (i, (winning, ours)) in cards.into_iter().enumerate() {
        let matches = (&ours & &winning).len();
        for prize in 0..matches {
            counts[i + 1 + prize] += counts[i];
        }
    }
    Ok(counts.iter().sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 4).unwrap()).unwrap()).unwrap(),
        25183
    );
    assert_eq!(
        part2(parse(parse::data(2023, 4).unwrap()).unwrap()).unwrap(),
        5667240
    );
}
