pub fn parse(fh: std::fs::File) -> anyhow::Result<Vec<i64>> {
    Ok(
        crate::util::parse::ints(crate::util::parse::split(fh, b','))
            .collect(),
    )
}

pub fn part1(crabs: Vec<i64>) -> anyhow::Result<i64> {
    Ok((0..=crabs.iter().copied().max().unwrap())
        .map(|start| {
            crabs.iter().copied().map(|crab| (crab - start).abs()).sum()
        })
        .min()
        .unwrap())
}

pub fn part2(crabs: Vec<i64>) -> anyhow::Result<i64> {
    Ok((0..=crabs.iter().copied().max().unwrap())
        .map(|start| {
            crabs
                .iter()
                .copied()
                .map(|crab| {
                    let diff = (crab - start).abs();
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
        part1(parse(crate::util::data(2021, 7).unwrap()).unwrap()).unwrap(),
        333755
    );
    assert_eq!(
        part2(parse(crate::util::data(2021, 7).unwrap()).unwrap()).unwrap(),
        94017638
    );
}
