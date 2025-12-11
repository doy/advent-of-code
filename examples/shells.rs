/// shell puzzle from really neat gimmicks level 9
use advent_of_code::prelude::*;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Shell {
    Red,
    Green,
    Blue,
    Yellow,
    None,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Puzzle {
    cells: [[Shell; 3]; 3],
}

impl Puzzle {
    fn at(&self, pos: Pos) -> Shell {
        self.cells[pos.0.0][pos.1.0]
    }

    fn set_at(&mut self, pos: Pos, val: Shell) {
        self.cells[pos.0.0][pos.1.0] = val
    }

    fn empty(&self) -> Pos {
        for row in 0..=2 {
            for col in 0..=2 {
                if self.cells[row][col] == Shell::None {
                    return Pos(Row(row), Col(col));
                }
            }
        }
        panic!("empty not found");
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..=2 {
            for col in 0..=2 {
                match self.cells[row][col] {
                    Shell::Red => write!(f, "R")?,
                    Shell::Green => write!(f, "G")?,
                    Shell::Blue => write!(f, "B")?,
                    Shell::Yellow => write!(f, "Y")?,
                    Shell::None => write!(f, " ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Solver;

impl advent_of_code::graph::Graph<Puzzle, Pos> for Solver {
    fn edges(&self, v: Puzzle) -> impl IntoIterator<Item = Pos> {
        let empty = v.empty();
        empty
            .adjacent(Size(Row(3), Col(3)), false)
            .filter(move |pos| !(empty.1 == Col(1) && pos.1 == Col(1)))
    }

    fn edge(&self, mut v: Puzzle, e: Pos) -> (Puzzle, u64) {
        v.set_at(v.empty(), v.at(e));
        v.set_at(e, Shell::None);
        (v, 1)
    }
}

fn main() {
    let start = Puzzle {
        cells: [
            // this is randomized
            [Shell::Red, Shell::Green, Shell::Red],
            [Shell::Yellow, Shell::None, Shell::Green],
            [Shell::Yellow, Shell::Blue, Shell::Blue],
        ],
    };
    let end = Puzzle {
        cells: [
            [Shell::Blue, Shell::Red, Shell::Yellow],
            [Shell::Green, Shell::None, Shell::Green],
            [Shell::Blue, Shell::Red, Shell::Yellow],
        ],
    };
    let (len, path) = Solver.dijkstra(start, |state| state == end).unwrap();
    println!("{len} moves:");
    for state in path {
        println!("{state}");
    }
}
