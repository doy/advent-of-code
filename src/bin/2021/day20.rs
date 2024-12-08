use advent_of_code::prelude::*;

pub struct Image {
    algorithm: Vec<bool>,
    map: Grid<bool>,
    outer: bool,
}

impl Image {
    fn new(algorithm: Vec<bool>, map: Grid<bool>) -> Self {
        Self {
            algorithm,
            map,
            outer: false,
        }
    }

    fn enhance(&mut self) {
        let mut new_map: Grid<bool> = Grid::default();
        new_map.grow(Size(self.map.rows() + 2, self.map.cols() + 2));
        for row in 0..new_map.rows().0 {
            for col in 0..new_map.cols().0 {
                let neighbors: &[(Option<usize>, Option<usize>)] = &[
                    (row.checked_sub(2), col.checked_sub(2)),
                    (row.checked_sub(2), col.checked_sub(1)),
                    (row.checked_sub(2), Some(col)),
                    (row.checked_sub(1), col.checked_sub(2)),
                    (row.checked_sub(1), col.checked_sub(1)),
                    (row.checked_sub(1), Some(col)),
                    (Some(row), col.checked_sub(2)),
                    (Some(row), col.checked_sub(1)),
                    (Some(row), Some(col)),
                ];
                let neighbors = neighbors.iter().map(|neighbor| {
                    if let (Some(row), Some(col)) = neighbor {
                        self.map
                            .get(Row(*row))
                            .and_then(|row| row.get(Col(*col)).copied())
                            .unwrap_or(self.outer)
                    } else {
                        self.outer
                    }
                });
                let mut idx = 0;
                for neighbor in neighbors {
                    idx = idx * 2 + usize::from(neighbor);
                }
                new_map[Row(row)][Col(col)] = self.algorithm[idx]
            }
        }
        self.map = new_map;
        if self.outer {
            self.outer = self.algorithm[511];
        } else {
            self.outer = self.algorithm[0];
        }
    }

    fn count_true(&self) -> usize {
        if self.outer {
            panic!("infinite");
        }
        self.map.cells().filter(|c| **c).count()
    }
}

pub fn parse(fh: File) -> Result<Image> {
    let mut lines = parse::raw_lines(fh);
    let algorithm = lines.next().unwrap();
    let algorithm: Vec<_> = algorithm
        .as_bytes()
        .iter()
        .map(|b| match b {
            b'#' => true,
            b'.' => false,
            _ => panic!("bad algorithm"),
        })
        .collect();
    lines.next().unwrap();
    let map = parse::bool_grid(lines, b'#', b'.');
    Ok(Image::new(algorithm, map))
}

pub fn part1(mut image: Image) -> Result<usize> {
    for _ in 0..2 {
        image.enhance();
    }
    Ok(image.count_true())
}

pub fn part2(mut image: Image) -> Result<usize> {
    for _ in 0..50 {
        image.enhance();
    }
    Ok(image.count_true())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2021, 20).unwrap()).unwrap()).unwrap(),
        5306
    );
    assert_eq!(
        part2(parse(parse::data(2021, 20).unwrap()).unwrap()).unwrap(),
        17497
    );
}
