use advent_of_code::{graph::Graph, prelude::*};

pub struct Map(Grid<u8>);

impl Map {
    fn rating(&self, pos: Pos) -> i64 {
        if self.0[pos] == 0 {
            return 1;
        }
        self.0
            .adjacent(pos, false)
            .filter(|apos| self.0[*apos] + 1 == self.0[pos])
            .map(|pos| self.rating(pos))
            .sum()
    }
}

impl advent_of_code::graph::Graph<Pos, Pos> for Map {
    type Edges = Vec<Pos>;

    fn edges(&self, v: Pos) -> Self::Edges {
        self.0
            .adjacent(v, false)
            .filter(|e| self.0[v] + 1 == self.0[*e])
            .collect()
    }

    fn edge(&self, _: Pos, e: Pos) -> (Pos, u64) {
        (e, 1)
    }
}

pub fn parse(fh: File) -> Result<Map> {
    Ok(Map(parse::grid(parse::raw_lines(fh), |c, _| c - b'0')))
}

pub fn part1(map: Map) -> Result<i64> {
    let trailheads: Vec<Pos> = map
        .0
        .indexed_cells()
        .filter_map(|(pos, c)| (*c == 0).then_some(pos))
        .collect();
    let peaks: Vec<Pos> = map
        .0
        .indexed_cells()
        .filter_map(|(pos, c)| (*c == 9).then_some(pos))
        .collect();
    let mut total = 0;
    for trailhead in trailheads {
        let mut score = 0;
        let paths = map.dijkstra_full(trailhead);
        for peak in &peaks {
            if matches!(paths.get(peak), Some((_, 9))) {
                score += 1;
            }
        }
        total += score;
    }
    Ok(total)
}

pub fn part2(map: Map) -> Result<i64> {
    let peaks: Vec<Pos> = map
        .0
        .indexed_cells()
        .filter_map(|(pos, c)| (*c == 9).then_some(pos))
        .collect();
    let mut total = 0;
    for peak in peaks {
        total += map.rating(peak);
    }
    Ok(total)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 10).unwrap()).unwrap()).unwrap(),
        607
    );
    assert_eq!(
        part2(parse(parse::data(2024, 10).unwrap()).unwrap()).unwrap(),
        1384
    );
}
