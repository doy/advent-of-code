#![allow(dead_code)]
#![allow(unused_variables)]

use crate::prelude::*;

pub fn parse(
    fh: File,
) -> Result<impl Iterator<Item = (HashSet<char>, HashSet<char>)>> {
    Ok(parse::lines(fh).map(|line| {
        let (first, second) = line.split_at(line.len() / 2);
        let first: HashSet<char> = first.chars().collect();
        let second: HashSet<char> = second.chars().collect();
        (first, second)
    }))
}

pub fn part1(
    sacks: impl Iterator<Item = (HashSet<char>, HashSet<char>)>,
) -> Result<i64> {
    Ok(sacks
        .map(|(first, second)| {
            i64::from(priority(*first.intersection(&second).next().unwrap()))
        })
        .sum())
}

pub fn part2(
    mut sacks: impl Iterator<Item = (HashSet<char>, HashSet<char>)>,
) -> Result<i64> {
    let mut total = 0;
    while let (Some(first), Some(second), Some(third)) =
        (sacks.next(), sacks.next(), sacks.next())
    {
        let first: HashSet<char> = first.0.union(&first.1).copied().collect();
        let second: HashSet<char> =
            second.0.union(&second.1).copied().collect();
        let third: HashSet<char> = third.0.union(&third.1).copied().collect();
        total += i64::from(priority(
            *(&(&first & &second) & &third).iter().next().unwrap(),
        ));
    }
    Ok(total)
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => u32::from(c) - u32::from('a') + 1,
        'A'..='Z' => u32::from(c) - u32::from('A') + 27,
        _ => unreachable!(),
    }
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 3).unwrap()).unwrap()).unwrap(),
        8493
    );
    assert_eq!(
        part2(parse(parse::data(2022, 3).unwrap()).unwrap()).unwrap(),
        2552
    );
}
