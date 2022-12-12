use advent_of_code::prelude::*;

const WINDOW: usize = 25;

pub fn parse(fh: File) -> Result<Vec<u64>> {
    Ok(parse::lines(fh).collect())
}

pub fn part1(list: Vec<u64>) -> Result<u64> {
    for i in 0..(list.len() - WINDOW) {
        let set = &list[i..i + WINDOW];
        let n = list[i + WINDOW];
        if !valid(set, n) {
            return Ok(n);
        }
    }

    Err(anyhow!("failed to find invalid number"))
}

pub fn part2(list: Vec<u64>) -> Result<u64> {
    let mut invalid = None;
    for i in 0..(list.len() - WINDOW) {
        let set = &list[i..i + WINDOW];
        let n = list[i + WINDOW];
        if !valid(set, n) {
            invalid = Some(n);
        }
    }
    if invalid.is_none() {
        return Err(anyhow!("failed to find invalid number"));
    }
    let invalid = invalid.unwrap();

    for i in 0..list.len() {
        for j in i..list.len() {
            let seq = &list[i..=j];
            if invalid == seq.iter().sum::<u64>() {
                return Ok(seq.iter().copied().min().unwrap()
                    + seq.iter().copied().max().unwrap());
            }
        }
    }

    Err(anyhow!("failed to find sequence summing to invalid number"))
}

fn valid(set: &[u64], n: u64) -> bool {
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

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2020, 9).unwrap()).unwrap()).unwrap(),
        373803594
    );
    assert_eq!(
        part2(parse(parse::data(2020, 9).unwrap()).unwrap()).unwrap(),
        51152360
    );
}
