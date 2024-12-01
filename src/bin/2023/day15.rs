use advent_of_code::prelude::*;

#[derive(Clone, Debug)]
struct LensBox {
    lenses: Vec<(String, i64)>,
}

impl LensBox {
    fn new() -> Self {
        Self { lenses: vec![] }
    }

    fn insert(&mut self, label: String, focal_length: i64) {
        if let Some(idx) = self.lenses.iter().position(|(s, _)| *s == label) {
            self.lenses[idx] = (label, focal_length);
        } else {
            self.lenses.push((label, focal_length));
        }
    }

    fn remove(&mut self, label: &str) {
        if let Some(idx) = self.lenses.iter().position(|(s, _)| *s == label) {
            self.lenses.remove(idx);
        }
    }
}

fn hash(s: &str) -> usize {
    let mut current = 0usize;
    for c in s.bytes() {
        current += usize::from(c);
        current *= 17;
        current %= 256;
    }
    current
}

pub fn parse(fh: File) -> Result<Vec<String>> {
    Ok(parse::split(fh, b',').collect())
}

pub fn part1(ops: Vec<String>) -> Result<i64> {
    Ok(ops
        .into_iter()
        .map(|s| hash(&s))
        .sum::<usize>()
        .try_into()
        .unwrap())
}

pub fn part2(ops: Vec<String>) -> Result<i64> {
    let mut boxes = vec![LensBox::new(); 256];
    for op in ops {
        if let Some(idx) = op.find('=') {
            let label = &op[..idx];
            let lens_box = hash(label);
            let focal_length: i64 = op[(idx + 1)..].parse().unwrap();
            boxes[lens_box].insert(label.to_string(), focal_length);
        } else if op.ends_with('-') {
            let label = &op[..(op.len() - 1)];
            let lens_box = hash(label);
            boxes[lens_box].remove(label);
        }
    }
    Ok(boxes
        .into_iter()
        .enumerate()
        .map(|(box_num, lens_box)| {
            lens_box
                .lenses
                .iter()
                .enumerate()
                .map(|(lens_num, (_, focal_length))| {
                    (i64::try_from(box_num).unwrap() + 1)
                        * (i64::try_from(lens_num).unwrap() + 1)
                        * focal_length
                })
                .sum::<i64>()
        })
        .sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 15).unwrap()).unwrap()).unwrap(),
        497373
    );
    assert_eq!(
        part2(parse(parse::data(2023, 15).unwrap()).unwrap()).unwrap(),
        259356
    );
}
