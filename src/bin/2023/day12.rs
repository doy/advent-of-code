#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Condition {
    Good,
    Bad,
    Unknown,
}

impl TryFrom<char> for Condition {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            '.' => Condition::Good,
            '#' => Condition::Bad,
            '?' => Condition::Unknown,
            _ => bail!("unknown condition {value}"),
        })
    }
}

pub struct Record {
    condition: Vec<Condition>,
    lengths: Vec<usize>,
}

impl Record {
    fn arrangements(&self) -> usize {
        arrangements(&self.condition, &self.lengths)
    }

    fn unfold(self) -> Self {
        let mut condition = vec![];
        condition.extend_from_slice(&self.condition);
        for _ in 0..4 {
            condition.push(Condition::Unknown);
            condition.extend_from_slice(&self.condition);
        }
        Self {
            condition,
            lengths: self.lengths.repeat(5),
        }
    }
}

fn arrangements(conditions: &[Condition], chunks: &[usize]) -> usize {
    let mut memo = HashMap::new();
    arrangements_memo(conditions, chunks, &mut memo)
}

fn arrangements_memo<'a, 'b>(
    conditions: &'a [Condition],
    chunks: &'b [usize],
    memo: &mut HashMap<(&'a [Condition], &'b [usize]), usize>,
) -> usize {
    if let Some(count) = memo.get(&(conditions, chunks)) {
        return *count;
    }
    let count = _arrangements(conditions, chunks, memo);
    memo.insert((conditions, chunks), count);
    count
}

fn _arrangements<'a, 'b>(
    conditions: &'a [Condition],
    chunks: &'b [usize],
    memo: &mut HashMap<(&'a [Condition], &'b [usize]), usize>,
) -> usize {
    let good_prefix = conditions
        .iter()
        .copied()
        .take_while(|condition| *condition == Condition::Good)
        .count();
    if good_prefix > 0 {
        return arrangements_memo(&conditions[good_prefix..], chunks, memo);
    }

    if conditions.is_empty() {
        if chunks.is_empty() {
            return 1;
        } else {
            return 0;
        }
    } else if chunks.is_empty() {
        if conditions.contains(&Condition::Bad) {
            return 0;
        } else {
            return 1;
        }
    } else if chunks.iter().sum::<usize>() + chunks.len() - 1
        > conditions.len()
    {
        return 0;
    }

    let next = conditions
        .iter()
        .copied()
        .take_while(|condition| *condition != Condition::Good)
        .take(chunks[0])
        .count();
    if next < chunks[0] {
        if conditions
            .iter()
            .copied()
            .take(next)
            .all(|condition| condition == Condition::Unknown)
        {
            return arrangements_memo(&conditions[next..], chunks, memo);
        } else {
            return 0;
        }
    }

    let mut total = 0;
    if conditions[0] == Condition::Unknown {
        total += arrangements_memo(&conditions[1..], chunks, memo);
    }
    if next == conditions.len() || conditions[next] != Condition::Bad {
        total += arrangements_memo(
            &conditions[(next + 1).min(conditions.len())..],
            &chunks[1..],
            memo,
        );
    }

    total
}

impl std::str::FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        Ok(Record {
            condition: parts
                .next()
                .unwrap()
                .chars()
                .map(|c| c.try_into().unwrap())
                .collect(),
            lengths: parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect(),
        })
    }
}

pub fn parse(fh: File) -> Result<Vec<Record>> {
    Ok(parse::lines(fh).collect())
}

pub fn part1(records: Vec<Record>) -> Result<i64> {
    Ok(records
        .into_iter()
        .map(|record| record.arrangements())
        .sum::<usize>()
        .try_into()
        .unwrap())
}

pub fn part2(records: Vec<Record>) -> Result<i64> {
    Ok(records
        .into_iter()
        .map(|record| record.unfold().arrangements())
        .sum::<usize>()
        .try_into()
        .unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 12).unwrap()).unwrap()).unwrap(),
        7407
    );
    assert_eq!(
        part2(parse(parse::data(2023, 12).unwrap()).unwrap()).unwrap(),
        30568243604962
    );
}
