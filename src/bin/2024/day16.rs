use advent_of_code::{graph::Graph, prelude::*};

pub struct Map {
    map: Grid<bool>,
    start: Pos,
    end: Pos,
}

pub fn parse(fh: File) -> Result<Map> {
    let mut start = Pos::default();
    let mut end = Pos::default();
    let map = parse::grid(parse::raw_lines(fh), |c, pos| match c {
        b'#' => false,
        b'.' => true,
        b'S' => {
            start = pos;
            true
        }
        b'E' => {
            end = pos;
            true
        }
        _ => unreachable!(),
    });
    Ok(Map { map, start, end })
}

impl advent_of_code::graph::Graph<(Pos, Direction), (Pos, Direction)>
    for Map
{
    fn edges(
        &self,
        (pos, direction): (Pos, Direction),
    ) -> impl IntoIterator<Item = (Pos, Direction)> {
        let mut edges =
            vec![(pos, direction.turn_left()), (pos, direction.turn_right())];
        let next = direction.move_checked(pos, self.map.size()).unwrap();
        if self.map[next] {
            edges.push((next, direction));
        }
        edges
    }

    fn edge(
        &self,
        v: (Pos, Direction),
        e: (Pos, Direction),
    ) -> ((Pos, Direction), u64) {
        (e, if v.0 == e.0 { 1000 } else { 1 })
    }
}

pub fn part1(map: Map) -> Result<i64> {
    Ok(map
        .dijkstra((map.start, Direction::Right), |(pos, _)| pos == map.end)
        .unwrap()
        .0
        .try_into()
        .unwrap())
}

pub fn part2(map: Map) -> Result<i64> {
    let from_start = map.dijkstra_full((map.start, Direction::Right));
    let mut from_end = vec![];
    for direction in [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ] {
        if map.map[direction.move_checked(map.end, map.map.size()).unwrap()] {
            from_end
                .push(map.dijkstra_full((map.end, direction.turn_around())));
        }
    }
    let min_distance = from_end
        .iter()
        .map(|from_end| {
            from_end[&(map.start, Direction::Right.turn_around())].1
        })
        .min()
        .unwrap();
    Ok(map
        .map
        .par_indexed_cells()
        .filter(|(_, c)| **c)
        .map(|(pos, _)| {
            if [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
            .iter()
            .any(|direction| {
                from_start[&(pos, *direction)].1
                    + from_end
                        .iter()
                        .map(|from_end| {
                            from_end[&(pos, direction.turn_around())].1
                        })
                        .min()
                        .unwrap()
                    == min_distance
            }) {
                1
            } else {
                0
            }
        })
        .sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 16).unwrap()).unwrap()).unwrap(),
        98416
    );
    assert_eq!(
        part2(parse(parse::data(2024, 16).unwrap()).unwrap()).unwrap(),
        471
    );
}
