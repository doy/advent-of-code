use crate::prelude::*;

const ORIENTATIONS: &[&dyn Fn(Point) -> Point] = &[
    &|p| Point::new(p.y, p.z, p.x),
    &|p| Point::new(-p.y, -p.z, p.x),
    &|p| Point::new(p.z, -p.y, p.x),
    &|p| Point::new(-p.z, p.y, p.x),
    &|p| Point::new(p.y, -p.z, -p.x),
    &|p| Point::new(-p.y, p.z, -p.x),
    &|p| Point::new(p.z, p.y, -p.x),
    &|p| Point::new(-p.z, -p.y, -p.x),
    &|p| Point::new(p.x, -p.z, p.y),
    &|p| Point::new(-p.x, p.z, p.y),
    &|p| Point::new(p.z, p.x, p.y),
    &|p| Point::new(-p.z, -p.x, p.y),
    &|p| Point::new(p.x, p.z, -p.y),
    &|p| Point::new(-p.x, -p.z, -p.y),
    &|p| Point::new(p.z, -p.x, -p.y),
    &|p| Point::new(-p.z, p.x, -p.y),
    &|p| Point::new(p.x, p.y, p.z),
    &|p| Point::new(-p.x, -p.y, p.z),
    &|p| Point::new(p.y, -p.x, p.z),
    &|p| Point::new(-p.y, p.x, p.z),
    &|p| Point::new(p.x, -p.y, -p.z),
    &|p| Point::new(-p.x, p.y, -p.z),
    &|p| Point::new(p.y, p.x, -p.z),
    &|p| Point::new(-p.y, -p.x, -p.z),
];

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i16,
    y: i16,
    z: i16,
}

impl Point {
    fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl std::fmt::Display for Point {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Point>,
}

impl Scanner {
    fn parse(lines: &mut impl Iterator<Item = String>) -> Option<Self> {
        if lines.next().is_some() {
            let mut beacons = vec![];
            for line in lines {
                if line.is_empty() {
                    break;
                }
                let mut parts = line.split(',').map(|i| i.parse().unwrap());
                let x = parts.next().unwrap();
                let y = parts.next().unwrap();
                let z = parts.next().unwrap();
                beacons.push(Point::new(x, y, z))
            }
            Some(Self { beacons })
        } else {
            None
        }
    }

    fn matches(&self, other: &HashSet<Point>) -> Option<(usize, Point)> {
        for (i, beacons) in self.each_orientation().enumerate() {
            let mut offsets = vec![];
            for beacon1 in beacons.clone() {
                for beacon2 in other {
                    offsets.push(*beacon2 - beacon1);
                }
            }
            for offset in offsets {
                let set1: HashSet<_> =
                    beacons.iter().map(|beacon| *beacon + offset).collect();
                let matches = set1.intersection(other).count();
                if matches == 0 {
                    panic!("bug");
                }
                if matches >= 12 {
                    return Some((i, offset));
                }
            }
        }
        None
    }

    fn each_orientation(&self) -> impl Iterator<Item = Vec<Point>> {
        let beacons = self.beacons.clone();
        ORIENTATIONS.iter().map(move |orientation| {
            beacons.iter().map(|beacon| orientation(*beacon)).collect()
        })
    }

    fn at_orientation<F: Fn(Point) -> Point>(
        &self,
        orientation: F,
        offset: Point,
    ) -> Self {
        Self {
            beacons: self
                .beacons
                .iter()
                .map(|beacon| orientation(*beacon) + offset)
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct Scan {
    scanners: Vec<Scanner>,
}

impl Scan {
    fn parse(mut lines: impl Iterator<Item = String>) -> Self {
        let mut scanners = vec![];
        while let Some(scanner) = Scanner::parse(lines.by_ref()) {
            scanners.push(scanner);
        }
        Self { scanners }
    }

    fn scanners(&self) -> &[Scanner] {
        &self.scanners
    }
}

pub fn parse(fh: File) -> Result<Scan> {
    Ok(Scan::parse(parse::lines(fh)))
}

pub fn part1(scan: Scan) -> Result<i64> {
    let mut beacons: HashSet<Point> = HashSet::new();
    let mut skip = None;
    for (i, scanner1) in scan.scanners().iter().enumerate() {
        for (j, scanner2) in scan.scanners().iter().enumerate().skip(i + 1) {
            if let Some((orientation, offset)) =
                scanner2.matches(&scanner1.beacons.iter().copied().collect())
            {
                let scanner2 = scanner2
                    .at_orientation(ORIENTATIONS[orientation], offset);
                beacons.extend(scanner1.beacons.iter());
                beacons.extend(scanner2.beacons.iter());
                skip = Some((i, j));
                break;
            }
        }
        if skip.is_some() {
            break;
        }
    }
    let skip = skip.unwrap();
    let mut scanners = scan.scanners().to_vec();
    scanners.remove(skip.1);
    scanners.remove(skip.0);

    let mut found = None;
    loop {
        for (i, scanner) in scanners.iter().enumerate() {
            if let Some((orientation, offset)) = scanner.matches(&beacons) {
                let scanner =
                    scanner.at_orientation(ORIENTATIONS[orientation], offset);
                beacons.extend(scanner.beacons.iter());
                found = Some(i);
                break;
            }
        }
        if let Some(idx) = found {
            scanners.remove(idx);
            found = None;
        } else {
            break;
        }
    }
    Ok(beacons.len().try_into()?)
}

pub fn part2(scan: Scan) -> Result<i64> {
    let mut beacons: HashSet<Point> = HashSet::new();
    let mut skip = None;
    let mut offsets = vec![];
    for (i, scanner1) in scan.scanners().iter().enumerate() {
        for (j, scanner2) in scan.scanners().iter().enumerate().skip(i + 1) {
            if let Some((orientation, offset)) =
                scanner2.matches(&scanner1.beacons.iter().copied().collect())
            {
                let scanner2 = scanner2
                    .at_orientation(ORIENTATIONS[orientation], offset);
                beacons.extend(scanner1.beacons.iter());
                beacons.extend(scanner2.beacons.iter());
                skip = Some((i, j));
                offsets.push(Point::new(0, 0, 0));
                offsets.push(offset);
                break;
            }
        }
        if skip.is_some() {
            break;
        }
    }
    let skip = skip.unwrap();
    let mut scanners = scan.scanners().to_vec();
    scanners.remove(skip.1);
    scanners.remove(skip.0);

    let mut found = None;
    loop {
        for (i, scanner) in scanners.iter().enumerate() {
            if let Some((orientation, offset)) = scanner.matches(&beacons) {
                let scanner =
                    scanner.at_orientation(ORIENTATIONS[orientation], offset);
                beacons.extend(scanner.beacons.iter());
                offsets.push(offset);
                found = Some(i);
                break;
            }
        }
        if let Some(idx) = found {
            scanners.remove(idx);
            found = None;
        } else {
            break;
        }
    }
    let mut max = 0;
    for offset1 in &offsets {
        for offset2 in &offsets {
            let dist = *offset1 - *offset2;
            let dist = dist.x.abs() + dist.y.abs() + dist.z.abs();
            if dist > max {
                max = dist;
            }
        }
    }
    Ok(max.into())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2021, 19).unwrap()).unwrap()).unwrap(),
        338
    );
    assert_eq!(
        part2(parse(parse::data(2021, 19).unwrap()).unwrap()).unwrap(),
        9862
    );
}
