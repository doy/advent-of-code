#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

fn dist(a: (IRow, ICol), b: (IRow, ICol)) -> usize {
    a.0.abs_diff(b.0).0 + a.1.abs_diff(b.1).0
}

#[derive(Debug, Copy, Clone)]
struct Sensor {
    pos: (IRow, ICol),
    beacon: (IRow, ICol),
    radius: usize,
}

impl Sensor {
    fn new(pos: (IRow, ICol), beacon: (IRow, ICol)) -> Self {
        Self {
            pos,
            beacon,
            radius: dist(pos, beacon),
        }
    }

    fn row_radius(&self, row: IRow) -> usize {
        self.radius - self.pos.0.abs_diff(row).0
    }

    fn in_radius(&self, pos: (IRow, ICol)) -> bool {
        dist(self.pos, pos) <= self.radius
    }
}

pub struct Map {
    sensors: Vec<Sensor>,
}

impl Map {
    fn new(mut sensors: Vec<Sensor>) -> Self {
        sensors.sort_unstable_by(|a, b| b.radius.cmp(&a.radius));
        Self { sensors }
    }

    fn nearby_sensor(&self, pos: (IRow, ICol)) -> Option<Sensor> {
        self.sensors
            .iter()
            .copied()
            .find(|sensor| sensor.in_radius(pos))
    }

    fn beacons(&self) -> HashSet<(IRow, ICol)> {
        self.sensors
            .iter()
            .copied()
            .map(|sensor| sensor.beacon)
            .collect()
    }
}

pub fn parse(fh: File) -> Result<Map> {
    let mut sensors = vec![];
    for line in parse::raw_lines(fh) {
        let cap = regex_captures!(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)", &line)
            .ok_or_else(|| anyhow::anyhow!("no match"))?;
        let sensor_x: isize = cap[1].parse()?;
        let sensor_y: isize = cap[2].parse()?;
        let beacon_x: isize = cap[3].parse()?;
        let beacon_y: isize = cap[4].parse()?;
        sensors.push(Sensor::new(
            (IRow(sensor_y), ICol(sensor_x)),
            (IRow(beacon_y), ICol(beacon_x)),
        ));
    }
    Ok(Map::new(sensors))
}

pub fn part1(mut map: Map) -> Result<usize> {
    let row = IRow(2_000_000);
    map.sensors = map
        .sensors
        .iter()
        .copied()
        .filter(|sensor| {
            ((sensor.pos.0 - sensor.radius as isize)
                ..=(sensor.pos.0 + sensor.radius as isize))
                .contains(&row)
        })
        .collect();
    let min_sensor = map
        .sensors
        .iter()
        .min_by_key(|sensor| sensor.pos.1)
        .unwrap();
    let max_sensor = map
        .sensors
        .iter()
        .max_by_key(|sensor| sensor.pos.1)
        .unwrap();
    let mut total = 0;
    let mut col = min_sensor.pos.1 - min_sensor.row_radius(row) as isize;
    while col < max_sensor.pos.1 + max_sensor.row_radius(row) as isize {
        if let Some(sensor) = map.nearby_sensor((row, col)) {
            let row_radius = sensor.radius - sensor.pos.0.abs_diff(row).0;
            let skip = (sensor.pos.1 - col.0).0 as usize + row_radius + 1;
            total += skip;
            col = col + skip as isize;
        } else {
            col = col + 1;
        }
    }
    Ok(total
        - map
            .beacons()
            .iter()
            .filter(|beacon| beacon.0 == row)
            .count())
}

pub fn part2(map: Map) -> Result<isize> {
    for row in (0..=4_000_000).map(IRow) {
        let mut col = ICol(0);
        while col <= ICol(4_000_000) {
            if let Some(sensor) = map.nearby_sensor((row, col)) {
                col = sensor.pos.1 + sensor.row_radius(row) as isize + 1;
            } else {
                return Ok((col.0) * 4_000_000 + (row.0));
            }
        }
    }
    panic!("couldn't find beacon");
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 15).unwrap()).unwrap()).unwrap(),
        4737443
    );
    assert_eq!(
        part2(parse(parse::data(2022, 15).unwrap()).unwrap()).unwrap(),
        11482462818989
    );
}
