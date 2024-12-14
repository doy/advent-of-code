use advent_of_code::prelude::*;

pub struct Robot {
    pos: Pos,
    v: IPos,
}

impl Robot {
    fn at_second(&self, size: Size, t: isize) -> Pos {
        Pos(
            ((size.0.i() * t + self.pos.0.i() + self.v.0 * t) % size.0.i())
                .u(),
            ((size.1.i() * t + self.pos.1.i() + self.v.1 * t) % size.1.i())
                .u(),
        )
    }
}

impl std::str::FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let cap = regex_captures!(
            r"^p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)$",
            s
        )
        .unwrap();
        Ok(Self {
            pos: Pos(
                Row(cap[2].parse().unwrap()),
                Col(cap[1].parse().unwrap()),
            ),
            v: IPos(
                IRow(cap[4].parse().unwrap()),
                ICol(cap[3].parse().unwrap()),
            ),
        })
    }
}

#[allow(dead_code)]
fn display(robots: &[Robot]) {
    for row in (0..103).map(Row) {
        for col in (0..101).map(Col) {
            if robots.iter().any(|robot| robot.pos == Pos(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn parse(fh: File) -> Result<Vec<Robot>> {
    Ok(parse::lines(fh).collect())
}

pub fn part1(robots: Vec<Robot>) -> Result<i64> {
    let size = Size(Row(103), Col(101));
    let positions: Vec<_> = robots
        .par_iter()
        .map(|robot| robot.at_second(size, 100))
        .collect();
    let mut quadrants = vec![0, 0, 0, 0];
    let center = Pos(size.0 / 2, size.1 / 2);
    for pos in positions {
        if pos.0 < center.0 && pos.1 < center.1 {
            quadrants[0] += 1;
        }
        if pos.0 < center.0 && pos.1 > center.1 {
            quadrants[1] += 1;
        }
        if pos.0 > center.0 && pos.1 < center.1 {
            quadrants[2] += 1;
        }
        if pos.0 > center.0 && pos.1 > center.1 {
            quadrants[3] += 1;
        }
    }
    Ok(quadrants.into_iter().product())
}

pub fn part2(robots: Vec<Robot>) -> Result<i64> {
    let size = Size(Row(103), Col(101));
    Ok((0..isize::MAX)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|t| {
            let positions: HashSet<Pos> = robots
                .iter()
                .map(|robot| robot.at_second(size, *t))
                .collect();
            positions
                .iter()
                .filter(|pos| {
                    pos.adjacent(size, true)
                        .all(|pos| positions.contains(&pos))
                })
                .take(5)
                .count()
                == 5
        })
        .unwrap()
        .try_into()
        .unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 14).unwrap()).unwrap()).unwrap(),
        217328832
    );
    assert_eq!(
        part2(parse(parse::data(2024, 14).unwrap()).unwrap()).unwrap(),
        7412
    );
}
