pub fn part1() -> anyhow::Result<()> {
    let ints = crate::util::read_ints("data/1.txt")?;
    for i in &ints {
        for j in &ints {
            if i + j == 2020 {
                println!("{} * {} = {}", i, j, i * j);
                return Ok(());
            }
        }
    }
    Err(anyhow::anyhow!("no numbers summing to 2020 found"))
}

pub fn part2() -> anyhow::Result<()> {
    let ints = crate::util::read_ints("data/1.txt")?;
    for i in &ints {
        for j in &ints {
            for k in &ints {
                if i + j + k == 2020 {
                    println!("{} * {} * {} = {}", i, j, k, i * j * k);
                    return Ok(());
                }
            }
        }
    }
    Err(anyhow::anyhow!("no numbers summing to 2020 found"))
}
