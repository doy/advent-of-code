use crate::prelude::*;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Range3D {
    x: std::ops::RangeInclusive<i64>,
    y: std::ops::RangeInclusive<i64>,
    z: std::ops::RangeInclusive<i64>,
}

impl Range3D {
    fn new(
        x: std::ops::RangeInclusive<i64>,
        y: std::ops::RangeInclusive<i64>,
        z: std::ops::RangeInclusive<i64>,
    ) -> Self {
        Self { x, y, z }
    }

    fn contains(&self, point: &Point3D) -> bool {
        self.x.contains(&point.x)
            && self.y.contains(&point.y)
            && self.z.contains(&point.z)
    }
}

#[derive(Debug, Clone)]
struct Rule {
    on: bool,
    range: Range3D,
}

impl Rule {
    fn parse(line: &str) -> Self {
        let captures = regex_captures!(
            r"^(\w+) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$",
            line
        )
        .unwrap();
        Self {
            on: &captures[1] == "on",
            range: Range3D::new(
                captures[2].parse().unwrap()..=captures[3].parse().unwrap(),
                captures[4].parse().unwrap()..=captures[5].parse().unwrap(),
                captures[6].parse().unwrap()..=captures[7].parse().unwrap(),
            ),
        }
    }

    fn contains(&self, point: &Point3D) -> Option<bool> {
        if self.range.contains(point) {
            Some(self.on)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Reactor {
    rules: Vec<Rule>,
}

impl Reactor {
    fn parse(lines: impl Iterator<Item = String>) -> Self {
        Self {
            rules: lines.map(|line| Rule::parse(&line)).collect(),
        }
    }

    fn on(&self, point: &Point3D) -> bool {
        for rule in self.rules.iter().rev() {
            if let Some(on) = rule.contains(point) {
                return on;
            }
        }
        false
    }
}

pub fn parse(fh: File) -> Result<Reactor> {
    Ok(Reactor::parse(parse::lines(fh)))
}

pub fn part1(reactor: Reactor) -> Result<i64> {
    let mut total = 0;
    for x in -50..=50 {
        for y in -50..=50 {
            for z in -50..=50 {
                if reactor.on(&Point3D::new(x, y, z)) {
                    total += 1;
                }
            }
        }
    }
    Ok(total)
}

pub fn part2(reactor: Reactor) -> Result<i64> {
    let mut x = vec![];
    let mut y = vec![];
    let mut z = vec![];
    for rule in &reactor.rules {
        x.push(*rule.range.x.start());
        x.push(rule.range.x.end() + 1);
        y.push(*rule.range.y.start());
        y.push(rule.range.y.end() + 1);
        z.push(*rule.range.z.start());
        z.push(rule.range.z.end() + 1);
    }
    x.sort_unstable();
    y.sort_unstable();
    z.sort_unstable();

    let mut total = 0;
    for i in 0..(x.len() - 1) {
        for j in 0..(y.len() - 1) {
            for k in 0..(z.len() - 1) {
                if reactor.on(&Point3D::new(x[i], y[j], z[k])) {
                    total += (x[i + 1] - x[i])
                        * (y[j + 1] - y[j])
                        * (z[k + 1] - z[k])
                }
            }
        }
    }
    Ok(total)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2021, 22).unwrap()).unwrap()).unwrap(),
        570915
    );
    assert_eq!(
        part2(parse(parse::data(2021, 22).unwrap()).unwrap()).unwrap(),
        1268313839428137
    );
}
