use advent_of_code::prelude::*;

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
    fn edges(&self, v: Pos) -> impl IntoIterator<Item = Pos> {
        self.0
            .adjacent(v, false)
            .filter(move |e| self.0[v] + 1 == self.0[*e])
    }

    fn edge(&self, _: Pos, e: Pos) -> (Pos, u64) {
        (e, 1)
    }
}

pub fn parse(fh: File) -> Result<Map> {
    Ok(Map(parse::grid(parse::raw_lines(fh), |c, _| c - b'0')))
}

pub fn part1(map: Map) -> Result<i64> {
    Ok(map
        .0
        .indexed_cells()
        .filter_map(|(pos, c)| (*c == 0).then_some(pos))
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|trailhead| {
            i64::try_from(
                map.dijkstra_full(trailhead)
                    .values()
                    .filter(|(_, len)| *len == 9)
                    .count(),
            )
            .unwrap()
        })
        .sum())
}

pub fn part2(map: Map) -> Result<i64> {
    Ok(map
        .0
        .indexed_cells()
        .filter_map(|(pos, c)| (*c == 9).then_some(pos))
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|peak| map.rating(peak))
        .sum())
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
