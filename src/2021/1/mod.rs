pub fn part1() -> anyhow::Result<i64> {
    Ok(data_ints!()?
        .windows(2)
        .map(|a| a[1] - a[0])
        .filter(|x| *x > 0)
        .count()
        .try_into()?)
}

pub fn part2() -> anyhow::Result<i64> {
    Ok(data_ints!()?
        .windows(3)
        .map(|a| a[0] + a[1] + a[2])
        .collect::<Vec<_>>()
        .windows(2)
        .map(|a| a[1] - a[0])
        .filter(|x| *x > 0)
        .count()
        .try_into()?)
}

#[test]
fn test() {
    assert_eq!(part1().unwrap(), 1602);
    assert_eq!(part2().unwrap(), 1633);
}
