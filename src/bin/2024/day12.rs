use advent_of_code::prelude::*;

fn find_new_region(plot: &Grid<(u8, bool)>) -> Option<Pos> {
    plot.indexed_cells()
        .find(|(_, (_, seen))| !seen)
        .map(|(pos, _)| pos)
}

fn find_new_region_hole(plot: &Grid<bool>) -> Option<Pos> {
    plot.indexed_cells()
        .find(|(_, seen)| !*seen)
        .map(|(pos, _)| pos)
}

pub fn parse(fh: File) -> Result<Grid<u8>> {
    Ok(parse::grid(parse::raw_lines(fh), |c, _| c))
}

pub fn part1(plot: Grid<u8>) -> Result<i64> {
    let mut seen = plot
        .indexed_cells()
        .map(|(pos, c)| (pos, (*c, false)))
        .collect();
    let mut total = 0;
    while let Some(pos) = find_new_region(&seen) {
        let plant = plot[pos];
        let region = seen.flood_fill(
            pos,
            &(plant, true),
            |(c, seen)| !seen && *c == plant,
            false,
        );
        let area = region.len();
        let mut interior = 0;
        for pos in &region {
            if region.contains(&Pos(pos.0, pos.1 + 1))
                && region.contains(&Pos(pos.0 + 1, pos.1))
                && region.contains(&Pos(pos.0 + 1, pos.1 + 1))
            {
                interior += 1;
            }
        }
        let mut overlaps = 0;
        for pos in &region {
            if !region.contains(&Pos(pos.0, pos.1 + 1))
                && !region.contains(&Pos(pos.0 + 1, pos.1))
                && region.contains(&Pos(pos.0 + 1, pos.1 + 1))
            {
                overlaps += 1;
            }
            if pos.0 > Row(0)
                && !region.contains(&Pos(pos.0, pos.1 + 1))
                && !region.contains(&Pos(pos.0 - 1, pos.1))
                && region.contains(&Pos(pos.0 - 1, pos.1 + 1))
            {
                overlaps += 1;
            }
        }
        let mut holes = 0;
        let mut seen_hole: Grid<_> = plot
            .indexed_cells()
            .map(|(pos, _)| (pos, region.contains(&pos)))
            .collect();
        while let Some(pos) = find_new_region_hole(&seen_hole) {
            let hole_region =
                seen_hole.flood_fill(pos, &true, |seen| !seen, false);
            if !hole_region.iter().any(|pos| {
                pos.0 == Row(0)
                    || pos.0 == seen_hole.rows() - 1
                    || pos.1 == Col(0)
                    || pos.1 == seen_hole.cols() - 1
            }) {
                holes += 1;
            }
        }
        let perimeter = 2 * (area + 1 - interior - holes + overlaps);
        let price = area * perimeter;
        total += price;
    }
    Ok(total.try_into().unwrap())
}

pub fn part2(plot: Grid<u8>) -> Result<i64> {
    let mut seen = plot
        .indexed_cells()
        .map(|(pos, c)| (pos, (*c, false)))
        .collect();
    let mut total = 0;
    while let Some(pos) = find_new_region(&seen) {
        let plant = plot[pos];
        let region = seen.flood_fill(
            pos,
            &(plant, true),
            |(c, seen)| !seen && *c == plant,
            false,
        );
        let area = region.len();

        let min_row = region.iter().min_by_key(|pos| pos.0 .0).unwrap().0;
        let max_row = region.iter().max_by_key(|pos| pos.0 .0).unwrap().0;
        let min_col = region.iter().min_by_key(|pos| pos.1 .0).unwrap().1;
        let max_col = region.iter().max_by_key(|pos| pos.1 .0).unwrap().1;

        let mut horiz = 0;
        let mut fence = false;
        for col in (min_col.0..=max_col.0).map(Col) {
            let pos = Pos(min_row, col);
            if region.contains(&pos) {
                if !fence {
                    fence = true;
                    horiz += 1;
                }
            } else {
                fence = false;
            }
        }
        for row in (min_row.0 + 1..=max_row.0).map(Row) {
            let mut cur_fence = None;
            for col in (min_col.0..=max_col.0).map(Col) {
                let pos = Pos(row, col);
                let pos_above = Pos(row - 1, col);
                if region.contains(&pos) ^ region.contains(&pos_above) {
                    if cur_fence
                        != Some((
                            region.contains(&pos),
                            region.contains(&pos_above),
                        ))
                    {
                        cur_fence = Some((
                            region.contains(&pos),
                            region.contains(&pos_above),
                        ));
                        horiz += 1;
                    }
                } else {
                    cur_fence = None;
                }
            }
        }
        fence = false;
        for col in (min_col.0..=max_col.0).map(Col) {
            let pos = Pos(max_row, col);
            if region.contains(&pos) {
                if !fence {
                    fence = true;
                    horiz += 1;
                }
            } else {
                fence = false;
            }
        }

        let mut vert = 0;
        let mut fence = false;
        for row in (min_row.0..=max_row.0).map(Row) {
            let pos = Pos(row, min_col);
            if region.contains(&pos) {
                if !fence {
                    fence = true;
                    vert += 1;
                }
            } else {
                fence = false;
            }
        }
        for col in (min_col.0 + 1..=max_col.0).map(Col) {
            let mut cur_fence = None;
            for row in (min_row.0..=max_row.0).map(Row) {
                let pos = Pos(row, col);
                let pos_left = Pos(row, col - 1);
                if region.contains(&pos) ^ region.contains(&pos_left) {
                    if cur_fence
                        != Some((
                            region.contains(&pos),
                            region.contains(&pos_left),
                        ))
                    {
                        cur_fence = Some((
                            region.contains(&pos),
                            region.contains(&pos_left),
                        ));
                        vert += 1;
                    }
                } else {
                    cur_fence = None;
                }
            }
        }
        fence = false;
        for row in (min_row.0..=max_row.0).map(Row) {
            let pos = Pos(row, max_col);
            if region.contains(&pos) {
                if !fence {
                    fence = true;
                    vert += 1;
                }
            } else {
                fence = false;
            }
        }

        let sides = horiz + vert;
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
