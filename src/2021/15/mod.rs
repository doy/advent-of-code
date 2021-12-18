use crate::util::grid::*;

fn dijkstra(map: &Grid<u8>) -> i64 {
    let mut to_visit: priority_queue::PriorityQueue<_, _> = (0..map.rows().0)
        .flat_map(|row| {
            (0..map.cols().0).map(move |col| (Row(row), Col(col)))
        })
        .map(|pos| {
            (
                pos,
                std::cmp::Reverse(if pos == (Row(0), Col(0)) {
                    0
                } else {
                    i64::max_value()
                }),
            )
        })
        .collect();

    while let Some(((row, col), std::cmp::Reverse(distance))) = to_visit.pop()
    {
        if row == Row(map.rows().0 - 1) && col == Col(map.cols().0 - 1) {
            return distance;
        }

        for neighbor in map.adjacent(row, col, false) {
            if to_visit.get(&neighbor).is_some() {
                let new_distance =
                    distance + i64::from(map[neighbor.0][neighbor.1]);
                if new_distance < to_visit.get_priority(&neighbor).unwrap().0
                {
                    to_visit.change_priority(
                        &neighbor,
                        std::cmp::Reverse(new_distance),
                    );
                }
            }
        }
    }
    unreachable!()
}

pub fn parse(fh: std::fs::File) -> anyhow::Result<Grid<u8>> {
    Ok(crate::util::parse::digit_grid(crate::util::parse::lines(
        fh,
    )))
}

pub fn part1(grid: Grid<u8>) -> anyhow::Result<i64> {
    Ok(dijkstra(&grid))
}

pub fn part2(grid: Grid<u8>) -> anyhow::Result<i64> {
    let mut large_grid = Grid::default();
    large_grid.grow(Row(grid.rows().0 * 5), Col(grid.cols().0 * 5));
    for lrow in 0..5 {
        for lcol in 0..5 {
            for ((Row(row), Col(col)), val) in grid.indexed_cells() {
                let mut val = val + lrow + lcol;
                while val > 9 {
                    val -= 9;
                }
                large_grid[Row(usize::from(lrow) * grid.rows().0 + row)]
                    [Col(usize::from(lcol) * grid.cols().0 + col)] = val;
            }
        }
    }
    Ok(dijkstra(&large_grid))
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(crate::util::data(2021, 15).unwrap()).unwrap()).unwrap(),
        441
    );
    assert_eq!(
        part2(parse(crate::util::data(2021, 15).unwrap()).unwrap()).unwrap(),
        2849
    );
}
