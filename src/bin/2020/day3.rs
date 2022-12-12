use advent_of_code::prelude::*;

pub struct Map {
    grid: Grid<bool>,
}

impl Map {
    fn new(grid: Grid<bool>) -> Self {
        Self { grid }
    }

    fn rows(&self) -> usize {
        self.grid.rows().0
    }

    fn tree_at(&self, row: Row, col: Col) -> Result<bool> {
        // unwrap safe because cycle().nth() can never fail
        Ok(*self.grid[row].iter().cycle().nth(col.0).unwrap())
    }

    fn trees_for_slope(
        &self,
        row_incr: usize,
        col_incr: usize,
    ) -> Result<u64> {
        let mut trees = 0;
        for r in 0..self.rows() / row_incr {
            let row = r * row_incr;
            let col = r * col_incr;
            if self.tree_at(Row(row), Col(col))? {
                trees += 1;
            }
        }
        Ok(trees)
    }
}

pub fn parse(fh: File) -> Result<Map> {
    Ok(Map::new(parse::bool_grid(parse::raw_lines(fh), b'#', b'.')))
}

pub fn part1(map: Map) -> Result<u64> {
    map.trees_for_slope(1, 3)
}

pub fn part2(map: Map) -> Result<u64> {
    Ok(map.trees_for_slope(1, 1)?
        * map.trees_for_slope(1, 3)?
        * map.trees_for_slope(1, 5)?
        * map.trees_for_slope(1, 7)?
        * map.trees_for_slope(2, 1)?)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2020, 3).unwrap()).unwrap()).unwrap(),
        292
    );
    assert_eq!(
        part2(parse(parse::data(2020, 3).unwrap()).unwrap()).unwrap(),
        9354744432
    );
}
