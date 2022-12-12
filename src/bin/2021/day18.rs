use advent_of_code::prelude::*;

#[derive(Clone)]
pub enum Number {
    Value(u8),
    Pair(Box<Number>, Box<Number>),
}

impl std::str::FromStr for Number {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(s))
    }
}

impl Number {
    fn parse(s: &str) -> Self {
        if s.starts_with('[') {
            Self::parse_pair(s)
        } else {
            Self::parse_number(s)
        }
    }

    fn parse_pair(s: &str) -> Self {
        let mut depth = 1;
        let mut start = 1;
        let mut children = vec![];
        for (i, c) in s.chars().enumerate().skip(1) {
            match c {
                '[' => {
                    depth += 1;
                }
                ',' => {
                    if depth == 1 {
                        children.push(Self::parse(&s[start..i]));
                        start = i + 1;
                    }
                }
                ']' => {
                    depth -= 1;
                }
                _ => {}
            }
            if depth == 0 {
                children.push(Self::parse(&s[start..i]));
                break;
            }
        }
        let right = children.pop().unwrap();
        let left = children.pop().unwrap();
        Self::Pair(Box::new(left), Box::new(right))
    }

    fn parse_number(s: &str) -> Self {
        Self::Value(s.as_bytes()[0] - b'0')
    }

    fn reduce(&mut self) {
        loop {
            if !self.visit_explode() && !self.visit_split() {
                break;
            }
        }
    }

    fn visit_explode(&mut self) -> bool {
        let mut idx = 0;
        if let Some((explode_idx, (left, right))) =
            self.find_to_explode(0, &mut idx)
        {
            idx = 0;
            self.explode(explode_idx, left, right, &mut idx);
            true
        } else {
            false
        }
    }

    fn find_to_explode(
        &mut self,
        depth: usize,
        idx: &mut usize,
    ) -> Option<(usize, (u8, u8))> {
        match self {
            Self::Value(_) => {
                *idx += 1;
                None
            }
            Self::Pair(left, right) => {
                if depth == 4 {
                    let left = if let Self::Value(n) = **left {
                        n
                    } else {
                        panic!("unexpected pair")
                    };
                    let right = if let Self::Value(n) = **right {
                        n
                    } else {
                        panic!("unexpected pair")
                    };
                    *self = Self::Value(0);
                    Some((*idx, (left, right)))
                } else {
                    left.find_to_explode(depth + 1, idx)
                        .or_else(|| right.find_to_explode(depth + 1, idx))
                }
            }
        }
    }

    fn explode(
        &mut self,
        explode_idx: usize,
        left_val: u8,
        right_val: u8,
        idx: &mut usize,
    ) {
        match self {
            Self::Value(ref mut n) => {
                if *idx + 1 == explode_idx {
                    *n += left_val;
                } else if *idx == explode_idx + 1 {
                    *n += right_val;
                }
                *idx += 1;
            }
            Self::Pair(left, right) => {
                left.explode(explode_idx, left_val, right_val, idx);
                right.explode(explode_idx, left_val, right_val, idx);
            }
        }
    }

    fn visit_split(&mut self) -> bool {
        match self {
            Self::Value(n) => {
                if *n >= 10 {
                    *self = Self::Pair(
                        Box::new(Self::Value(*n / 2)),
                        Box::new(Self::Value(*n - *n / 2)),
                    );
                    true
                } else {
                    false
                }
            }
            Self::Pair(left, right) => {
                left.visit_split() || right.visit_split()
            }
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            Self::Value(n) => u64::from(*n),
            Self::Pair(left, right) => {
                left.magnitude() * 3 + right.magnitude() * 2
            }
        }
    }
}

impl std::ops::Add for Number {
    type Output = Number;
    fn add(self, other: Number) -> Self::Output {
        let mut ret = Number::Pair(Box::new(self), Box::new(other));
        ret.reduce();
        ret
    }
}

impl std::ops::Add for &Number {
    type Output = Number;
    fn add(self, other: &Number) -> Self::Output {
        let mut ret =
            Number::Pair(Box::new(self.clone()), Box::new(other.clone()));
        ret.reduce();
        ret
    }
}

impl std::ops::AddAssign for Number {
    fn add_assign(&mut self, other: Number) {
        *self = self.clone() + other;
    }
}

impl std::iter::Sum for Number {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        if let Some(first) = iter.next() {
            let mut sum = first;
            for num in iter {
                sum += num;
            }
            sum
        } else {
            Number::Value(0)
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        match self {
            Self::Value(n) => write!(f, "{}", n),
            Self::Pair(left, right) => {
                write!(f, "[{},{}]", left, right)
            }
        }
    }
}

pub fn parse(fh: File) -> Result<impl Iterator<Item = Number>> {
    Ok(parse::lines(fh))
}

pub fn part1(numbers: impl Iterator<Item = Number>) -> Result<u64> {
    let sum: Number = numbers.sum();
    Ok(sum.magnitude())
}

pub fn part2(numbers: impl Iterator<Item = Number>) -> Result<u64> {
    let nums: Vec<_> = numbers.collect();
    let mut max = 0;
    for (i, n1) in nums.iter().enumerate() {
        for (j, n2) in nums.iter().enumerate() {
            if i == j {
                continue;
            }
            let magnitude = (n1 + n2).magnitude();
            if magnitude > max {
                max = magnitude;
            }
        }
    }
    Ok(max)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2021, 18).unwrap()).unwrap()).unwrap(),
        3806
    );
    assert_eq!(
        part2(parse(parse::data(2021, 18).unwrap()).unwrap()).unwrap(),
        4727
    );
}
