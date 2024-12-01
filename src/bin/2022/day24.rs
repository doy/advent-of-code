#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

#[derive(Debug, Clone)]
pub struct Map {
    blizzards: HashSet<((Row, Col), Direction)>,
    size: (Row, Col),
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (0..self.size.0 .0).map(Row) {
            for col in (0..self.size.1 .0).map(Col) {
                let blizzards: Vec<_> = self
                    .blizzards
                    .iter()
                    .filter(|(pos, dir)| *pos == (row, col))
                    .collect();
                match blizzards.len() {
                    0 => write!(f, ".")?,
                    1 => write!(f, "{}", blizzards[0].1)?,
                    _ => write!(f, "{}", blizzards.len())?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn new(
        blizzards: HashSet<((Row, Col), Direction)>,
        size: (Row, Col),
    ) -> Self {
        Self { blizzards, size }
    }

    #[must_use]
    fn step(&self) -> Self {
        Self {
            blizzards: self
                .blizzards
                .iter()
                .map(|(pos, direction)| {
                    (direction.move_wrapped(*pos, self.size), *direction)
                })
                .collect(),
            size: self.size,
        }
    }

    fn blizzard(&self, pos: (Row, Col)) -> bool {
        self.blizzards.contains(&(pos, Direction::Up))
            || self.blizzards.contains(&(pos, Direction::Down))
            || self.blizzards.contains(&(pos, Direction::Left))
            || self.blizzards.contains(&(pos, Direction::Right))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Pos {
    Start,
    End,
    Pos((Row, Col)),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct State {
    time: usize,
    pos: Pos,
}

impl State {
    fn new(start: bool) -> Self {
        Self {
            time: 0,
            pos: if start { Pos::Start } else { Pos::End },
        }
    }
}

pub fn parse(fh: File) -> Result<Map> {
    let mut lines = parse::raw_lines(fh);
    lines.next().unwrap();
    let mut blizzards = HashSet::new();
    let mut size = (Row(0), Col(0));
    for (row, line) in lines.enumerate() {
        let row = Row(row);
        if line.starts_with("##") {
            size = (row, Col(line.len() - 2));
            break;
        }
        for (col, c) in line.as_bytes().iter().enumerate() {
            let col = Col(col);
            if let Ok(direction) = Direction::try_from(*c) {
                blizzards.insert(((row, col - 1), direction));
            }
        }
    }
    Ok(Map::new(blizzards, size))
}

struct Pathfinder {
    maps_at_time: std::cell::RefCell<Vec<Map>>,
}

impl Pathfinder {
    fn new(map: Map) -> Self {
        Self {
            maps_at_time: std::cell::RefCell::new(vec![map]),
        }
    }

    fn size(&self) -> (Row, Col) {
        self.maps_at_time.borrow().first().unwrap().size
    }

    fn map_at_time(&self, time: usize) -> std::cell::Ref<'_, Map> {
        {
            let mut maps_at_time = self.maps_at_time.borrow_mut();
            while time >= maps_at_time.len() {
                let next = maps_at_time.iter().last().unwrap().step();
                maps_at_time.push(next);
            }
        }
        std::cell::Ref::map(self.maps_at_time.borrow(), |maps| {
            maps.get(time).unwrap()
        })
    }
}

impl advent_of_code::graph::Graph<State, Pos> for Pathfinder {
    type Edges = Vec<Pos>;

    fn edges(&self, state: State) -> Self::Edges {
        let size = self.size();
        let next = self.map_at_time(state.time + 1);
        let mut v = vec![];
        match state.pos {
            Pos::Start => {
                v.push(Pos::Start);
                if !next.blizzard((Row(0), Col(0))) {
                    v.push(Pos::Pos((Row(0), Col(0))));
                }
            }
            Pos::End => {
                v.push(Pos::End);
                if !next.blizzard((size.0 - 1, size.1 - 1)) {
                    v.push(Pos::Pos((size.0 - 1, size.1 - 1)));
                }
            }
            Pos::Pos(pos) => {
                if pos.0 < size.0 - 1 && !next.blizzard((pos.0 + 1, pos.1)) {
                    v.push(Pos::Pos((pos.0 + 1, pos.1)));
                }
                if pos.1 < size.1 - 1 && !next.blizzard((pos.0, pos.1 + 1)) {
                    v.push(Pos::Pos((pos.0, pos.1 + 1)));
                }
                if pos.0 > Row(0) && !next.blizzard((pos.0 - 1, pos.1)) {
                    v.push(Pos::Pos((pos.0 - 1, pos.1)));
                }
                if pos.1 > Col(0) && !next.blizzard((pos.0, pos.1 - 1)) {
                    v.push(Pos::Pos((pos.0, pos.1 - 1)));
                }
                if !next.blizzard(pos) {
                    v.push(Pos::Pos(pos));
                }
                if pos == (Row(0), Col(0)) {
                    v.push(Pos::Start);
                }
                if pos == (size.0 - 1, size.1 - 1) {
                    v.push(Pos::End);
                }
            }
        }
        v
    }

    fn edge(&self, state: State, pos: Pos) -> (State, u64) {
        (
            State {
                pos,
                time: state.time + 1,
            },
            1,
        )
    }
}

pub fn part1(map: Map) -> Result<u64> {
    let state = State::new(true);
    let pathfinder = Pathfinder::new(map);
    let (dist, _) = pathfinder.dijkstra(state, |state| state.pos == Pos::End);
    Ok(dist)
}

pub fn part2(map: Map) -> Result<u64> {
    let state = State::new(true);
    let pathfinder = Pathfinder::new(map);
    let (dist1, _) =
        pathfinder.dijkstra(state, |state| state.pos == Pos::End);

    let map = pathfinder.map_at_time(dist1 as usize).clone();
    let state = State::new(false);
    let pathfinder = Pathfinder::new(map);
    let (dist2, _) =
        pathfinder.dijkstra(state, |state| state.pos == Pos::Start);

    let map = pathfinder.map_at_time(dist2 as usize).clone();
    let state = State::new(true);
    let pathfinder = Pathfinder::new(map);
    let (dist3, _) =
        pathfinder.dijkstra(state, |state| state.pos == Pos::End);

    Ok(dist1 + dist2 + dist3)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 24).unwrap()).unwrap()).unwrap(),
        245
    );
    assert_eq!(
        part2(parse(parse::data(2022, 24).unwrap()).unwrap()).unwrap(),
        798
    );
}
