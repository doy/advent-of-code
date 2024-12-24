use advent_of_code::prelude::*;

pub struct Map {
    grid: Grid<bool>,
    guard: Pos,
}

fn run(
    grid: &Grid<bool>,
    mut guard: Pos,
    obstacle: Option<Pos>,
) -> Option<impl Iterator<Item = Pos>> {
    let mut seen: Grid<[bool; 4]> = Grid::default();
    seen.grow(grid.size());
    let mut direction = Direction::Up;
    loop {
        if seen[guard][direction as usize] {
            return None;
        }
        seen[guard][direction as usize] = true;
        if let Some(next) = direction.move_checked(guard, grid.size()) {
            if grid[next] && obstacle != Some(next) {
                guard = next;
            } else {
                direction = direction.turn_right();
            }
        } else {
            return Some(seen.into_indexed_cells().filter_map(
                |(pos, dirs)| dirs.iter().any(|b| *b).then_some(pos),
            ));
        }
    }
}

pub fn parse(fh: File) -> Result<Map> {
    let mut guard = Pos(Row(0), Col(0));
    let grid = parse::grid(parse::raw_lines(fh), |c, pos| match c {
        b'#' => false,
        b'.' => true,
        b'^' => {
            guard = pos;
            true
        }
        _ => unreachable!(),
    });
    Ok(Map { grid, guard })
}

pub fn part1(map: Map) -> Result<i64> {
    Ok(run(&map.grid, map.guard, None)
        .unwrap()
        .count()
        .try_into()
        .unwrap())
}

pub fn part2(map: Map) -> Result<i64> {
    let path: Vec<_> = run(&map.grid, map.guard, None).unwrap().collect();
    Ok(path
        .par_iter()
        .copied()
        .map(|pos| {
            if run(&map.grid, map.guard, Some(pos)).is_none() {
                1
            } else {
                0
            }
        })
        .sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 6).unwrap()).unwrap()).unwrap(),
        4964
    );
    assert_eq!(
        part2(parse(parse::data(2024, 6).unwrap()).unwrap()).unwrap(),
        1740
    );
}
