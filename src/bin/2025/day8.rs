use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Vec<(i64, i64, i64)>> {
    Ok(parse::raw_lines(fh)
        .map(|s| {
            let mut parts = s.split(',');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect())
}

fn distance(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> i64 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    let dz = p2.2 - p1.2;
    dx * dx + dy * dy + dz * dz
}

struct Forest {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Forest {
    fn new_disjoint(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            size: vec![1; size],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return;
        }
        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent[y] = x;
        self.size[x] += self.size[y];
    }
}

pub fn part1(boxes: Vec<(i64, i64, i64)>) -> Result<i64> {
    let mut forest = Forest::new_disjoint(boxes.len());
    let mut distances: Vec<_> = boxes
        .par_iter()
        .copied()
        .enumerate()
        .flat_map(|(i1, p1)| {
            boxes
                .par_iter()
                .copied()
                .enumerate()
                .skip(i1 + 1)
                .map(move |(i2, p2)| ((i1, i2), distance(p1, p2)))
        })
        .collect();
    distances.par_sort_unstable_by_key(|(_, dist)| *dist);
    for (box1, box2) in distances.into_iter().map(|(i, _)| i).take(1000) {
        forest.union(box1, box2);
    }
    let mut sizes = (0..boxes.len()).map(|i| forest.find(i)).fold(
        std::iter::repeat_n(0, boxes.len())
            .enumerate()
            .collect::<Vec<_>>(),
        |mut acc, i| {
            acc[i].1 += 1;
            acc
        },
    );
    sizes.par_sort_unstable_by_key(|(_, size)| std::cmp::Reverse(*size));

    Ok(sizes[0].1 * sizes[1].1 * sizes[2].1)
}

pub fn part2(boxes: Vec<(i64, i64, i64)>) -> Result<i64> {
    let mut forest = Forest::new_disjoint(boxes.len());
    let mut distances: Vec<_> = boxes
        .par_iter()
        .copied()
        .enumerate()
        .flat_map(|(i1, p1)| {
            boxes
                .par_iter()
                .copied()
                .enumerate()
                .skip(i1 + 1)
                .map(move |(i2, p2)| ((i1, i2), distance(p1, p2)))
        })
        .collect();
    distances.par_sort_unstable_by_key(|(_, dist)| *dist);
    for (box1, box2) in distances.into_iter().map(|(i, _)| i) {
        forest.union(box1, box2);
        if (0..boxes.len()).all(|i| forest.find(i) == forest.find(0)) {
            return Ok(boxes[box1].0 * boxes[box2].0);
        }
    }
    unreachable!();
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2025, 8).unwrap()).unwrap()).unwrap(),
        75680
    );
    assert_eq!(
        part2(parse(parse::data(2025, 8).unwrap()).unwrap()).unwrap(),
        8995844880
    );
}
