use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Vec<usize>> {
    Ok(parse::split(fh, b',').collect())
}

pub fn part1(crabs: Vec<usize>) -> Result<usize> {
    Ok((0..=crabs.iter().copied().max().unwrap())
        .map(|start| {
            crabs.iter().copied().map(|crab| crab.abs_diff(start)).sum()
        })
        .min()
        .unwrap())
}

pub fn part2(crabs: Vec<usize>) -> Result<usize> {
    Ok((0..=crabs.iter().copied().max().unwrap())
        .map(|start| {
            crabs
                .iter()
                .copied()
                .map(|crab| {
                    let diff = crab.abs_diff(start);
                    diff * (diff + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2021, 7).unwrap()).unwrap()).unwrap(),
        333755
    );
    assert_eq!(
        part2(parse(parse::data(2021, 7).unwrap()).unwrap()).unwrap(),
        94017638
    );
}
