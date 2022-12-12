use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Vec<usize>> {
    Ok(parse::split(fh, b',').collect())
}

pub fn part1(mut fishes: Vec<usize>) -> Result<usize> {
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
    Ok(fishes.len())
}

pub fn part2(fishes: Vec<usize>) -> Result<usize> {
    let mut by_age = VecDeque::new();
    by_age.resize(9, 0);
    for fish in fishes {
        by_age[fish] += 1;
    }
    for _ in 0..256 {
        let new = by_age.pop_front().unwrap();
        by_age[6] += new;
        by_age.push_back(new);
    }
    Ok(by_age.iter().sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2021, 6).unwrap()).unwrap()).unwrap(),
        379114
    );
    assert_eq!(
        part2(parse(parse::data(2021, 6).unwrap()).unwrap()).unwrap(),
        1702631502303
    );
}
