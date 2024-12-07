use advent_of_code::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Mul,
    Cat,
}

#[derive(Debug)]
pub struct Problem {
    total: i64,
    ints: Vec<i64>,
}

impl std::str::FromStr for Problem {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let total = parts.next().unwrap().parse().unwrap();
        let ints = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Ok(Self { total, ints })
    }
}

impl Problem {
    fn solve1(&self) -> Option<Vec<Op>> {
        let ints: Vec<i64> = self.ints.iter().copied().rev().collect();
        self.solve1_rec(&ints)
            .map(|ops| ops.into_iter().rev().collect())
    }

    fn solve1_rec(&self, ints: &[i64]) -> Option<Vec<Op>> {
        if ints.len() == 1 {
            if ints[0] == self.total {
                return Some(vec![]);
            } else {
                return None;
            }
        }
        if ints[ints.len() - 1] > self.total {
            return None;
        }
        let mut ints = ints.to_vec();
        let a = ints.pop().unwrap();
        let b = ints.pop().unwrap();
        ints.push(a + b);
        if let Some(mut ops) = self.solve1_rec(&ints) {
            ops.push(Op::Add);
            return Some(ops);
        }
        ints.pop().unwrap();
        ints.push(a * b);
        if let Some(mut ops) = self.solve1_rec(&ints) {
            ops.push(Op::Mul);
            return Some(ops);
        }
        None
    }

    fn solve2(&self) -> Option<Vec<Op>> {
        let ints: Vec<i64> = self.ints.iter().copied().rev().collect();
        self.solve2_rec(&ints)
            .map(|ops| ops.into_iter().rev().collect())
    }

    fn solve2_rec(&self, ints: &[i64]) -> Option<Vec<Op>> {
        if ints.len() == 1 {
            if ints[0] == self.total {
                return Some(vec![]);
            } else {
                return None;
            }
        }
        if ints[ints.len() - 1] > self.total {
            return None;
        }
        let mut ints = ints.to_vec();
        let a = ints.pop().unwrap();
        let b = ints.pop().unwrap();
        ints.push(a + b);
        if let Some(mut ops) = self.solve2_rec(&ints) {
            ops.push(Op::Add);
            return Some(ops);
        }
        ints.pop().unwrap();
        ints.push(a * b);
        if let Some(mut ops) = self.solve2_rec(&ints) {
            ops.push(Op::Mul);
            return Some(ops);
        }
        ints.pop().unwrap();
        ints.push(b + a * 10i64.pow(b.ilog10() + 1));
        if let Some(mut ops) = self.solve2_rec(&ints) {
            ops.push(Op::Cat);
            return Some(ops);
        }
        None
    }
}

pub fn parse(fh: File) -> Result<Vec<Problem>> {
    Ok(parse::lines(fh).collect())
}

pub fn part1(problems: Vec<Problem>) -> Result<i64> {
    Ok(problems
        .par_iter()
        .map(|problem| {
            if problem.solve1().is_some() {
                problem.total
            } else {
                0
            }
        })
        .sum())
}

pub fn part2(problems: Vec<Problem>) -> Result<i64> {
    Ok(problems
        .par_iter()
        .map(|problem| {
            if problem.solve2().is_some() {
                problem.total
            } else {
                0
            }
        })
        .sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 7).unwrap()).unwrap()).unwrap(),
        66343330034722
    );
    assert_eq!(
        part2(parse(parse::data(2024, 7).unwrap()).unwrap()).unwrap(),
        637696070419031
    );
}
