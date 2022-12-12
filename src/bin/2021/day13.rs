use advent_of_code::prelude::*;

#[derive(Default)]
pub struct Paper {
    grid: Grid<bool>,
}

impl Paper {
    fn set(&mut self, row: Row, col: Col) {
        self.grid.grow(Row(row.0 + 1), Col(col.0 + 1));
        self.grid[row][col] = true;
    }

    fn fold(&mut self, horizontal: bool, coord: usize) {
        let mut clone = Self::default();
        if horizontal {
            clone.grid.grow(
                self.grid.rows(),
                Col(coord.max(self.grid.cols().0 - coord - 1)),
            );
            for ((Row(row), Col(col)), cell) in self.grid.indexed_cells() {
                if *cell {
                    if coord > self.grid.cols().0 - coord - 1 {
                        if col < coord {
                            clone.set(Row(row), Col(col));
                        } else if col > coord {
                            clone.set(Row(row), Col(coord * 2 - col));
                        }
                    } else {
                        if col < coord {
                            clone.set(
                                Row(row),
                                Col(self.grid.cols().0 - coord * 2 - 1 + col),
                            );
                        } else if col > coord {
                            clone.set(
                                Row(row),
                                Col(self.grid.cols().0 - col - 1),
                            );
                        }
                    }
                }
            }
        } else {
            clone.grid.grow(
                Row(coord.max(self.grid.rows().0 - coord - 1)),
                self.grid.cols(),
            );
            for ((Row(row), Col(col)), cell) in self.grid.indexed_cells() {
                if *cell {
                    if coord > self.grid.rows().0 - coord - 1 {
                        if row < coord {
                            clone.set(Row(row), Col(col));
                        } else if row > coord {
                            clone.set(Row(coord * 2 - row), Col(col));
                        }
                    } else {
                        if row < coord {
                            clone.set(
                                Row(self.grid.rows().0 - coord * 2 - 1 + row),
                                Col(col),
                            );
                        } else if row > coord {
                            clone.set(
                                Row(self.grid.rows().0 - row - 1),
                                Col(col),
                            );
                        }
                    }
                }
            }
        }
        *self = clone;
    }

    fn total(&self) -> usize {
        self.grid.cells().filter(|b| **b).count()
    }
}

impl std::fmt::Display for Paper {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            self.grid.display_packed(|b| if *b { '#' } else { '.' })
        )
    }
}

pub fn parse(fh: File) -> Result<(Paper, Vec<(bool, usize)>)> {
    let mut paper = Paper::default();
    let mut folds = vec![];
    for line in parse::raw_lines(fh) {
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
            paper.set(Row(y), Col(x));
        }
    }
    Ok((paper, folds))
}

pub fn part1(
    (mut paper, folds): (Paper, Vec<(bool, usize)>),
) -> Result<usize> {
    paper.fold(folds[0].0, folds[0].1);
    Ok(paper.total())
}

pub fn part2(
    (mut paper, folds): (Paper, Vec<(bool, usize)>),
) -> Result<usize> {
    for fold in folds {
        paper.fold(fold.0, fold.1);
    }

    println!("{}", paper);
    Ok(paper.total())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2021, 13).unwrap()).unwrap()).unwrap(),
        678
    );
    assert_eq!(
        part2(parse(parse::data(2021, 13).unwrap()).unwrap()).unwrap(),
        95
    );
}
