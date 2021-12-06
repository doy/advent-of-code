pub fn part1() -> anyhow::Result<i64> {
    let mut fishes: Vec<u8> = data_lines!()?
        .next()
        .unwrap()?
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    for _ in 0..80 {
        let mut new = 0;
        for fish in fishes.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new += 1;
            } else {
                *fish -= 1;
            }
        }
        fishes.resize(fishes.len() + new, 8);
    }
    Ok(fishes.len().try_into()?)
}

pub fn part2() -> anyhow::Result<i64> {
    let fishes: Vec<u8> = data_lines!()?
        .next()
        .unwrap()?
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;
    let mut by_age = std::collections::VecDeque::new();
    by_age.resize(9, 0);
    for fish in fishes {
        by_age[fish as usize] += 1;
    }
    for _ in 0..256 {
        let new = by_age.pop_front().unwrap();
        by_age[6] += new;
        by_age.push_back(new);
    }
    Ok(by_age.iter().sum())
}
