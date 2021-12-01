pub fn part1() -> anyhow::Result<i64> {
    let ints = data_ints!()?;
    for i in &ints {
        for j in &ints {
            if i + j == 2020 {
                return Ok(i * j);
            }
        }
    }
    Err(anyhow::anyhow!("no numbers summing to 2020 found"))
}

pub fn part2() -> anyhow::Result<i64> {
    let ints = data_ints!()?;
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
