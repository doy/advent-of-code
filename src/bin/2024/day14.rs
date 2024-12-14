use advent_of_code::prelude::*;

pub struct Robot {
    pos: Pos,
    v: IPos,
}

impl Robot {
    fn mv(&mut self, size: Size) {
        self.pos = Pos(
            ((size.0.i() + self.pos.0.i() + self.v.0) % size.0.i()).u(),
            ((size.1.i() + self.pos.1.i() + self.v.1) % size.1.i()).u(),
        );
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

pub fn part1(mut robots: Vec<Robot>) -> Result<i64> {
    let size = Size(Row(103), Col(101));
    for _ in 0..100 {
        for robot in &mut robots {
            robot.mv(size);
        }
    }
    let mut quadrants = vec![0, 0, 0, 0];
    let center = Pos(Row(size.0 .0 / 2), Col(size.1 .0 / 2));
    for robot in robots {
        if robot.pos.0 < center.0 && robot.pos.1 < center.1 {
            quadrants[0] += 1;
        }
        if robot.pos.0 < center.0 && robot.pos.1 > center.1 {
            quadrants[1] += 1;
        }
        if robot.pos.0 > center.0 && robot.pos.1 < center.1 {
            quadrants[2] += 1;
        }
        if robot.pos.0 > center.0 && robot.pos.1 > center.1 {
            quadrants[3] += 1;
        }
    }
    Ok(quadrants.into_iter().product())
}

pub fn part2(mut robots: Vec<Robot>) -> Result<i64> {
    let size = Size(Row(103), Col(101));
    let mut seconds = 0;
    loop {
        seconds += 1;
        for robot in &mut robots {
            robot.mv(size);
        }
        let positions: HashSet<Pos> =
            robots.iter().map(|robot| robot.pos).collect();
        if positions
            .iter()
            .filter(|pos| {
                pos.adjacent(size, true).all(|pos| positions.contains(&pos))
            })
            .take(5)
            .count()
            == 5
        {
            // display(&robots);
            break;
        }
    }
    Ok(seconds)
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
