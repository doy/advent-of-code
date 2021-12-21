use crate::prelude::*;

pub fn parse(fh: File) -> Result<Vec<i64>> {
    Ok(parse::ints(parse::lines(fh)).collect())
}

pub fn part1(ints: Vec<i64>) -> Result<i64> {
    for i in &ints {
        for j in &ints {
            if i + j == 2020 {
                return Ok(i * j);
            }
        }
    }
    Err(anyhow!("no numbers summing to 2020 found"))
}

pub fn part2(ints: Vec<i64>) -> Result<i64> {
    for i in &ints {
        for j in &ints {
            for k in &ints {
                if i + j + k == 2020 {
                    return Ok(i * j * k);
                }
            }
        }
    }
    Err(anyhow!("no numbers summing to 2020 found"))
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2020, 1).unwrap()).unwrap()).unwrap(),
        445536
    );
    assert_eq!(
        part2(parse(parse::data(2020, 1).unwrap()).unwrap()).unwrap(),
        138688160
    );
}
