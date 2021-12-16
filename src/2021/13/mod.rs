#![allow(clippy::collapsible_else_if)]
#![allow(clippy::comparison_chain)]

struct Map {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
}

impl Map {
    fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            cells: vec![],
        }
    }

    fn resize(&mut self, width: usize, height: usize) {
        self.cells.resize_with(height, std::vec::Vec::new);
        for row in &mut self.cells {
            row.resize(width, false);
        }
        self.width = width;
        self.height = height;
    }

    fn set(&mut self, x: usize, y: usize) {
        self.resize((x + 1).max(self.width), (y + 1).max(self.height));
        self.cells[y][x] = true;
    }

    fn fold(&mut self, horizontal: bool, coord: usize) {
        let mut clone = Self::new();
        if horizontal {
            clone.resize(coord.max(self.width - coord - 1), self.height);
            for (j, row) in self.cells.iter().enumerate() {
                for (i, cell) in row.iter().enumerate() {
                    if *cell {
                        if coord > self.width - coord - 1 {
                            if i < coord {
                                clone.set(i, j);
                            } else if i > coord {
                                clone.set(coord * 2 - i, j);
                            }
                        } else {
                            if i < coord {
                                clone.set(self.width - coord * 2 - 1 + i, j);
                            } else if i > coord {
                                clone.set(self.width - i - 1, j);
                            }
                        }
                    }
                }
            }
        } else {
            clone.resize(self.width, coord.max(self.height - coord - 1));
            for (j, row) in self.cells.iter().enumerate() {
                for (i, cell) in row.iter().enumerate() {
                    if *cell {
                        if coord > self.height - coord - 1 {
                            if j < coord {
                                clone.set(i, j);
                            } else if j > coord {
                                clone.set(i, coord * 2 - j);
                            }
                        } else {
                            if j < coord {
                                clone.set(i, self.height - coord * 2 - 1 + j);
                            } else if j > coord {
                                clone.set(i, self.height - j - 1);
                            }
                        }
                    }
                }
            }
        }
        *self = clone;
    }

    fn total(&self) -> i64 {
        let mut total = 0;
        for row in &self.cells {
            for cell in row {
                if *cell {
                    total += 1;
                }
            }
        }
        total
    }

    fn print(&self) {
        for row in &self.cells {
            for cell in row {
                if *cell {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

pub fn part1() -> anyhow::Result<i64> {
    let mut map = Map::new();
    let mut folds = vec![];
    for line in data_lines!()? {
        if line.is_empty() {
            continue;
        }
        if let Some(fold) = line.strip_prefix("fold along ") {
            let mut fold = fold.split('=');
            let horizontal = fold.next().unwrap() == "x";
            let coord: usize = fold.next().unwrap().parse()?;
            folds.push((horizontal, coord));
        } else {
            let mut coords = line.split(',');
            let x: usize = coords.next().unwrap().parse()?;
            let y: usize = coords.next().unwrap().parse()?;
            map.set(x, y);
        }
    }

    map.fold(folds[0].0, folds[0].1);
    Ok(map.total())
}

pub fn part2() -> anyhow::Result<i64> {
    let mut map = Map::new();
    let mut folds = vec![];
    for line in data_lines!()? {
        if line.is_empty() {
            continue;
        }
        if let Some(fold) = line.strip_prefix("fold along ") {
            let mut fold = fold.split('=');
            let horizontal = fold.next().unwrap() == "x";
            let coord: usize = fold.next().unwrap().parse()?;
            folds.push((horizontal, coord));
        } else {
            let mut coords = line.split(',');
            let x: usize = coords.next().unwrap().parse()?;
            let y: usize = coords.next().unwrap().parse()?;
            map.set(x, y);
        }
    }

    for fold in folds {
        map.fold(fold.0, fold.1);
    }

    map.print();
    Ok(map.total())
}

#[test]
fn test() {
    assert_eq!(part1().unwrap(), 678);
    assert_eq!(part2().unwrap(), 95);
}
