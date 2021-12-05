struct Map {
    points: Vec<Vec<i64>>,
}

impl Map {
    fn new() -> Self {
        Self { points: vec![] }
    }

    fn resize(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.points.resize_with(
            (x1 + 1).max(x2 + 1).max(self.points.len()),
            Vec::new,
        );
        for col in &mut self.points {
            col.resize((y1 + 1).max(y2 + 1).max(col.len()), 0);
        }
    }

    fn mark_horizontal(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    ) -> bool {
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                self.points[x1][y] += 1;
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
        if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                self.points[x][y1] += 1;
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
                    self.points[x1.min(x2) + i][y1.min(y2) + i] += 1;
                } else if x1 > x2 {
                    self.points[x2 + i][y2 - i] += 1;
                } else {
                    self.points[x1 + i][y1 - i] += 1;
                }
            }
            true
        } else {
            false
        }
    }

    fn count_overlapping(&self) -> usize {
        self.points
            .iter()
            .flat_map(|v| v.iter())
            .filter(|x| **x >= 2)
            .count()
    }
}

impl std::fmt::Display for Map {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        for row in &self.points {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part1() -> anyhow::Result<i64> {
    let rx = regex::Regex::new("^(\\d+),(\\d+) -> (\\d+),(\\d+)$").unwrap();
    let mut map = Map::new();
    for line in data_lines!()? {
        let line = line?;
        let nums: Vec<usize> = rx
            .captures(&line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|s| s.unwrap().as_str().parse())
            .collect::<Result<_, _>>()?;
        map.resize(nums[0], nums[1], nums[2], nums[3]);
        let _ = map.mark_horizontal(nums[0], nums[1], nums[2], nums[3])
            || map.mark_vertical(nums[0], nums[1], nums[2], nums[3]);
    }
    Ok(map.count_overlapping().try_into()?)
}

pub fn part2() -> anyhow::Result<i64> {
    let rx = regex::Regex::new("^(\\d+),(\\d+) -> (\\d+),(\\d+)$").unwrap();
    let mut map = Map::new();
    for line in data_lines!()? {
        let line = line?;
        let nums: Vec<usize> = rx
            .captures(&line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|s| s.unwrap().as_str().parse())
            .collect::<Result<_, _>>()?;
        map.resize(nums[0], nums[1], nums[2], nums[3]);
        let _ = map.mark_horizontal(nums[0], nums[1], nums[2], nums[3])
            || map.mark_vertical(nums[0], nums[1], nums[2], nums[3])
            || map.mark_diagonal(nums[0], nums[1], nums[2], nums[3])
            || unreachable!();
    }
    Ok(map.count_overlapping().try_into()?)
}
