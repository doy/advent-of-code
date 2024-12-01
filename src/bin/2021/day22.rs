use advent_of_code::prelude::*;

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

    fn intersects(&self, other: &Self) -> bool {
        (self.x.contains(other.x.start())
            || self.x.contains(other.x.end())
            || other.x.contains(self.x.start())
            || other.x.contains(self.x.end()))
            && (self.y.contains(other.y.start())
                || self.y.contains(other.y.end())
                || other.y.contains(self.y.start())
                || other.y.contains(self.y.end()))
            && (self.z.contains(other.z.start())
                || self.z.contains(other.z.end())
                || other.z.contains(self.z.start())
                || other.z.contains(self.z.end()))
    }

    fn intersected_ranges(&self, other: &Self) -> Vec<Self> {
        let mut ret = vec![];
        let mut found = false;
        for x in Self::split_dimension(&self.x, &other.x) {
            for y in Self::split_dimension(&self.y, &other.y) {
                for z in Self::split_dimension(&self.z, &other.z) {
                    let new = Self::new(x.clone(), y.clone(), z.clone());
                    if other.intersects(&new) {
                        if found {
                            panic!("bug 1");
                        }
                        found = true;
                    } else {
                        ret.push(new);
                    }
                }
            }
        }
        if !found {
            panic!("bug 2");
        }
        ret
    }

    fn split_dimension(
        to_split: &std::ops::RangeInclusive<i64>,
        split_by: &std::ops::RangeInclusive<i64>,
    ) -> Vec<std::ops::RangeInclusive<i64>> {
        if split_by.start() <= to_split.start() {
            if split_by.end() < to_split.start() {
                panic!("bug 3");
            } else if split_by.end() >= to_split.end() {
                vec![to_split.clone()]
            } else {
                vec![
                    *to_split.start()..=*split_by.end(),
                    (*split_by.end() + 1)..=*to_split.end(),
                ]
            }
        } else if split_by.start() > to_split.end() {
            panic!("bug 4");
        } else if split_by.end() >= to_split.end() {
            vec![
                *to_split.start()..=(*split_by.start() - 1),
                *split_by.start()..=*to_split.end(),
            ]
        } else {
            vec![
                *to_split.start()..=(*split_by.start() - 1),
                split_by.clone(),
                (*split_by.end() + 1)..=*to_split.end(),
            ]
        }
    }

    fn size(&self) -> i64 {
        (self.x.end() - self.x.start() + 1)
            * (self.y.end() - self.y.start() + 1)
            * (self.z.end() - self.z.start() + 1)
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
    Ok(Reactor::parse(parse::raw_lines(fh)))
}

pub fn part1(reactor: Reactor) -> Result<u64> {
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
    let mut on: HashSet<Range3D> = HashSet::new();
    for rule in reactor.rules {
        let (intersected, nonintersected) = on
            .into_iter()
            .partition(|range| rule.range.intersects(range));
        on = nonintersected;
        for range in intersected {
            on.extend(range.intersected_ranges(&rule.range));
        }
        if rule.on {
            on.insert(rule.range);
        }
    }
    Ok(on.iter().map(|range| range.size()).sum())
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
