use advent_of_code::prelude::*;

fn bounding_box(region: &HashSet<Pos>) -> (Pos, Pos) {
    let mut min = Pos(Row(usize::MAX), Col(usize::MAX));
    let mut max = Pos(Row(0), Col(0));
    for pos in region {
        if pos.0 < min.0 {
            min.0 = pos.0;
        }
        if pos.0 > max.0 {
            max.0 = pos.0
        }
        if pos.1 < min.1 {
            min.1 = pos.1;
        }
        if pos.1 > max.1 {
            max.1 = pos.1
        }
    }
    (min, max)
}

pub fn parse(fh: File) -> Result<Grid<u8>> {
    Ok(parse::grid(parse::raw_lines(fh), |c, _| c))
}

pub fn part1(plot: Grid<u8>) -> Result<i64> {
    let mut seen: Grid<bool> = Grid::default();
    seen.grow(plot.size());
    let mut total = 0;
    while let Some(pos) = seen.find_next(|_, seen| !*seen) {
        let plant = plot[pos];
        let region = seen.flood_fill(
            pos,
            &true,
            |pos, seen| !seen && plot[pos] == plant,
            false,
        );
        let area = region.len();
        let (min, max) = bounding_box(&region);
        let rows = min.0.to_inclusive(max.0);
        let cols = min.1.to_inclusive(max.1);
        let mut perimeter = 0;
        // outer edges
        for row in [min.0, max.0] {
            perimeter += cols
                .clone()
                .filter(|col| region.contains(&Pos(row, *col)))
                .count();
        }
        for col in [min.1, max.1] {
            perimeter += rows
                .clone()
                .filter(|row| region.contains(&Pos(*row, col)))
                .count();
        }
        // interior
        perimeter += rows
            .clone()
            .skip(1)
            .flat_map(|row| cols.clone().map(move |col| Pos(row, col)))
            .filter(|pos| {
                region.contains(pos) ^ region.contains(&Pos(pos.0 - 1, pos.1))
            })
            .count();
        perimeter += cols
            .clone()
            .skip(1)
            .flat_map(|col| rows.clone().map(move |row| Pos(row, col)))
            .filter(|pos| {
                region.contains(pos) ^ region.contains(&Pos(pos.0, pos.1 - 1))
            })
            .count();
        total += area * perimeter;
    }
    Ok(total.try_into().unwrap())
}

pub fn part2(plot: Grid<u8>) -> Result<i64> {
    let mut seen: Grid<bool> = Grid::default();
    seen.grow(plot.size());
    let mut total = 0;
    while let Some(pos) = seen.find_next(|_, seen| !*seen) {
        let plant = plot[pos];
        let region = seen.flood_fill(
            pos,
            &true,
            |pos, seen| !seen && plot[pos] == plant,
            false,
        );
        let area = region.len();
        let (min, max) = bounding_box(&region);
        let rows = min.0.to_inclusive(max.0);
        let cols = min.1.to_inclusive(max.1);
        let mut sides = 0;
        // outer edges
        for row in [min.0, max.0] {
            let mut seen_side = false;
            for col in cols.clone() {
                let pos = Pos(row, col);
                if region.contains(&pos) {
                    if !seen_side {
                        sides += 1;
                        seen_side = true;
                    }
                } else {
                    seen_side = false;
                }
            }
        }
        for col in [min.1, max.1] {
            let mut seen_side = false;
            for row in rows.clone() {
                let pos = Pos(row, col);
                if region.contains(&pos) {
                    if !seen_side {
                        sides += 1;
                        seen_side = true;
                    }
                } else {
                    seen_side = false;
                }
            }
        }
        // interior
        for row in rows.clone().skip(1) {
            let mut seen_side = None;
            for col in cols.clone() {
                let pos = Pos(row, col);
                let prev_pos = Pos(row - 1, col);
                if region.contains(&pos) ^ region.contains(&prev_pos) {
                    if seen_side
                        != Some((
                            region.contains(&pos),
                            region.contains(&prev_pos),
                        ))
                    {
                        seen_side = Some((
                            region.contains(&pos),
                            region.contains(&prev_pos),
                        ));
                        sides += 1;
                    }
                } else {
                    seen_side = None;
                }
            }
        }
        for col in cols.clone().skip(1) {
            let mut seen_side = None;
            for row in rows.clone() {
                let pos = Pos(row, col);
                let prev_pos = Pos(row, col - 1);
                if region.contains(&pos) ^ region.contains(&prev_pos) {
                    if seen_side
                        != Some((
                            region.contains(&pos),
                            region.contains(&prev_pos),
                        ))
                    {
                        seen_side = Some((
                            region.contains(&pos),
                            region.contains(&prev_pos),
                        ));
                        sides += 1;
                    }
                } else {
                    seen_side = None;
                }
            }
        }
        total += area * sides;
    }
    Ok(total.try_into().unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 12).unwrap()).unwrap()).unwrap(),
        1352976
    );
    assert_eq!(
        part2(parse(parse::data(2024, 12).unwrap()).unwrap()).unwrap(),
        808796
    );
}
