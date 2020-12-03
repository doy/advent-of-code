use anyhow::Context as _;
use std::io::Read as _;

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
    ) -> anyhow::Result<usize> {
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

pub fn part1() -> anyhow::Result<()> {
    let map = read_map()?;
    println!("{}", map.trees_for_slope(3, 1)?);
    Ok(())
}

pub fn part2() -> anyhow::Result<()> {
    let map = read_map()?;
    println!(
        "{}",
        map.trees_for_slope(1, 1)?
            * map.trees_for_slope(3, 1)?
            * map.trees_for_slope(5, 1)?
            * map.trees_for_slope(7, 1)?
            * map.trees_for_slope(1, 2)?
    );
    Ok(())
}

fn read_map() -> anyhow::Result<Map> {
    let mut f = std::fs::File::open("data/3.txt")
        .context("couldn't find data file 3.txt")?;
    let mut map_str = vec![];
    f.read_to_end(&mut map_str)
        .context("failed to read map contents")?;
    Map::parse(&map_str)
}
