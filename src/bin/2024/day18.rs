use advent_of_code::prelude::*;

pub struct Map {
    bytes: Vec<Pos>,
}

struct BoolGrid<'a>(&'a Grid<bool>);

impl advent_of_code::graph::Graph<Pos, Pos> for BoolGrid<'_> {
    fn edges(&self, v: Pos) -> impl IntoIterator<Item = Pos> {
        self.0.adjacent(v, false).filter(|pos| !self.0[*pos])
    }

    fn edge(&self, _: Pos, e: Pos) -> (Pos, u64) {
        (e, 1)
    }
}

pub fn parse(fh: File) -> Result<Map> {
    Ok(Map {
        bytes: parse::raw_lines(fh)
            .map(|line| {
                let mut parts = line.split(',');
                let col = parts.next().unwrap().parse().unwrap();
                let row = parts.next().unwrap().parse().unwrap();
                Pos(Row(row), Col(col))
            })
            .collect(),
    })
}

pub fn part1(map: Map) -> Result<i64> {
    let size = Size(Row(71), Col(71));
    let mut grid = Grid::default();
    grid.grow(size);
    let start = Pos::default();
    let end = Pos(size.0 - 1, size.1 - 1);
    for byte in &map.bytes[..1024] {
        grid[*byte] = true;
    }
    Ok(BoolGrid(&grid)
        .dijkstra(start, |pos| pos == end)
        .unwrap()
        .0
        .try_into()
        .unwrap())
}

pub fn part2(map: Map) -> Result<i64> {
    let size = Size(Row(71), Col(71));
    let mut grid = Grid::default();
    grid.grow(size);
    let start = Pos::default();
    let end = Pos(size.0 - 1, size.1 - 1);
    let idx: Vec<_> = (0..map.bytes.len()).collect();
    let i = idx
        .binary_search_by(|i| {
            let mut grid = grid.clone();
            for pos in &map.bytes[..=*i] {
                grid[*pos] = true;
            }
            if BoolGrid(&grid).dijkstra(start, |pos| pos == end).is_none() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        })
        .unwrap_err();
    Ok((map.bytes[i].0.0 + map.bytes[i].1.0 * 100)
        .try_into()
        .unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 18).unwrap()).unwrap()).unwrap(),
        316
    );
    assert_eq!(
        part2(parse(parse::data(2024, 18).unwrap()).unwrap()).unwrap(),
        4518
    );
}
