use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<impl Iterator<Item = String>> {
    Ok(parse::raw_lines(fh))
}

pub fn part1(lines: impl Iterator<Item = String>) -> Result<usize> {
    let mut yes = HashSet::new();
    let mut total = 0;
    for line in lines {
        if line.is_empty() {
            total += yes.len();
            yes = HashSet::new();
        } else {
            for c in line.chars() {
                yes.insert(c);
            }
        }
    }
    total += yes.len();
    Ok(total)
}

pub fn part2(lines: impl Iterator<Item = String>) -> Result<usize> {
    let mut yes = HashSet::new();
    for c in 'a'..='z' {
        yes.insert(c);
    }
    let mut total = 0;
    for line in lines {
        if line.is_empty() {
            total += yes.len();
            yes = HashSet::new();
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
    total += yes.len();
    Ok(total)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2020, 6).unwrap()).unwrap()).unwrap(),
        6930
    );
    assert_eq!(
        part2(parse(parse::data(2020, 6).unwrap()).unwrap()).unwrap(),
        3585
    );
}
