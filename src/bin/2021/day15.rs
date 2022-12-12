use advent_of_code::prelude::*;

pub struct Map {
    grid: Grid<u8>,
}

impl advent_of_code::graph::Graph<(Row, Col), (Row, Col)> for Map {
    type Edges = advent_of_code::grid::Adjacent;

    fn edges(&self, v: (Row, Col)) -> Self::Edges {
        self.grid.adjacent(v.0, v.1, false)
    }

    fn edge(&self, _v: (Row, Col), e: (Row, Col)) -> ((Row, Col), u64) {
        (e, u64::from(self.grid[e.0][e.1]))
    }
}

pub fn parse(fh: File) -> Result<Map> {
    Ok(Map {
        grid: parse::digit_grid(parse::raw_lines(fh)),
    })
}

pub fn part1(map: Map) -> Result<u64> {
    Ok(map
        .dijkstra(
            (Row(0), Col(0)),
            (map.grid.rows() - 1, map.grid.cols() - 1),
        )
        .0)
}

pub fn part2(map: Map) -> Result<u64> {
    let mut large_grid = Grid::default();
    large_grid.grow(Row(map.grid.rows().0 * 5), Col(map.grid.cols().0 * 5));
    for lrow in 0..5 {
        for lcol in 0..5 {
            for ((Row(row), Col(col)), val) in map.grid.indexed_cells() {
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
        .dijkstra(
            (Row(0), Col(0)),
            (large_map.grid.rows() - 1, large_map.grid.cols() - 1),
        )
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
