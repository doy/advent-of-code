use advent_of_code::prelude::*;

pub struct Tree {
    presents: Vec<usize>,
    regions: Vec<(Size, Vec<usize>)>,
}

pub fn parse(fh: File) -> Result<Tree> {
    let mut lines = parse::raw_lines(fh);
    let mut presents = vec![];
    loop {
        let mut chunk = parse::chunk(&mut lines).peekable();
        if let Some(line) = chunk.peek() {
            if line.ends_with(':') {
                chunk.next();
                presents.push(
                    chunk
                        .map(|line| {
                            line.chars().filter(|c| *c == '#').count()
                        })
                        .sum(),
                );
            } else {
                let regions = chunk
                    .map(|line| {
                        let mut parts = line.split(": ");
                        let size = parts.next().unwrap();
                        let present_count = parts.next().unwrap();

                        let mut parts = size.split('x');
                        let size = Size(
                            Row(parts.next().unwrap().parse().unwrap()),
                            Col(parts.next().unwrap().parse().unwrap()),
                        );

                        let parts = present_count.split(' ');
                        let present_count =
                            parts.map(|n| n.parse().unwrap()).collect();
                        (size, present_count)
                    })
                    .collect();
                return Ok(Tree { presents, regions });
            }
        } else {
            unreachable!()
        }
    }
}

pub fn part1(tree: Tree) -> Result<i64> {
    Ok(tree
        .regions
        .iter()
        .filter(|(size, present_count)| {
            size.0.0 * size.1.0
                >= present_count
                    .iter()
                    .zip(&tree.presents)
                    .map(|(a, b)| a * b)
                    .sum()
        })
        .count()
        .try_into()
        .unwrap())
}

pub fn part2(_: Tree) -> Result<i64> {
    Ok(0)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2025, 12).unwrap()).unwrap()).unwrap(),
        569
    );
    assert_eq!(
        part2(parse(parse::data(2025, 12).unwrap()).unwrap()).unwrap(),
        0
    );
}
