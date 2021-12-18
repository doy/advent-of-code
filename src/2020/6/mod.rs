pub fn parse(
    fh: std::fs::File,
) -> anyhow::Result<impl Iterator<Item = String>> {
    Ok(crate::util::parse::lines(fh))
}

pub fn part1(lines: impl Iterator<Item = String>) -> anyhow::Result<i64> {
    let mut yes = std::collections::HashSet::new();
    let mut total = 0;
    for line in lines {
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

pub fn part2(lines: impl Iterator<Item = String>) -> anyhow::Result<i64> {
    let mut yes = std::collections::HashSet::new();
    for c in 'a'..='z' {
        yes.insert(c);
    }
    let mut total = 0;
    for line in lines {
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
    assert_eq!(
        part1(parse(crate::util::data(2020, 6).unwrap()).unwrap()).unwrap(),
        6930
    );
    assert_eq!(
        part2(parse(crate::util::data(2020, 6).unwrap()).unwrap()).unwrap(),
        3585
    );
}
