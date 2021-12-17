use crate::util::grid::*;

fn iterate(grid: &mut Grid<(u8, bool)>) -> i64 {
    let mut flashes = 0;
    for (cell, _) in grid.cells_mut() {
        *cell += 1;
    }

    loop {
        let mut new_flashes = 0;
        let mut updates: Grid<u8> = Grid::default();
        updates.grow(grid.rows(), grid.cols());
        for ((row, col), (cell, flashed)) in grid.indexed_cells_mut() {
            if *flashed {
                continue;
            }
            if *cell > 9 {
                *flashed = true;
                new_flashes += 1;
                for (row, col) in updates.adjacent(row, col, true) {
                    updates[row][col] += 1;
                }
            }
        }
        if new_flashes > 0 {
            flashes += new_flashes;
            for ((row, col), val) in updates.indexed_cells() {
                grid[row][col].0 += val;
            }
        } else {
            break;
        }
    }

    for (cell, flashed) in grid.cells_mut() {
        if *flashed {
            *cell = 0;
            *flashed = false;
        }
    }

    flashes
}

pub fn part1() -> anyhow::Result<i64> {
    let mut map = data_digit_grid!()
        .indexed_cells()
        .map(|((row, col), cell)| ((row, col), (*cell, false)))
        .collect();
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += iterate(&mut map);
    }
    Ok(flashes)
}

pub fn part2() -> anyhow::Result<i64> {
    let mut map = data_digit_grid!()
        .indexed_cells()
        .map(|((row, col), cell)| ((row, col), (*cell, false)))
        .collect();

    let mut step = 1;
    loop {
        let flashes = iterate(&mut map);
        if flashes == (map.rows().0 * map.cols().0).try_into()? {
            break;
        }
        step += 1;
    }
    Ok(step)
}

#[test]
fn test() {
    assert_eq!(part1().unwrap(), 1673);
    assert_eq!(part2().unwrap(), 279);
}
