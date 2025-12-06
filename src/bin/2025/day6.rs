use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<String> {
    Ok(parse::string(fh))
}

pub fn part1(input: String) -> Result<i64> {
    let mut rows: Vec<Vec<_>> = vec![];
    for line in input.lines() {
        let nums: Vec<_> = line.split_whitespace().collect();
        if ["+", "*"].contains(&nums[0]) {
            let ops = nums;
            let mut cols = vec![vec![]; rows[0].len()];
            for row in rows {
                for (i, num) in row.into_iter().enumerate() {
                    cols[i].push(num);
                }
            }
            return Ok(cols
                .into_iter()
                .enumerate()
                .map(|(i, nums)| match ops[i] {
                    "+" => nums.iter().sum::<i64>(),
                    "*" => nums.iter().product::<i64>(),
                    _ => unreachable!(),
                })
                .sum());
        } else {
            rows.push(nums.into_iter().map(|s| s.parse().unwrap()).collect());
        }
    }
    unreachable!()
}

pub fn part2(input: String) -> Result<i64> {
    let mut input_cols = vec![];
    for line in input.lines() {
        input_cols.resize_with(line.len(), Vec::new);
        for (i, c) in line.bytes().rev().enumerate() {
            input_cols[i].push(c);
        }
    }

    let mut total = 0;
    let mut nums: Vec<i64> = vec![];
    for col in input_cols {
        let s = String::from_utf8(col).unwrap();
        let s = s.trim();
        if s.is_empty() {
            continue;
        }
        if let Some(n) = s.strip_suffix(['+', '*']) {
            nums.push(n.trim().parse().unwrap());
            if s.ends_with('+') {
                total += nums.drain(..).sum::<i64>();
            } else {
                total += nums.drain(..).product::<i64>();
            }
        } else {
            nums.push(s.parse().unwrap());
        }
    }
    Ok(total)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2025, 6).unwrap()).unwrap()).unwrap(),
        3525371263915
    );
    assert_eq!(
        part2(parse(parse::data(2025, 6).unwrap()).unwrap()).unwrap(),
        6846480843636
    );
}
