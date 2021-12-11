fn adjacent(
    i: usize,
    j: usize,
    max_i: usize,
    max_j: usize,
) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    if i > 0 {
        if j > 0 {
            ret.push((i - 1, j - 1));
        }
        ret.push((i - 1, j));
        if j < max_j {
            ret.push((i - 1, j + 1));
        }
    }
    if j > 0 {
        ret.push((i, j - 1));
    }
    if j < max_j {
        ret.push((i, j + 1));
    }
    if i < max_i {
        if j > 0 {
            ret.push((i + 1, j - 1));
        }
        ret.push((i + 1, j));
        if j < max_j {
            ret.push((i + 1, j + 1));
        }
    }
    ret
}

fn iterate(map: &mut Vec<Vec<(i64, bool)>>) -> i64 {
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
                    for (i, j) in
                        adjacent(i, j, map.len() - 1, map[0].len() - 1)
                    {
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
    let mut map = data_lines!()?
        .collect::<Result<Vec<String>, _>>()?
        .iter()
        .map(|line| {
            line.bytes()
                .map(|b| ((b - b'0') as i64, false))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut flashes = 0;
    for _ in 0..100 {
        flashes += iterate(&mut map);
    }
    Ok(flashes)
}

pub fn part2() -> anyhow::Result<i64> {
    let mut map = data_lines!()?
        .collect::<Result<Vec<String>, _>>()?
        .iter()
        .map(|line| {
            line.bytes()
                .map(|b| ((b - b'0') as i64, false))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

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
