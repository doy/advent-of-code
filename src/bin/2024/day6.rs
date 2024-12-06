use advent_of_code::prelude::*;

pub struct Map {
    grid: Grid<bool>,
    guard: (Row, Col),
}

fn run(
    grid: &Grid<bool>,
    mut guard: (Row, Col),
) -> Option<HashSet<(Row, Col, Direction)>> {
    let mut seen: HashSet<(Row, Col, Direction)> = HashSet::new();
    let mut direction = Direction::Up;
    for _ in 0..(4 * grid.rows().0 * grid.cols().0) {
        let cur = (guard.0, guard.1, direction);
        if seen.contains(&cur) {
            break;
        }
        seen.insert(cur);
        match direction {
            Direction::Up => {
                if guard.0 == Row(0) {
                    return Some(seen);
                }
                let mut next = guard;
                next.0 .0 -= 1;
                if grid[next.0][next.1] {
                    guard = next;
                } else {
                    direction = direction.turn_right();
                }
            }
            Direction::Down => {
                if guard.0 == Row(grid.rows().0 - 1) {
                    return Some(seen);
                }
                let mut next = guard;
                next.0 .0 += 1;
                if grid[next.0][next.1] {
                    guard = next;
                } else {
                    direction = direction.turn_right();
                }
            }
            Direction::Left => {
                if guard.1 == Col(0) {
                    return Some(seen);
                }
                let mut next = guard;
                next.1 .0 -= 1;
                if grid[next.0][next.1] {
                    guard = next;
                } else {
                    direction = direction.turn_right();
                }
            }
            Direction::Right => {
                if guard.1 == Col(grid.cols().0 - 1) {
                    return Some(seen);
                }
                let mut next = guard;
                next.1 .0 += 1;
                if grid[next.0][next.1] {
                    guard = next;
                } else {
                    direction = direction.turn_right();
                }
            }
        }
    }
    None
}

pub fn parse(fh: File) -> Result<Map> {
    let mut guard = (Row(0), Col(0));
    let grid = parse::grid(parse::lines(fh), |c, row, col| match c {
        b'#' => false,
        b'.' => true,
        b'^' => {
            guard = (row, col);
            true
        }
        _ => unreachable!(),
    });
    Ok(Map { grid, guard })
}

pub fn part1(map: Map) -> Result<i64> {
    Ok(run(&map.grid, map.guard)
        .unwrap()
        .into_iter()
        .map(|(row, col, _)| (row, col))
        .collect::<HashSet<(Row, Col)>>()
        .len()
        .try_into()
        .unwrap())
}

pub fn part2(map: Map) -> Result<i64> {
    let mut total = 0;
    let possible: HashSet<(Row, Col)> = run(&map.grid, map.guard)
        .unwrap()
        .into_iter()
        .map(|(row, col, _)| (row, col))
        .collect();
    for (row, col) in possible {
        let mut grid = map.grid.clone();
        grid[row][col] = false;
        if run(&grid, map.guard).is_none() {
            total += 1;
        }
    }
    Ok(total)
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
