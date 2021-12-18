use crate::util::grid::*;

#[derive(Default, Clone)]
struct Map {
    grid: Grid<i64>,
}

impl Map {
    fn mark_horizontal(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    ) -> bool {
        self.grid
            .grow(Row((y1 + 1).max(y2 + 1)), Col((x1 + 1).max(x2 + 1)));
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                self.grid[Row(y)][Col(x1)] += 1;
            }
            true
        } else {
            false
        }
    }

    fn mark_vertical(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    ) -> bool {
        self.grid
            .grow(Row((y1 + 1).max(y2 + 1)), Col((x1 + 1).max(x2 + 1)));
        if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                self.grid[Row(y1)][Col(x)] += 1;
            }
            true
        } else {
            false
        }
    }

    fn mark_diagonal(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    ) -> bool {
        if x1.max(x2) - x1.min(x2) == y1.max(y2) - y1.min(y2) {
            for i in 0..=(x1.max(x2) - x1.min(x2)) {
                if x1 > x2 && y1 > y2 || x1 < x2 && y1 < y2 {
                    self.grid[Row(y1.min(y2) + i)][Col(x1.min(x2) + i)] += 1;
                } else if x1 > x2 {
                    self.grid[Row(y2 - i)][Col(x2 + i)] += 1;
                } else {
                    self.grid[Row(y1 - i)][Col(x1 + i)] += 1;
                }
            }
            true
        } else {
            false
        }
    }

    fn count_overlapping(&self) -> usize {
        self.grid.cells().filter(|x| **x >= 2).count()
    }
}

pub fn parse(
    fh: std::fs::File,
) -> anyhow::Result<impl Iterator<Item = Vec<usize>>> {
    let rx = regex::Regex::new("^(\\d+),(\\d+) -> (\\d+),(\\d+)$")?;
    Ok(crate::util::parse::lines(fh).map(move |line| {
        rx.captures(&line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|s| s.unwrap().as_str().parse())
            .collect::<Result<_, _>>()
            .unwrap()
    }))
}

pub fn part1(
    coords: impl Iterator<Item = Vec<usize>>,
) -> anyhow::Result<i64> {
    let mut map = Map::default();
    for nums in coords {
        let _ = map.mark_horizontal(nums[0], nums[1], nums[2], nums[3])
            || map.mark_vertical(nums[0], nums[1], nums[2], nums[3]);
    }
    Ok(map.count_overlapping().try_into()?)
}

pub fn part2(
    coords: impl Iterator<Item = Vec<usize>>,
) -> anyhow::Result<i64> {
    let mut map = Map::default();
    for nums in coords {
        let _ = map.mark_horizontal(nums[0], nums[1], nums[2], nums[3])
            || map.mark_vertical(nums[0], nums[1], nums[2], nums[3])
            || map.mark_diagonal(nums[0], nums[1], nums[2], nums[3])
            || unreachable!();
    }
    Ok(map.count_overlapping().try_into()?)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(crate::util::data(2021, 5).unwrap()).unwrap()).unwrap(),
        6311
    );
    assert_eq!(
        part2(parse(crate::util::data(2021, 5).unwrap()).unwrap()).unwrap(),
        19929
    );
}
