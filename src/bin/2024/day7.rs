use advent_of_code::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Mul,
    Cat,
}

impl Op {
    fn run(self, a: i64, b: i64) -> i64 {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
            Self::Cat => b + a * 10i64.pow(advent_of_code::num::digits(b)),
        }
    }
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
    fn solve(&self, ops: &[Op]) -> Option<Vec<Op>> {
        let ints: Vec<i64> = self.ints.iter().copied().rev().collect();
        self.solve_rec(&ints, ops)
            .map(|ops| ops.into_iter().rev().collect())
    }

    fn solve_rec(&self, ints: &[i64], used_ops: &[Op]) -> Option<Vec<Op>> {
        if ints.len() == 1 {
            return (ints[0] == self.total).then(Vec::new);
        }

        let mut ints = ints.to_vec();
        let a = ints.pop().unwrap();
        if a > self.total {
            return None;
        }
        let b = ints.pop().unwrap();
        ints.push(0);
        let idx = ints.len() - 1;

        for op in used_ops {
            ints[idx] = op.run(a, b);
            if let Some(mut ops) = self.solve_rec(&ints, used_ops) {
                ops.push(*op);
                return Some(ops);
            }
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
            if problem.solve(&[Op::Add, Op::Mul]).is_some() {
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
            if problem.solve(&[Op::Add, Op::Mul, Op::Cat]).is_some() {
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
