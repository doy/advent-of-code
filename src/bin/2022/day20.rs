#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

#[derive(Default)]
pub struct WrappingQueue {
    queue: Vec<(usize, isize)>,
}

impl std::fmt::Debug for WrappingQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.queue.iter().copied().map(|v| v.1))
            .finish()
    }
}

impl WrappingQueue {
    fn mix(&mut self, idx: usize) {
        let val = self.queue.remove(idx);
        let new_idx: usize = ((idx as isize + val.1)
            .rem_euclid(self.queue.len() as isize))
        .try_into()
        .unwrap();
        self.queue.insert(new_idx, val);
    }
}

pub fn parse(fh: File) -> Result<WrappingQueue> {
    Ok(WrappingQueue {
        queue: parse::lines(fh).enumerate().collect(),
    })
}

pub fn part1(mut q: WrappingQueue) -> Result<isize> {
    for i in 0..q.queue.len() {
        let idx = q.queue.iter().position(|(idx, _)| *idx == i).unwrap();
        q.mix(idx);
    }
    let zero_idx = q.queue.iter().position(|(_, val)| *val == 0).unwrap();
    Ok(q.queue[(zero_idx + 1000) % q.queue.len()].1
        + q.queue[(zero_idx + 2000) % q.queue.len()].1
        + q.queue[(zero_idx + 3000) % q.queue.len()].1)
}

pub fn part2(q: WrappingQueue) -> Result<isize> {
    let key = 811589153;
    let mut q = WrappingQueue {
        queue: q.queue.into_iter().map(|(i, v)| (i, key * v)).collect(),
    };
    for _ in 0..10 {
        for i in 0..q.queue.len() {
            let idx = q.queue.iter().position(|(idx, _)| *idx == i).unwrap();
            q.mix(idx);
        }
    }
    let zero_idx = q.queue.iter().position(|(_, val)| *val == 0).unwrap();
    Ok(q.queue[(zero_idx + 1000) % q.queue.len()].1
        + q.queue[(zero_idx + 2000) % q.queue.len()].1
        + q.queue[(zero_idx + 3000) % q.queue.len()].1)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 20).unwrap()).unwrap()).unwrap(),
        17490
    );
    assert_eq!(
        part2(parse(parse::data(2022, 20).unwrap()).unwrap()).unwrap(),
        1632917375836
    );
}
