#![allow(dead_code)]
#![allow(unused_variables)]

use crate::prelude::*;

pub fn parse(fh: File) -> Result<String> {
    Ok(parse::string(fh))
}

pub fn part1(s: String) -> Result<i64> {
    for (i, slice) in s.as_bytes().windows(4).enumerate() {
        if slice.iter().copied().collect::<HashSet<u8>>().len() == 4 {
            return Ok(i64::try_from(i).unwrap() + 4);
        }
    }
    Err(anyhow!("couldn't find marker"))
}

pub fn part2(s: String) -> Result<i64> {
    for (i, slice) in s.as_bytes().windows(14).enumerate() {
        if slice.iter().copied().collect::<HashSet<u8>>().len() == 14 {
            return Ok(i64::try_from(i).unwrap() + 14);
        }
    }
    Err(anyhow!("couldn't find marker"))
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 6).unwrap()).unwrap()).unwrap(),
        1155
    );
    assert_eq!(
        part2(parse(parse::data(2022, 6).unwrap()).unwrap()).unwrap(),
        2789
    );
}
