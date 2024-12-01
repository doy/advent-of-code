use advent_of_code::prelude::*;

#[derive(Debug)]
pub struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn distance(&self, t: i64) -> i64 {
        (self.time - t) * t
    }
}

pub fn parse(fh: File) -> Result<Vec<Race>> {
    let mut lines = parse::raw_lines(fh);
    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();
    let times: Vec<_> = time
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let distances: Vec<_> = distance
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    Ok((0..times.len())
        .map(|i| Race {
            time: times[i],
            distance: distances[i],
        })
        .collect())
}

pub fn part1(races: Vec<Race>) -> Result<i64> {
    Ok(races
        .into_iter()
        .map(|race| -> i64 {
            (0..race.time)
                .map(|t| race.distance(t))
                .filter(|d| *d > race.distance)
                .count()
                .try_into()
                .unwrap()
        })
        .product())
}

pub fn part2(races: Vec<Race>) -> Result<i64> {
    let mut real_race = Race {
        time: 0,
        distance: 0,
    };
    for race in races {
        real_race.time =
            real_race.time * 10i64.pow(race.time.ilog10() + 1) + race.time;
        real_race.distance = real_race.distance
            * 10i64.pow(race.distance.ilog10() + 1)
            + race.distance;
    }
    Ok((0..real_race.time)
        .map(|t| real_race.distance(t))
        .filter(|d| *d > real_race.distance)
        .count()
        .try_into()
        .unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 6).unwrap()).unwrap()).unwrap(),
        861300
    );
    assert_eq!(
        part2(parse(parse::data(2023, 6).unwrap()).unwrap()).unwrap(),
        28101347
    );
}
