pub fn part1() -> anyhow::Result<i64> {
    const WINDOW: usize = 25;

    let list = data_ints!()?;
    for i in 0..(list.len() - WINDOW) {
        let set = &list[i..i + WINDOW];
        let n = list[i + WINDOW];
        if !valid(set, n) {
            return Ok(n);
        }
    }

    Err(anyhow::anyhow!("failed to find invalid number"))
}

pub fn part2() -> anyhow::Result<i64> {
    const WINDOW: usize = 25;

    let list = data_ints!()?;
    let mut invalid = None;
    for i in 0..(list.len() - WINDOW) {
        let set = &list[i..i + WINDOW];
        let n = list[i + WINDOW];
        if !valid(set, n) {
            invalid = Some(n);
        }
    }
    if invalid.is_none() {
        return Err(anyhow::anyhow!("failed to find invalid number"));
    }
    let invalid = invalid.unwrap();

    for i in 0..list.len() {
        for j in i..list.len() {
            let seq = &list[i..=j];
            if invalid == seq.iter().sum() {
                return Ok(seq.iter().copied().min().unwrap()
                    + seq.iter().copied().max().unwrap());
            }
        }
    }

    Err(anyhow::anyhow!(
        "failed to find sequence summing to invalid number"
    ))
}

fn valid(set: &[i64], n: i64) -> bool {
    for i in 0..set.len() {
        for j in 0..set.len() {
            if i == j {
                continue;
            }
            let i = set[i];
            let j = set[j];
            if i + j == n {
                return true;
            }
        }
    }
    false
}
