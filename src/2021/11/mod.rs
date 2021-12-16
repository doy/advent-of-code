fn iterate(map: &mut Vec<Vec<(u8, bool)>>) -> i64 {
    let mut flashes = 0;
    for line in map.iter_mut() {
        for (cell, _) in line.iter_mut() {
            *cell += 1;
        }
    }

    loop {
        let mut new_flashes = 0;
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j].1 {
                    continue;
                }
                if map[i][j].0 > 9 {
                    map[i][j].1 = true;
                    new_flashes += 1;
                    for (i, j) in crate::util::adjacent(
                        i,
                        j,
                        map.len() - 1,
                        map[0].len() - 1,
                        true,
                    ) {
                        map[i][j].0 += 1;
                    }
                }
            }
        }
        if new_flashes > 0 {
            flashes += new_flashes;
        } else {
            break;
        }
    }

    for line in map.iter_mut() {
        for (cell, flashed) in line.iter_mut() {
            if *flashed {
                *cell = 0;
                *flashed = false;
            }
        }
    }
    flashes
}

pub fn part1() -> anyhow::Result<i64> {
    let mut map: Vec<Vec<_>> = data_digit_grid!()
        .iter()
        .map(|line| line.iter().map(|i| (*i, false)).collect())
        .collect();
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += iterate(&mut map);
    }
    Ok(flashes)
}

pub fn part2() -> anyhow::Result<i64> {
    let mut map: Vec<Vec<_>> = data_digit_grid!()
        .iter()
        .map(|line| line.iter().map(|i| (*i, false)).collect())
        .collect();

    let mut step = 1;
    loop {
        let flashes = iterate(&mut map);
        if flashes == (map.len() * map[0].len()).try_into()? {
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
