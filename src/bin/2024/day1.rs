use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<(Vec<i64>, Vec<i64>)> {
    Ok(parse::raw_lines(fh)
        .map(parse::fields2)
        .unzip::<i64, i64, _, _>())
}

pub fn part1((mut list1, mut list2): (Vec<i64>, Vec<i64>)) -> Result<i64> {
    list1.sort_unstable();
    list2.sort_unstable();
    let mut total = 0;
    for (i, j) in list1.iter().copied().zip(list2.iter().copied()) {
        total += i64::try_from(i.abs_diff(j)).unwrap();
    }
    Ok(total)
}

pub fn part2((list1, list2): (Vec<i64>, Vec<i64>)) -> Result<i64> {
    let mut total = 0;
    for i in list1 {
        total += i * i64::try_from(
            list2.iter().copied().filter(|j| i == *j).count(),
        )
        .unwrap();
    }
    Ok(total)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 1).unwrap()).unwrap()).unwrap(),
        2057374
    );
    assert_eq!(
        part2(parse(parse::data(2024, 1).unwrap()).unwrap()).unwrap(),
        23177084
    );
}
