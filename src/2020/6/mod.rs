pub fn part1() -> anyhow::Result<i64> {
    let input = data_str!()?;
    let mut yes = std::collections::HashSet::new();
    let mut total = 0;
    for line in input.lines() {
        if line.is_empty() {
            total += yes.len() as i64;
            yes = std::collections::HashSet::new();
        } else {
            for c in line.chars() {
                yes.insert(c);
            }
        }
    }
    total += yes.len() as i64;
    Ok(total)
}

pub fn part2() -> anyhow::Result<i64> {
    let input = data_str!()?;
    let mut yes = std::collections::HashSet::new();
    for c in 'a'..='z' {
        yes.insert(c);
    }
    let mut total = 0;
    for line in input.lines() {
        if line.is_empty() {
            total += yes.len() as i64;
            yes = std::collections::HashSet::new();
            for c in 'a'..='z' {
                yes.insert(c);
            }
        } else {
            for c in 'a'..='z' {
                if !line.contains(c) {
                    yes.remove(&c);
                }
            }
        }
    }
    total += yes.len() as i64;
    Ok(total)
}

#[test]
fn test() {
    assert_eq!(part1().unwrap(), 6930);
    assert_eq!(part2().unwrap(), 3585);
}
