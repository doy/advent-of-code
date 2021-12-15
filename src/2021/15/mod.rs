fn adjacent(
    i: usize,
    j: usize,
    max_i: usize,
    max_j: usize,
) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    if i > 0 {
        ret.push((i - 1, j));
    }
    if j > 0 {
        ret.push((i, j - 1));
    }
    if j < max_j {
        ret.push((i, j + 1));
    }
    if i < max_i {
        ret.push((i + 1, j));
    }
    ret
}

fn dijkstra(map: &[Vec<u8>]) -> i64 {
    let mut to_visit: priority_queue::PriorityQueue<_, _> = (0..map.len())
        .flat_map(|i| (0..map[0].len()).map(move |j| (i, j)))
        .map(|pos| {
            (
                pos,
                std::cmp::Reverse(if pos == (0, 0) {
                    0
                } else {
                    i64::max_value()
                }),
            )
        })
        .collect();

    while let Some((pos, std::cmp::Reverse(distance))) = to_visit.pop() {
        if pos == (map.len() - 1, map[0].len() - 1) {
            return distance;
        }

        for neighbor in
            adjacent(pos.0, pos.1, map.len() - 1, map[0].len() - 1)
        {
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

pub fn part1() -> anyhow::Result<i64> {
    let map: Vec<Vec<_>> = data_lines!()?
        .map(|line| line.map(|line| line.bytes().map(|b| b - b'0').collect()))
        .collect::<Result<_, _>>()?;
    Ok(dijkstra(&map))
}

pub fn part2() -> anyhow::Result<i64> {
    let map: Vec<Vec<_>> = data_lines!()?
        .map(|line| line.map(|line| line.bytes().map(|b| b - b'0').collect()))
        .collect::<Result<_, _>>()?;
    let mut large_map = vec![vec![0; map.len() * 5]; map[0].len() * 5];
    for li in 0..5 {
        for lj in 0..5 {
            for (i, row) in map.iter().enumerate() {
                for (j, val) in row.iter().enumerate() {
                    let mut val = val + li + lj;
                    if val > 9 {
                        val -= 9;
                    }
                    large_map[usize::from(li) * map.len() + i]
                        [usize::from(lj) * map[0].len() + j] = val;
                }
            }
        }
    }
    Ok(dijkstra(&large_map))
}
