use advent_of_code::prelude::*;

fn is_safe(report: &[i64]) -> bool {
    if !report.is_sorted() && !report.is_sorted_by(|a, b| a > b) {
        return false;
    }
    for pair in report.windows(2) {
        let diff = pair[0].abs_diff(pair[1]);
        if !(1..=3).contains(&diff) {
            return false;
        }
    }
    true
}

pub fn parse(fh: File) -> Result<Vec<Vec<i64>>> {
    Ok(parse::lines(fh)
        .map(|line: String| {
            line.split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect()
        })
        .collect())
}

pub fn part1(reports: Vec<Vec<i64>>) -> Result<i64> {
    Ok(reports
        .into_iter()
        .filter(|report| is_safe(report))
        .count()
        .try_into()
        .unwrap())
}

pub fn part2(reports: Vec<Vec<i64>>) -> Result<i64> {
    Ok(reports
        .into_iter()
        .filter(|report| {
            (0..report.len()).any(|i| {
                let mut report = report.clone();
                report.remove(i);
                is_safe(&report)
            })
        })
        .count()
        .try_into()
        .unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 2).unwrap()).unwrap()).unwrap(),
        242
    );
    assert_eq!(
        part2(parse(parse::data(2024, 2).unwrap()).unwrap()).unwrap(),
        311
    );
}
