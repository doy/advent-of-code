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

        for neighbor in crate::util::adjacent(
            pos.0,
            pos.1,
            map.len() - 1,
            map[0].len() - 1,
            false,
        ) {
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
    Ok(dijkstra(&data_digit_grid!()))
}

pub fn part2() -> anyhow::Result<i64> {
    let map = data_digit_grid!();
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

#[test]
fn test() {
    assert_eq!(part1().unwrap(), 441);
    assert_eq!(part2().unwrap(), 2849);
}
