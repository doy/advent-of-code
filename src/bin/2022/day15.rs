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

    fn in_radius(&self, pos: (IRow, ICol)) -> bool {
        dist(self.pos, pos) <= self.radius
    }
}

pub struct Map {
    sensors: Vec<Sensor>,
}

impl Map {
    fn largest_radius(&self) -> usize {
        self.sensors
            .iter()
            .map(|sensor| sensor.radius)
            .max()
            .unwrap()
    }

    fn width(&self) -> usize {
        let min_col: ICol = self
            .sensors
            .iter()
            .map(|sensor| sensor.pos.1)
            .min()
            .unwrap();
        let max_col: ICol = self
            .sensors
            .iter()
            .map(|sensor| sensor.pos.1)
            .max()
            .unwrap();
        max_col.abs_diff(min_col).0
    }

    fn height(&self) -> usize {
        let min_row: IRow = self
            .sensors
            .iter()
            .map(|sensor| sensor.pos.0)
            .min()
            .unwrap();
        let max_row: IRow = self
            .sensors
            .iter()
            .map(|sensor| sensor.pos.0)
            .max()
            .unwrap();
        max_row.abs_diff(min_row).0
    }

    fn nearby_sensor(&self, pos: (IRow, ICol)) -> Option<Sensor> {
        self.sensors
            .iter()
            .copied()
            .find(|sensor| sensor.in_radius(pos))
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
    Ok(Map { sensors })
}

pub fn part1(map: Map) -> Result<usize> {
    let row = IRow(2_000_000);
    let margin = map.largest_radius() as isize + 1;
    let mut total = 0;
    for col in (-margin..(map.width() as isize + margin)).map(ICol) {
        if map.sensors.iter().any(|sensor| sensor.beacon == (row, col)) {
            continue;
        }
        if map.nearby_sensor((row, col)).is_some() {
            total += 1;
        }
    }
    Ok(total)
}

pub fn part2(map: Map) -> Result<isize> {
    for row in (0..=4_000_000).map(IRow) {
        let mut col = ICol(0);
        loop {
            if let Some(sensor) = map.nearby_sensor((row, col)) {
                let row_radius = sensor.radius - sensor.pos.0.abs_diff(row).0;
                col = sensor.pos.1 + row_radius as isize + 1;
                if col > ICol(4_000_000) {
                    break;
                }
            } else {
                return Ok((col.0) * 4_000_000 + (row.0));
            }
        }
    }
    Ok(0)
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
