pub fn part1() -> anyhow::Result<i64> {
    let crabs: Vec<i64> = data_lines!()?
        .next()
        .unwrap()?
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    Ok((0..=crabs.iter().copied().max().unwrap())
        .map(|start| {
            crabs.iter().copied().map(|crab| (crab - start).abs()).sum()
        })
        .min()
        .unwrap())
}

pub fn part2() -> anyhow::Result<i64> {
    let crabs: Vec<i64> = data_lines!()?
        .next()
        .unwrap()?
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

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
    assert_eq!(part1().unwrap(), 333755);
    assert_eq!(part2().unwrap(), 94017638);
}
