#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

fn dist(a: (Row, Col), b: (Row, Col)) -> usize {
    a.0.abs_diff(b.0).0 + a.1.abs_diff(b.1).0
}

#[derive(Debug, Copy, Clone)]
struct Sensor {
    pos: (Row, Col),
    beacon: (Row, Col),
    radius: usize,
}

impl Sensor {
    fn new(pos: (Row, Col), beacon: (Row, Col)) -> Self {
        Self {
            pos,
            beacon,
            radius: dist(pos, beacon),
        }
    }

    fn in_radius(&self, pos: (Row, Col)) -> bool {
        dist(self.pos, pos) <= self.radius
    }
}

pub struct Map {
    sensors: Vec<Sensor>,
    range_x: usize,
    range_y: usize,
    offset_x: usize,
    offset_y: usize,
}

impl Map {
    fn nearby_sensor(&self, pos: (Row, Col)) -> Option<Sensor> {
        self.sensors
            .iter()
            .copied()
            .find(|sensor| sensor.in_radius(pos))
    }
}

pub fn parse(fh: File) -> Result<Map> {
    let mut sensor_positions = vec![];
    for line in parse::raw_lines(fh) {
        let cap = regex_captures!(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)", &line)
            .ok_or_else(|| anyhow::anyhow!("no match"))?;
        let sensor_x: i64 = cap[1].parse()?;
        let sensor_y: i64 = cap[2].parse()?;
        let beacon_x: i64 = cap[3].parse()?;
        let beacon_y: i64 = cap[4].parse()?;
        sensor_positions.push((sensor_x, sensor_y, beacon_x, beacon_y));
    }
    let min_x = sensor_positions
        .iter()
        .map(|x| x.0)
        .chain(sensor_positions.iter().map(|x| x.2))
        .min()
        .ok_or_else(|| anyhow::anyhow!("empty list"))?;
    let min_y = sensor_positions
        .iter()
        .map(|x| x.1)
        .chain(sensor_positions.iter().map(|x| x.3))
        .min()
        .ok_or_else(|| anyhow::anyhow!("empty list"))?;
    let max_x = sensor_positions
        .iter()
        .map(|x| x.0)
        .chain(sensor_positions.iter().map(|x| x.2))
        .max()
        .ok_or_else(|| anyhow::anyhow!("empty list"))?;
    let max_y = sensor_positions
        .iter()
        .map(|x| x.1)
        .chain(sensor_positions.iter().map(|x| x.3))
        .max()
        .ok_or_else(|| anyhow::anyhow!("empty list"))?;

    let range_x = (max_x - min_x) as usize;
    let range_y = (max_y - min_y) as usize;
    let offset_x = -min_x as usize + range_x;
    let offset_y = -min_y as usize + range_y;

    let mut sensors = vec![];
    for sensor in sensor_positions {
        let pos = (
            Row((sensor.1 + offset_y as i64) as usize),
            Col((sensor.0 + offset_x as i64) as usize),
        );
        let beacon = (
            Row((sensor.3 + offset_y as i64) as usize),
            Col((sensor.2 + offset_x as i64) as usize),
        );
        sensors.push(Sensor::new(pos, beacon));
    }
    Ok(Map {
        sensors,
        range_x,
        range_y,
        offset_x,
        offset_y,
    })
}

pub fn part1(map: Map) -> Result<i64> {
    let row = Row(2_000_000 + map.offset_y);
    let mut total = 0;
    for col in (0..(3 * map.range_x)).map(Col) {
        if map.sensors.iter().any(|sensor| sensor.beacon == (row, col)) {
            continue;
        }
        if map.nearby_sensor((row, col)).is_some() {
            total += 1;
        }
    }
    Ok(total)
}

pub fn part2(map: Map) -> Result<usize> {
    for row in (0..=4_000_000).map(|r| Row(r + map.offset_y)) {
        let mut col = Col(map.offset_x);
        loop {
            if let Some(sensor) = map.nearby_sensor((row, col)) {
                let row_radius = sensor.radius - sensor.pos.0.abs_diff(row).0;
                col = sensor.pos.1 + row_radius + 1;
                if col > Col(4_000_000 + map.offset_x) {
                    break;
                }
            } else {
                return Ok((col.0 - map.offset_x) * 4_000_000
                    + (row.0 - map.offset_y));
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
