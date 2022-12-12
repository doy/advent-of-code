use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<impl Iterator<Item = String>> {
    Ok(parse::raw_lines(fh))
}

pub fn part1(lines: impl Iterator<Item = String>) -> Result<u64> {
    let (total_lines, by_pos) = pos_counts(lines)?;
    let gamma: String = by_pos
        .iter()
        .map(|pos| if pos * 2 >= total_lines { '1' } else { '0' })
        .collect();
    let epsilon: String = by_pos
        .iter()
        .map(|pos| if pos * 2 >= total_lines { '0' } else { '1' })
        .collect();
    Ok(bin_str_to_int(&gamma) * bin_str_to_int(&epsilon))
}

pub fn part2(lines: impl Iterator<Item = String>) -> Result<u64> {
    let mut oxygen: Vec<_> = lines.collect();
    let mut co2 = oxygen.clone();

    for i in 0..oxygen[0].len() {
        if oxygen.len() == 1 {
            break;
        }
        let (total_lines, by_pos) = pos_counts(oxygen.iter().cloned())?;
        let keep = if by_pos[i] * 2 >= total_lines {
            '1'
        } else {
            '0'
        };
        let new_oxygen = oxygen
            .iter()
            .filter(|l| l.chars().nth(i).unwrap() == keep)
            .cloned()
            .collect();
        oxygen = new_oxygen;
    }

    for i in 0..co2[0].len() {
        if co2.len() == 1 {
            break;
        }
        let (total_lines, by_pos) = pos_counts(co2.iter().cloned())?;
        let keep = if by_pos[i] * 2 >= total_lines {
            '0'
        } else {
            '1'
        };
        let new_co2 = co2
            .iter()
            .filter(|l| l.chars().nth(i).unwrap() == keep)
            .cloned()
            .collect();
        co2 = new_co2;
    }

    Ok(bin_str_to_int(&oxygen[0]) * bin_str_to_int(&co2[0]))
}

fn pos_counts(
    lines: impl Iterator<Item = String>,
) -> Result<(i64, Vec<i64>)> {
    let mut by_pos = vec![];
    let mut total_lines = 0;
    for line in lines {
        total_lines += 1;
        if by_pos.is_empty() {
            by_pos.resize(line.len(), 0);
        }
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                by_pos[i] += 1;
            }
        }
    }
    Ok((total_lines, by_pos))
}

fn bin_str_to_int(s: &str) -> u64 {
    let mut ret = 0;
    for (i, c) in s.chars().rev().enumerate() {
        if c == '1' {
            ret += 2u64.pow(i.try_into().unwrap());
        }
    }
    ret
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2021, 3).unwrap()).unwrap()).unwrap(),
        3009600
    );
    assert_eq!(
        part2(parse(parse::data(2021, 3).unwrap()).unwrap()).unwrap(),
        6940518
    );
}
