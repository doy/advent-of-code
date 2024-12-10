use advent_of_code::prelude::*;

pub struct Map {
    grid: Grid<bool>,
    guard: Pos,
}

fn run(
    grid: &Grid<bool>,
    mut guard: Pos,
) -> Option<HashSet<(Pos, Direction)>> {
    let mut seen: HashSet<(Pos, Direction)> = HashSet::new();
    let mut direction = Direction::Up;
    loop {
        let cur = (guard, direction);
        if seen.contains(&cur) {
            return None;
        }
        seen.insert(cur);
        if let Some(next) = direction.move_checked(guard, grid.size()) {
            if grid[next] {
                guard = next;
            } else {
                direction = direction.turn_right();
            }
        } else {
            return Some(seen);
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
    Ok(run(&map.grid, map.guard)
        .unwrap()
        .into_iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<Pos>>()
        .len()
        .try_into()
        .unwrap())
}

pub fn part2(map: Map) -> Result<i64> {
    Ok(run(&map.grid, map.guard)
        .unwrap()
        .into_iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<Pos>>()
        .par_iter()
        .copied()
        .map(|pos| {
            let mut grid = map.grid.clone();
            grid[pos] = false;
            if run(&grid, map.guard).is_none() {
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
