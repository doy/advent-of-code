use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Vec<Pos>> {
    Ok(parse::raw_lines(fh)
        .map(|s| {
            let mut parts = s.split(",");
            Pos(
                Row(parts.next().unwrap().parse().unwrap()),
                Col(parts.next().unwrap().parse().unwrap()),
            )
        })
        .collect())
}

fn area(p1: Pos, p2: Pos) -> usize {
    (p1.0.abs_diff(p2.0).0 + 1) * (p1.1.abs_diff(p2.1).0 + 1)
}

fn intersecting(p1: Pos, p2: Pos, greens: &[(Pos, Pos)]) -> bool {
    let ptl = Pos(p1.0.min(p2.0), p1.1.min(p2.1));
    let pbr = Pos(p1.0.max(p2.0), p1.1.max(p2.1));
    greens.iter().any(|(gp1, gp2)| {
        if gp1.0 == gp2.0 {
            if ((ptl.0.0 + 1)..=(pbr.0.0 - 1)).contains(&gp1.0.0) {
                let l = gp1.1.min(gp2.1).0;
                let r = gp1.1.max(gp2.1).0;
                !(l >= pbr.1.0 || r <= ptl.1.0)
            } else {
                false
            }
        } else if gp1.1 == gp2.1 {
            if ((ptl.1.0 + 1)..=(pbr.1.0 - 1)).contains(&gp1.1.0) {
                let t = gp1.0.min(gp2.0).0;
                let b = gp1.0.max(gp2.0).0;
                !(t >= pbr.0.0 || b <= ptl.0.0)
            } else {
                false
            }
        } else {
            unreachable!()
        }
    })
}

fn out_of_bounds(p1: Pos, p2: Pos, greens: &[(Pos, Pos)]) -> bool {
    if greens.contains(&(p1, p2)) || greens.contains(&(p2, p1)) {
        return false;
    }
    let midpoint = Pos((p1.0 + p2.0) / 2, (p1.1 + p2.1) / 2);
    greens
        .iter()
        .filter(|(gp1, gp2)| {
            if gp1.0 == gp2.0 {
                gp1.0 > midpoint.0
                    && (gp1.1.min(gp2.1)..=gp1.1.max(gp2.1))
                        .contains(&midpoint.1)
            } else if gp1.1 == gp2.1 {
                false
            } else {
                unreachable!()
            }
        })
        .count()
        .is_multiple_of(2)
}

pub fn part1(tiles: Vec<Pos>) -> Result<i64> {
    Ok(tiles
        .par_iter()
        .copied()
        .enumerate()
        .flat_map(|(i1, p1)| {
            tiles
                .par_iter()
                .copied()
                .enumerate()
                .skip(i1 + 1)
                .map(move |(_, p2)| area(p1, p2))
        })
        .max()
        .unwrap()
        .try_into()
        .unwrap())
}

pub fn part2(tiles: Vec<Pos>) -> Result<i64> {
    let greens: Vec<_> = tiles
        .iter()
        .copied()
        .zip(tiles.iter().copied().cycle().skip(1))
        .collect();
    Ok(tiles
        .par_iter()
        .copied()
        .enumerate()
        .flat_map(|(i1, p1)| {
            tiles
                .par_iter()
                .copied()
                .enumerate()
                .skip(i1 + 1)
                .map(move |(_, p2)| (p1, p2))
        })
        .filter(|(p1, p2)| {
            !intersecting(*p1, *p2, &greens)
                && !out_of_bounds(*p1, *p2, &greens)
        })
        .map(|(p1, p2)| area(p1, p2))
        .max()
        .unwrap()
        .try_into()
        .unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2025, 9).unwrap()).unwrap()).unwrap(),
        4790063600
    );
    assert_eq!(
        part2(parse(parse::data(2025, 9).unwrap()).unwrap()).unwrap(),
        1516172795
    );
}
