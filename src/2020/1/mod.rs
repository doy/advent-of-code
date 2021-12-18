pub fn parse(fh: std::fs::File) -> anyhow::Result<Vec<i64>> {
    Ok(crate::util::parse::ints(crate::util::parse::lines(fh)).collect())
}

pub fn part1(ints: Vec<i64>) -> anyhow::Result<i64> {
    for i in &ints {
        for j in &ints {
            if i + j == 2020 {
                return Ok(i * j);
            }
        }
    }
    Err(anyhow::anyhow!("no numbers summing to 2020 found"))
}

pub fn part2(ints: Vec<i64>) -> anyhow::Result<i64> {
    for i in &ints {
        for j in &ints {
            for k in &ints {
                if i + j + k == 2020 {
                    return Ok(i * j * k);
                }
            }
        }
    }
    Err(anyhow::anyhow!("no numbers summing to 2020 found"))
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(crate::util::data(2020, 1).unwrap()).unwrap()).unwrap(),
        445536
    );
    assert_eq!(
        part2(parse(crate::util::data(2020, 1).unwrap()).unwrap()).unwrap(),
        138688160
    );
}
