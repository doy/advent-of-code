use advent_of_code::prelude::*;

pub struct Map {
    grid: Grid<u8>,
}

impl advent_of_code::graph::Graph<Pos, Pos> for Map {
    fn edges(&self, v: Pos) -> impl IntoIterator<Item = Pos> {
        self.grid.adjacent(v, false)
    }

    fn edge(&self, _v: Pos, e: Pos) -> (Pos, u64) {
        (e, u64::from(self.grid[e]))
    }
}

pub fn parse(fh: File) -> Result<Map> {
    Ok(Map {
        grid: parse::digit_grid(parse::raw_lines(fh)),
    })
}

pub fn part1(map: Map) -> Result<u64> {
    Ok(map
        .dijkstra(Pos(Row(0), Col(0)), |v| {
            v == Pos(map.grid.rows() - 1, map.grid.cols() - 1)
        })
        .unwrap()
        .0)
}

pub fn part2(map: Map) -> Result<u64> {
    let mut large_grid = Grid::default();
    large_grid
        .grow(Size(Row(map.grid.rows().0 * 5), Col(map.grid.cols().0 * 5)));
    for lrow in 0..5 {
        for lcol in 0..5 {
            for (Pos(Row(row), Col(col)), val) in map.grid.indexed_cells() {
                let mut val = val + lrow + lcol;
                while val > 9 {
                    val -= 9;
                }
                large_grid[Row(usize::from(lrow) * map.grid.rows().0 + row)]
                    [Col(usize::from(lcol) * map.grid.cols().0 + col)] = val;
            }
        }
    }
    let large_map = Map { grid: large_grid };
    Ok(large_map
        .dijkstra(Pos(Row(0), Col(0)), |v| {
            v == Pos(large_map.grid.rows() - 1, large_map.grid.cols() - 1)
        })
        .unwrap()
        .0)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2021, 15).unwrap()).unwrap()).unwrap(),
        441
    );
    assert_eq!(
        part2(parse(parse::data(2021, 15).unwrap()).unwrap()).unwrap(),
        2849
    );
}
