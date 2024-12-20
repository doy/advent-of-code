use advent_of_code::prelude::*;

#[derive(Debug)]
pub struct Map {
    grid: Grid<bool>,
    start: Pos,
    end: Pos,
}

impl Map {
    fn count_paths(&self, cheat_distance: usize, time_save: u64) -> i64 {
        let forward = self.dijkstra_full(self.start);
        let backward = self.dijkstra_full(self.end);
        let max = forward[&self.end].1;
        self.grid
            .par_indexed_cells()
            .filter_map(|(pos, passable)| passable.then_some(pos))
            .map(|start| {
                self.grid
                    .near(start, false, cheat_distance)
                    .filter(|end| {
                        start > *end
                            && self.grid[*end]
                            && max
                                - (forward[&start].1 + backward[end].1)
                                    .min(forward[end].1 + backward[&start].1)
                                - u64::try_from(start.distance(*end, false))
                                    .unwrap()
                                + 1
                                >= time_save
                    })
                    .count()
            })
            .sum::<usize>()
            .try_into()
            .unwrap()
    }
}

impl advent_of_code::graph::Graph<Pos, Pos> for Map {
    type Edges = Vec<Pos>;

    fn edges(&self, v: Pos) -> Self::Edges {
        self.grid
            .adjacent(v, false)
            .filter(|pos| self.grid[*pos])
            .collect()
    }

    fn edge(&self, _: Pos, e: Pos) -> (Pos, u64) {
        (e, 1)
    }
}

pub fn parse(fh: File) -> Result<Map> {
    let mut start = Pos::default();
    let mut end = Pos::default();
    let grid = parse::grid(parse::raw_lines(fh), |c, pos| match c {
        b'.' => true,
        b'#' => false,
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
    Ok(Map { grid, start, end })
}

pub fn part1(map: Map) -> Result<i64> {
    Ok(map.count_paths(2, 100))
}

pub fn part2(map: Map) -> Result<i64> {
    Ok(map.count_paths(20, 100))
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 20).unwrap()).unwrap()).unwrap(),
        1263
    );
    assert_eq!(
        part2(parse(parse::data(2024, 20).unwrap()).unwrap()).unwrap(),
        957831
    );
}
