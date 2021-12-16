use anyhow::Context as _;

struct Map {
    grid: Vec<Vec<bool>>,
}

impl Map {
    fn new(grid: Vec<Vec<bool>>) -> Self {
        Self { grid }
    }

    fn rows(&self) -> usize {
        self.grid.len()
    }

    fn tree_at(&self, x: usize, y: usize) -> anyhow::Result<bool> {
        // unwrap safe because cycle().nth() can never fail
        Ok(*self
            .grid
            .get(y)
            .context("row too large")?
            .iter()
            .cycle()
            .nth(x)
            .unwrap())
    }

    fn trees_for_slope(
        &self,
        x_incr: usize,
        y_incr: usize,
    ) -> anyhow::Result<i64> {
        let mut trees = 0;
        for r in 0..self.rows() / y_incr {
            let x = r * x_incr;
            let y = r * y_incr;
            if self.tree_at(x, y)? {
                trees += 1;
            }
        }
        Ok(trees)
    }
}

pub fn part1() -> anyhow::Result<i64> {
    let map = Map::new(data_bool_map!(b'#', b'.'));
    map.trees_for_slope(3, 1)
}

pub fn part2() -> anyhow::Result<i64> {
    let map = Map::new(data_bool_map!(b'#', b'.'));
    Ok(map.trees_for_slope(1, 1)?
        * map.trees_for_slope(3, 1)?
        * map.trees_for_slope(5, 1)?
        * map.trees_for_slope(7, 1)?
        * map.trees_for_slope(1, 2)?)
}

#[test]
fn test() {
    assert_eq!(part1().unwrap(), 292);
    assert_eq!(part2().unwrap(), 9354744432);
}
