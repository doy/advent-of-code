use advent_of_code::prelude::*;

#[derive(Debug)]
pub struct Move {
    dir: Direction,
    count: usize,
}

impl Move {
    fn parse(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let dir = parts.next().unwrap().parse().unwrap();
        let count = parts.next().unwrap().parse().unwrap();
        Self { dir, count }
    }
}

pub struct Rope {
    knots: Vec<(Row, Col)>,
}

impl Rope {
    pub fn new(len: usize) -> Self {
        Self {
            knots: vec![Default::default(); len],
        }
    }

    pub fn at(&self, pos: (Row, Col)) -> Option<usize> {
        for (i, knot) in self.knots.iter().enumerate() {
            if knot == &pos {
                return Some(i);
            }
        }
        None
    }
}

pub struct Map {
    grid: std::collections::VecDeque<std::collections::VecDeque<bool>>,
    size: (Row, Col),
    rope: Rope,
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "({}, {})", self.size.0 .0, self.size.1 .0)?;
        for row in (0..self.size.0 .0).rev() {
            for col in 0..self.size.1 .0 {
                write!(
                    f,
                    "{}",
                    if let Some(idx) = self.rope.at((Row(row), Col(col))) {
                        char::from(b'0' + u8::try_from(idx).unwrap())
                    } else if self.grid[row][col] {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn new(len: usize) -> Self {
        let mut grid = std::collections::VecDeque::new();
        grid.push_back(std::collections::VecDeque::new());
        grid[0].push_back(true);
        Self {
            grid,
            size: (Row(1), Col(1)),
            rope: Rope::new(len),
        }
    }

    fn mv(&mut self, mv: &Move) {
        // println!("{:?}", mv);
        for _ in 0..mv.count {
            self.step(&mv.dir);
        }
    }

    fn step(&mut self, dir: &Direction) {
        let (Row(row), Col(col)) = self.rope.knots[0];
        match dir {
            Direction::Up => {
                if row == self.size.0 .0 - 1 {
                    let mut row_contents = std::collections::VecDeque::new();
                    row_contents
                        .resize_with(self.size.1 .0, Default::default);
                    self.grid.push_back(row_contents);
                    self.size.0 = Row(self.size.0 .0 + 1);
                }
                self.rope.knots[0].0 = Row(self.rope.knots[0].0 .0 + 1);
            }
            Direction::Down => {
                if row == 0 {
                    let mut row_contents = std::collections::VecDeque::new();
                    row_contents
                        .resize_with(self.size.1 .0, Default::default);
                    self.grid.push_front(row_contents);
                    for knot in &mut self.rope.knots {
                        knot.0 = Row(knot.0 .0 + 1);
                    }
                    self.size.0 = Row(self.size.0 .0 + 1);
                }
                self.rope.knots[0].0 = Row(self.rope.knots[0].0 .0 - 1);
            }
            Direction::Left => {
                if col == 0 {
                    for i in 0..self.size.0 .0 {
                        self.grid[i].push_front(Default::default());
                    }
                    for knot in &mut self.rope.knots {
                        knot.1 = Col(knot.1 .0 + 1);
                    }
                    self.size.1 = Col(self.size.1 .0 + 1);
                }
                self.rope.knots[0].1 = Col(self.rope.knots[0].1 .0 - 1);
            }
            Direction::Right => {
                if col == self.size.1 .0 - 1 {
                    for i in 0..self.size.0 .0 {
                        self.grid[i].push_back(Default::default());
                    }
                    self.size.1 = Col(self.size.1 .0 + 1);
                }
                self.rope.knots[0].1 = Col(self.rope.knots[0].1 .0 + 1);
            }
        }

        for i in 0..(self.rope.knots.len() - 1) {
            if self.rope.knots[i + 1]
                .0
                 .0
                .abs_diff(self.rope.knots[i].0 .0)
                > 1
                || self.rope.knots[i + 1]
                    .1
                     .0
                    .abs_diff(self.rope.knots[i].1 .0)
                    > 1
            {
                if self.rope.knots[i + 1].0 .0 < self.rope.knots[i].0 .0 {
                    self.rope.knots[i + 1].0 .0 += 1;
                }
                if self.rope.knots[i + 1].0 .0 > self.rope.knots[i].0 .0 {
                    self.rope.knots[i + 1].0 .0 -= 1;
                }
                if self.rope.knots[i + 1].1 .0 < self.rope.knots[i].1 .0 {
                    self.rope.knots[i + 1].1 .0 += 1;
                }
                if self.rope.knots[i + 1].1 .0 > self.rope.knots[i].1 .0 {
                    self.rope.knots[i + 1].1 .0 -= 1;
                }
            }
        }
        self.grid[self.rope.knots.last().unwrap().0 .0]
            [self.rope.knots.last().unwrap().1 .0] = true;
    }
}

pub fn parse(fh: File) -> Result<impl Iterator<Item = Move>> {
    Ok(parse::raw_lines(fh).map(|line| Move::parse(&line)))
}

pub fn part1(moves: impl Iterator<Item = Move>) -> Result<usize> {
    let mut map = Map::new(2);
    for mv in moves {
        // println!("{:?}", map);
        map.mv(&mv);
    }
    // println!("{:?}", map);
    Ok(map
        .grid
        .iter()
        .flat_map(|row| row.iter().copied())
        .filter(|cell| *cell)
        .count())
}

pub fn part2(moves: impl Iterator<Item = Move>) -> Result<usize> {
    let mut map = Map::new(10);
    for mv in moves {
        // println!("{:?}", map);
        map.mv(&mv);
    }
    // println!("{:?}", map);
    Ok(map
        .grid
        .iter()
        .flat_map(|row| row.iter().copied())
        .filter(|cell| *cell)
        .count())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 9).unwrap()).unwrap()).unwrap(),
        6037
    );
    assert_eq!(
        part2(parse(parse::data(2022, 9).unwrap()).unwrap()).unwrap(),
        2485
    );
}
