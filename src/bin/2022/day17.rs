#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
}

impl TryFrom<u8> for Direction {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'<' => Ok(Self::Left),
            b'>' => Ok(Self::Right),
            _ => Err(anyhow!("unknown char {}", value)),
        }
    }
}

struct Piece(Vec<(Row, Col)>);
static PIECES: once_cell::sync::Lazy<[Piece; 5]> =
    once_cell::sync::Lazy::new(|| {
        fn p(row: usize, col: usize) -> (Row, Col) {
            (Row(row), Col(col))
        }
        [
            Piece(vec![p(0, 0), p(0, 1), p(0, 2), p(0, 3)]),
            Piece(vec![p(0, 1), p(1, 0), p(1, 1), p(1, 2), p(2, 1)]),
            Piece(vec![p(0, 0), p(0, 1), p(0, 2), p(1, 2), p(2, 2)]),
            Piece(vec![p(0, 0), p(1, 0), p(2, 0), p(3, 0)]),
            Piece(vec![p(0, 0), p(0, 1), p(1, 0), p(1, 1)]),
        ]
    });

#[derive(Default)]
struct Chamber {
    grid: Grid<bool>,
    piece_pos: Option<(Row, Col)>,
    piece_idx: usize,
    extra_height: usize,
    seen: HashMap<u64, usize>,
    heights: Vec<usize>,
}

impl std::fmt::Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let absolute_pos = if let Some(piece_pos) = self.piece_pos {
            self.piece()
                .0
                .iter()
                .map(|(row, col)| {
                    (*row + piece_pos.0 .0, *col + piece_pos.1 .0)
                })
                .collect()
        } else {
            vec![]
        };
        for row in self
            .grid
            .each_row()
            .take(
                absolute_pos
                    .iter()
                    .copied()
                    .map(|(row, _)| row.0)
                    .chain(std::iter::once(self.height()))
                    .max()
                    .unwrap_or(0)
                    + 1,
            )
            .rev()
        {
            write!(f, "|")?;
            for col in self.grid.each_col() {
                if absolute_pos.iter().any(|(piece_row, piece_col)| {
                    row == *piece_row && col == *piece_col
                }) {
                    write!(f, "@")?;
                } else if self.grid[row][col] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "|")?;
        }
        write!(f, "|-------|")?;
        Ok(())
    }
}

impl Chamber {
    fn step(
        &mut self,
        idx: usize,
        direction: Direction,
    ) -> Option<(usize, Option<usize>)> {
        if self.piece_pos.is_none() {
            let pos = (Row(self.height() + 3), Col(2));
            self.grid.grow(pos.0 + 4, Col(7));
            self.piece_pos = Some(pos);
        }
        let (mut row, mut col) = self.piece_pos.unwrap();

        // println!("{}/{:?}", self.height(), direction);
        // println!("{}", self);
        // println!("*********");
        let new_col = Col(match direction {
            Direction::Left => col.0.saturating_sub(1),
            Direction::Right => col.0.saturating_add(1),
        });
        let piece = self.piece();
        if !self.collides(piece, (row, new_col)) {
            col = new_col;
            self.piece_pos = Some((row, col))
        }

        // println!("{}/Down", self.height());
        // println!("{}", self);
        // println!("*********");
        let collides = if row > Row(0) {
            self.collides(piece, (row - 1, col))
        } else {
            true
        };
        if collides {
            self.apply(piece, (row, col));
            self.shrink();
            self.heights.push(self.height() + self.extra_height);
            let prev = self.seen_grid_hash(idx);
            self.piece_pos = None;
            self.piece_idx += 1;
            Some((self.piece_idx - 1, prev))
        } else {
            row = row - 1;
            self.piece_pos = Some((row, col));
            None
        }
    }

    fn piece(&self) -> &'static Piece {
        &PIECES[self.piece_idx % PIECES.len()]
    }

    fn collides(&self, piece: &Piece, piece_pos: (Row, Col)) -> bool {
        for pos in &piece.0 {
            let absolute_pos =
                (piece_pos.0 + pos.0 .0, piece_pos.1 + pos.1 .0);
            if absolute_pos.1 >= Col(7) {
                return true;
            }
            if self.grid[absolute_pos.0][absolute_pos.1] {
                return true;
            }
        }
        false
    }

    fn apply(&mut self, piece: &Piece, piece_pos: (Row, Col)) {
        for pos in &piece.0 {
            let absolute_pos =
                (piece_pos.0 + pos.0 .0, piece_pos.1 + pos.1 .0);
            assert!(!self.grid[absolute_pos.0][absolute_pos.1]);
            self.grid[absolute_pos.0][absolute_pos.1] = true;
        }
    }

    fn shrink(&mut self) {
        let height = self
            .grid
            .each_col()
            .map(|col| {
                self.grid
                    .each_row()
                    .rev()
                    .find(|row| self.grid[*row][col])
                    .map(|row| row.0 + 1)
                    .unwrap_or(0)
            })
            .min()
            .unwrap_or(0);
        self.grid.unshift_rows(height);
        self.extra_height += height;
    }

    fn seen_grid_hash(&mut self, idx: usize) -> Option<usize> {
        use std::hash::{Hash as _, Hasher as _};
        let mut hasher = ahash::AHasher::default();
        self.grid.hash(&mut hasher);
        idx.hash(&mut hasher);
        let hash = hasher.finish();
        let ret = self.seen.get(&hash).copied();
        self.seen.insert(hash, self.piece_idx);
        ret
    }

    fn height(&self) -> usize {
        self.grid
            .each_row()
            .rev()
            .find(|row| self.grid[*row].iter().any(|c| *c))
            .map(|row| row.0 + 1)
            .unwrap_or(0)
    }
}

pub fn parse(fh: File) -> Result<Vec<Direction>> {
    Ok(parse::bytes(fh)
        .take_while(|c| *c == b'<' || *c == b'>')
        .map(|c| c.try_into().unwrap())
        .collect())
}

pub fn part1(directions: Vec<Direction>) -> Result<usize> {
    let mut chamber = Chamber::default();
    for (i, direction) in directions.iter().copied().enumerate().cycle() {
        if let Some((dropped, prev)) = chamber.step(i, direction) {
            if dropped + 1 >= 2022 {
                break;
            }
        }
    }
    Ok(chamber.height() + chamber.extra_height)
}

pub fn part2(directions: Vec<Direction>) -> Result<usize> {
    let mut chamber = Chamber::default();
    for (i, direction) in directions.iter().copied().enumerate().cycle() {
        if let Some((dropped, Some(prev))) = chamber.step(i, direction) {
            let cycle_len = dropped - prev;
            let height_diff =
                chamber.heights[dropped] - chamber.heights[prev];
            let start_height = chamber.heights[prev];
            let rest = 1_000_000_000_000 - (prev + 1);
            let remaining_full_cycles = rest / cycle_len;
            let partial_cycle_len = rest - remaining_full_cycles * cycle_len;
            let partial_height_diff = chamber.heights
                [prev + partial_cycle_len]
                - chamber.heights[prev];
            let height = start_height
                + height_diff * remaining_full_cycles
                + partial_height_diff;
            return Ok(height);
        }
    }
    unreachable!()
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 17).unwrap()).unwrap()).unwrap(),
        3083
    );
    assert_eq!(
        part2(parse(parse::data(2022, 17).unwrap()).unwrap()).unwrap(),
        1532183908048
    );
}
