use anyhow::Context as _;

struct Map {
    grid: Vec<Vec<bool>>,
}

impl Map {
    fn parse(s: &[u8]) -> anyhow::Result<Self> {
        let mut grid = vec![];
        let mut current_row = vec![];
        for c in s {
            match c {
                b'#' => {
                    current_row.push(true);
                }
                b'.' => {
                    current_row.push(false);
                }
                b'\n' => {
                    grid.push(current_row);
                    current_row = vec![];
                }
                _ => {
                    return Err(anyhow::anyhow!("invalid map char: '{}'", c));
                }
            }
        }
        if !current_row.is_empty() {
            grid.push(current_row);
        }
        Ok(Self { grid })
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
    let map = read_map()?;
    map.trees_for_slope(3, 1)
}

pub fn part2() -> anyhow::Result<i64> {
    let map = read_map()?;
    Ok(map.trees_for_slope(1, 1)?
        * map.trees_for_slope(3, 1)?
        * map.trees_for_slope(5, 1)?
        * map.trees_for_slope(7, 1)?
        * map.trees_for_slope(1, 2)?)
}

fn read_map() -> anyhow::Result<Map> {
    Map::parse(&data_bytes!()?)
}

#[test]
fn test() {
    assert_eq!(part1().unwrap(), 292);
    assert_eq!(part2().unwrap(), 9354744432);
}
