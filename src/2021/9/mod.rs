pub fn part1() -> anyhow::Result<i64> {
    let mut map = vec![];
    for line in data_lines!()? {
        let line = line?;
        let mut row = vec![];
        for c in line.bytes() {
            row.push(c - b'0');
        }
        map.push(row);
    }

    let mut risk = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let pos = map[i][j];
            let neighbors = [
                map.get(i + 1).and_then(|v| v.get(j)),
                map.get(i).and_then(|v| v.get(j + 1)),
                if i == 0 {
                    None
                } else {
                    map.get(i - 1).and_then(|v| v.get(j))
                },
                if j == 0 {
                    None
                } else {
                    map.get(i).and_then(|v| v.get(j - 1))
                },
            ];
            let neighbors: Vec<_> = neighbors.into_iter().flatten().collect();
            if neighbors.iter().all(|n| pos < **n) {
                risk += 1 + pos as i64;
            }
        }
    }
    Ok(risk)
}

pub fn part2() -> anyhow::Result<i64> {
    let mut map = vec![];
    for line in data_lines!()? {
        let line = line?;
        let mut row = vec![];
        for c in line.bytes() {
            row.push(c - b'0');
        }
        map.push(row);
    }

    let mut low = vec![];
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let pos = map[i][j];
            let neighbors = [
                map.get(i + 1).and_then(|v| v.get(j)),
                map.get(i).and_then(|v| v.get(j + 1)),
                if i == 0 {
                    None
                } else {
                    map.get(i - 1).and_then(|v| v.get(j))
                },
                if j == 0 {
                    None
                } else {
                    map.get(i).and_then(|v| v.get(j - 1))
                },
            ];
            let neighbors: Vec<_> = neighbors.into_iter().flatten().collect();
            if neighbors.iter().all(|n| pos < **n) {
                low.push((i, j));
            }
        }
    }

    let mut sizes = vec![];
    for (i, j) in low {
        let mut basin = vec![vec![false; map[0].len()]; map.len()];
        let mut check = vec![(i, j)];
        let mut count = 0;
        while let Some((i, j)) = check.pop() {
            if basin[i][j] || map[i][j] == 9 {
                continue;
            }

            basin[i][j] = true;
            count += 1;

            if i < map.len() - 1 && !basin[i + 1][j] {
                check.push((i + 1, j));
            }
            if i > 0 && !basin[i - 1][j] {
                check.push((i - 1, j));
            }
            if j < map[0].len() - 1 && !basin[i][j + 1] {
                check.push((i, j + 1));
            }
            if j > 0 && !basin[i][j - 1] {
                check.push((i, j - 1));
            }
        }
        sizes.push(count);
    }
    sizes.sort_unstable();
    Ok(sizes[sizes.len() - 1]
        * sizes[sizes.len() - 2]
        * sizes[sizes.len() - 3])
}
