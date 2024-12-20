use advent_of_code::prelude::*;

#[derive(Debug)]
pub struct Map {
    grid: Grid<bool>,
    start: Pos,
    end: Pos,
}

impl advent_of_code::graph::Graph<Pos, Pos> for Map {
    type Edges = Vec<Pos>;

    fn edges(&self, v: Pos) -> Self::Edges {
        self.grid
            .adjacent(v, false)
            .filter(|pos| self.grid[*pos])
            .collect()
    }

    fn edge(&self, _: Pos, e: Pos) -> (Pos, u64) {
        (e, 1)
    }
}

pub fn parse(fh: File) -> Result<Map> {
    let mut start = Pos::default();
    let mut end = Pos::default();
    let grid = parse::grid(parse::raw_lines(fh), |c, pos| match c {
        b'.' => true,
        b'#' => false,
        b'S' => {
            start = pos;
            true
        }
        b'E' => {
            end = pos;
            true
        }
        _ => unreachable!(),
    });
    Ok(Map { grid, start, end })
}

pub fn part1(map: Map) -> Result<i64> {
    let forward = map.dijkstra_full(map.start);
    let backward = map.dijkstra_full(map.end);
    let max = forward[&map.end].1;
    let mut total = 0;
    for (pos, passable) in map.grid.indexed_cells() {
        if *passable {
            continue;
        }
        if let (Some(left), Some(right)) = (
            Direction::Left.move_checked(pos, map.grid.size()),
            Direction::Right.move_checked(pos, map.grid.size()),
        ) {
            if map.grid[left] && map.grid[right] {
                let with_shortcut =
                    (forward[&left].1 + 1 + backward[&right].1)
                        .min(forward[&right].1 + 1 + backward[&left].1);
                if max - with_shortcut >= 100 {
                    total += 1;
                }
            }
        }
        if let (Some(up), Some(down)) = (
            Direction::Up.move_checked(pos, map.grid.size()),
            Direction::Down.move_checked(pos, map.grid.size()),
        ) {
            if map.grid[up] && map.grid[down] {
                let with_shortcut = (forward[&up].1 + 1 + backward[&down].1)
                    .min(forward[&down].1 + 1 + backward[&up].1);
                if max - with_shortcut >= 100 {
                    total += 1;
                }
            }
        }
    }
    Ok(total)
}

pub fn part2(map: Map) -> Result<i64> {
    let forward = map.dijkstra_full(map.start);
    let backward = map.dijkstra_full(map.end);
    let max = forward[&map.end].1;
    let mut total = 0;
    for start in map
        .grid
        .indexed_cells()
        .filter_map(|(pos, passable)| passable.then_some(pos))
    {
        for end_row in ((start.0 .0.saturating_sub(20usize))
            ..=(start.0 .0 + 20).min(map.grid.rows().0 - 1))
            .map(Row)
        {
            for end_col in ((start.1 .0.saturating_sub(20usize))
                ..=(start.1 .0 + 20).min(map.grid.cols().0 - 1))
                .map(Col)
            {
                let end = Pos(end_row, end_col);
                if !map.grid[end] {
                    continue;
                }
                let distance =
                    start.0.abs_diff(end.0).0 + start.1.abs_diff(end.1).0;
                if distance <= 20 {
                    let with_shortcut = (forward[&start].1
                        + u64::try_from(distance).unwrap()
                        - 1u64
                        + backward[&end].1)
                        .min(
                            forward[&end].1
                                + u64::try_from(distance).unwrap()
                                - 1u64
                                + backward[&start].1,
                        );
                    if max - with_shortcut >= 100 {
                        total += 1;
                    }
                }
            }
        }
    }
    Ok(total / 2)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 20).unwrap()).unwrap()).unwrap(),
        1263
    );
    assert_eq!(
        part2(parse(parse::data(2024, 20).unwrap()).unwrap()).unwrap(),
        957831
    );
}
