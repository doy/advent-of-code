use advent_of_code::prelude::*;

pub struct Map {
    grid: Grid<bool>,
    generator: (Row, Col),
    abyss: Row,
    sand_count: usize,
}

impl Map {
    fn filled(&self, pos: (Row, Col), floor: bool) -> bool {
        self.grid[pos.0][pos.1] || (floor && (pos.0 > self.abyss))
    }

    fn fill(&mut self, pos: (Row, Col)) {
        self.grid[pos.0][pos.1] = true;
    }

    fn drop(&mut self, floor: bool) -> bool {
        let mut pos = self.generator;
        if self.filled(pos, floor) {
            if floor {
                return true;
            } else {
                panic!("generator filled but no floor");
            }
        }
        loop {
            if !floor && pos.0 + 1 >= self.abyss {
                return true;
            }
            let mut can_fall = false;
            for next in [
                (pos.0 + 1, pos.1),
                (pos.0 + 1, pos.1 - 1),
                (pos.0 + 1, pos.1 + 1),
            ] {
                if !self.filled(next, floor) {
                    pos = next;
                    can_fall = true;
                    break;
                }
            }
            if !can_fall {
                break;
            }
        }
        self.fill(pos);
        self.sand_count += 1;
        false
    }
}

pub fn parse(fh: File) -> Result<Map> {
    let mut grid = Grid::default();
    for line in parse::raw_lines(fh) {
        let coords: Vec<_> = line
            .split(" -> ")
            .map(|pos| {
                let mut parts = pos.split(',');
                let col = parts.next().unwrap();
                let row = parts.next().unwrap();
                Pos(Row(row.parse().unwrap()), Col(col.parse().unwrap()))
            })
            .collect();
        for pair in coords.windows(2) {
            let pos = pair[0];
            let next_pos = pair[1];
            grid.grow(Size(pos.0 + 1, pos.1 + 1));
            grid.grow(Size(next_pos.0 + 1, next_pos.1 + 1));
            if pos.0 == next_pos.0 {
                for col in (pos.1.0.min(next_pos.1.0)
                    ..=pos.1.0.max(next_pos.1.0))
                    .map(Col)
                {
                    grid[pos.0][col] = true;
                }
            } else if pos.1 == next_pos.1 {
                for row in (pos.0.0.min(next_pos.0.0)
                    ..=pos.0.0.max(next_pos.0.0))
                    .map(Row)
                {
                    grid[row][pos.1] = true;
                }
            } else {
                panic!("diagonal line?");
            }
        }
    }
    let abyss = grid.rows();
    grid.grow(Size(
        grid.rows() + grid.rows().0,
        grid.cols() + grid.cols().0,
    ));
    Ok(Map {
        grid,
        generator: (Row(0), Col(500)),
        abyss,
        sand_count: 0,
    })
}

pub fn part1(mut map: Map) -> Result<usize> {
    loop {
        let abyss = map.drop(false);
        if abyss {
            return Ok(map.sand_count);
        }
    }
}

pub fn part2(mut map: Map) -> Result<usize> {
    loop {
        let blocked = map.drop(true);
        if blocked {
            return Ok(map.sand_count);
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 14).unwrap()).unwrap()).unwrap(),
        737
    );
    assert_eq!(
        part2(parse(parse::data(2022, 14).unwrap()).unwrap()).unwrap(),
        28145
    );
}
