fn fire(
    mut xv: i64,
    mut yv: i64,
    xrange: &std::ops::RangeInclusive<i64>,
    yrange: &std::ops::RangeInclusive<i64>,
) -> Option<i64> {
    let mut xpos = 0;
    let mut ypos = 0;
    let mut max_height = 0;
    loop {
        if xrange.contains(&xpos) && yrange.contains(&ypos) {
            return Some(max_height);
        } else if (xv >= 0 && xpos > *xrange.end())
            || (xv <= 0 && xpos < *xrange.start())
            || (ypos < *yrange.start())
        {
            return None;
        }

        xpos += xv;
        ypos += yv;
        if ypos > max_height {
            max_height = ypos;
        }
        match xv.cmp(&0) {
            std::cmp::Ordering::Greater => {
                xv -= 1;
            }
            std::cmp::Ordering::Less => {
                xv += 1;
            }
            _ => {}
        }
        yv -= 1;
    }
}

pub fn part1() -> anyhow::Result<i64> {
    let rx = regex::Regex::new(
        r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)",
    )
    .unwrap();
    let line = data_lines!()?.next().unwrap();
    let captures = rx.captures(&line).unwrap();
    let xrange: std::ops::RangeInclusive<i64> =
        captures[1].parse().unwrap()..=captures[2].parse().unwrap();
    let yrange: std::ops::RangeInclusive<i64> =
        captures[3].parse().unwrap()..=captures[4].parse().unwrap();

    let mut max_height = 0;
    for xv in *xrange.start().min(&0)..=*xrange.end().max(&0) {
        for yv in *yrange.start().min(&0)
            ..=yrange.start().abs().max(yrange.end().abs())
        {
            if let Some(height) = fire(xv, yv, &xrange, &yrange) {
                if height > max_height {
                    max_height = height;
                }
            }
        }
    }
    Ok(max_height)
}

pub fn part2() -> anyhow::Result<i64> {
    let rx = regex::Regex::new(
        r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)",
    )
    .unwrap();
    let line = data_lines!()?.next().unwrap();
    let captures = rx.captures(&line).unwrap();
    let xrange: std::ops::RangeInclusive<i64> =
        captures[1].parse().unwrap()..=captures[2].parse().unwrap();
    let yrange: std::ops::RangeInclusive<i64> =
        captures[3].parse().unwrap()..=captures[4].parse().unwrap();

    let mut count = 0;
    for xv in *xrange.start().min(&0)..=*xrange.end().max(&0) {
        for yv in *yrange.start().min(&0)
            ..=yrange.start().abs().max(yrange.end().abs())
        {
            if fire(xv, yv, &xrange, &yrange).is_some() {
                count += 1;
            }
        }
    }
    Ok(count)
}

#[test]
fn test() {
    assert_eq!(part1().unwrap(), 5886);
    assert_eq!(part2().unwrap(), 1806);
}
