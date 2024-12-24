use advent_of_code::prelude::*;

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
            .grow(Size(Row((y1 + 1).max(y2 + 1)), Col((x1 + 1).max(x2 + 1))));
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
            .grow(Size(Row((y1 + 1).max(y2 + 1)), Col((x1 + 1).max(x2 + 1))));
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

pub fn parse(fh: File) -> Result<impl Iterator<Item = Vec<usize>>> {
    Ok(parse::raw_lines(fh).map(move |line| {
        regex_captures!(r"^([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)$", &line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|s| s.unwrap().as_str().parse())
            .collect::<Result<_, _>>()
            .unwrap()
    }))
}

pub fn part1(coords: impl Iterator<Item = Vec<usize>>) -> Result<usize> {
    let mut map = Map::default();
    for nums in coords {
        let _ = map.mark_horizontal(nums[0], nums[1], nums[2], nums[3])
            || map.mark_vertical(nums[0], nums[1], nums[2], nums[3]);
    }
    Ok(map.count_overlapping())
}

pub fn part2(coords: impl Iterator<Item = Vec<usize>>) -> Result<usize> {
    let mut map = Map::default();
    for nums in coords {
        if map.mark_horizontal(nums[0], nums[1], nums[2], nums[3]) {
            continue;
        }
        if map.mark_vertical(nums[0], nums[1], nums[2], nums[3]) {
            continue;
        }
        if map.mark_diagonal(nums[0], nums[1], nums[2], nums[3]) {
            continue;
        }
        unreachable!();
    }
    Ok(map.count_overlapping())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2021, 5).unwrap()).unwrap()).unwrap(),
        6311
    );
    assert_eq!(
        part2(parse(parse::data(2021, 5).unwrap()).unwrap()).unwrap(),
        19929
    );
}
