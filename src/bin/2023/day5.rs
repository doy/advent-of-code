use advent_of_code::prelude::*;

#[derive(Debug, Clone)]
pub struct MapRange {
    dst_start: i64,
    src_start: i64,
    len: i64,
}

impl MapRange {
    fn map(&self, n: i64) -> Option<i64> {
        if n >= self.src_start && n < self.src_start + self.len {
            Some(self.dst_start + (n - self.src_start))
        } else {
            None
        }
    }

    fn src_range(&self) -> std::ops::RangeInclusive<i64> {
        self.src_start..=(self.src_start + self.len - 1)
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn map(&self, n: i64) -> i64 {
        for range in &self.ranges {
            if let Some(mapped) = range.map(n) {
                return mapped;
            }
        }
        n
    }

    fn map_range(
        &self,
        range: std::ops::RangeInclusive<i64>,
    ) -> Vec<std::ops::RangeInclusive<i64>> {
        let mut ranges = vec![range];
        for map_range in &self.ranges {
            let mut new_ranges = vec![];
            for range in ranges {
                new_ranges.extend(
                    split_range(range, map_range.src_range()).into_iter(),
                );
            }
            ranges = new_ranges;
        }
        ranges
            .into_iter()
            .map(|range| self.map(*range.start())..=self.map(*range.end()))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn location(&self, mut n: i64) -> i64 {
        for map in &self.maps {
            n = map.map(n);
        }
        n
    }
}

fn split_range(
    a: std::ops::RangeInclusive<i64>,
    b: std::ops::RangeInclusive<i64>,
) -> Vec<std::ops::RangeInclusive<i64>> {
    if a.start() > b.end() {
        vec![a]
    } else if a.start() < b.start() {
        if a.end() < b.start() {
            vec![a]
        } else if a.end() <= b.end() {
            vec![*a.start()..=(*b.start() - 1), *b.start()..=*a.end()]
        } else {
            vec![
                *a.start()..=(*b.start() - 1),
                b.clone(),
                (*b.end() + 1)..=*a.end(),
            ]
        }
    } else if a.end() <= b.end() {
        vec![a]
    } else {
        vec![*a.start()..=*b.end(), (*b.end() + 1)..=*a.end()]
    }
}

pub fn parse(fh: File) -> Result<Almanac> {
    let mut lines = parse::raw_lines(fh).fuse();

    let seeds = lines.next().unwrap();
    let seeds: Vec<i64> = seeds
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    lines.next().unwrap();

    let mut maps = vec![];
    while lines.next().is_some() {
        let mut ranges = vec![];
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let parts: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            ranges.push(MapRange {
                dst_start: parts[0],
                src_start: parts[1],
                len: parts[2],
            })
        }
        maps.push(Map { ranges })
    }

    Ok(Almanac { seeds, maps })
}

pub fn part1(almanac: Almanac) -> Result<i64> {
    Ok(almanac
        .seeds
        .iter()
        .copied()
        .map(|n| almanac.location(n))
        .min()
        .unwrap())
}

pub fn part2(almanac: Almanac) -> Result<i64> {
    let mut tests = vec![];
    for seed_range in almanac.seeds.chunks(2) {
        #[allow(clippy::single_range_in_vec_init)]
        let mut seed_ranges =
            vec![seed_range[0]..=(seed_range[0] + seed_range[1] - 1)];
        for map in &almanac.maps {
            seed_ranges = seed_ranges
                .into_iter()
                .flat_map(|range| map.map_range(range))
                .collect();
        }
        tests.extend(seed_ranges.into_iter().map(|range| *range.start()));
    }
    Ok(tests.into_iter().min().unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 5).unwrap()).unwrap()).unwrap(),
        173706076
    );
    assert_eq!(
        part2(parse(parse::data(2023, 5).unwrap()).unwrap()).unwrap(),
        11611182
    );
}
