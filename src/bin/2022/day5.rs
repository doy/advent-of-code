#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

#[derive(Default)]
pub struct Pile(Vec<char>);

impl Pile {
    fn move_from(&mut self, other: &mut Self, n: usize) {
        for _ in 0..n {
            self.0.push(other.0.pop().unwrap());
        }
    }

    fn unshift(&mut self, c: char) {
        self.0.insert(0, c);
    }

    fn push(&mut self, c: char) {
        self.0.push(c);
    }

    fn pop(&mut self) -> Option<char> {
        self.0.pop()
    }

    fn top(&self) -> Option<char> {
        self.0.last().copied()
    }
}

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

pub struct Crates {
    piles: Vec<Pile>,
    instructions: Vec<Instruction>,
}

pub fn parse(fh: File) -> Result<Crates> {
    let mut lines = parse::raw_lines(fh);
    let mut piles: Vec<Pile> = vec![];

    for line in lines.by_ref() {
        if line.starts_with(" 1 ") {
            break;
        }

        let pile_count = (line.len() + 1) / 4;
        piles.resize_with(pile_count, Default::default);

        for (pile_idx, pile) in piles.iter_mut().enumerate() {
            let idx = pile_idx * 4;
            assert!([' ', '['].contains(&line.chars().nth(idx).unwrap()));
            let crate_name = line.chars().nth(idx + 1).unwrap();
            if crate_name != ' ' {
                pile.unshift(crate_name);
            }
            assert!([' ', ']'].contains(&line.chars().nth(idx + 2).unwrap()));
            assert!([Some(' '), None].contains(&line.chars().nth(idx + 3)));
        }
    }

    assert_eq!(lines.next().unwrap(), "");

    let mut instructions = vec![];
    for line in lines {
        let captures = regex_captures!(
            r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$",
            &line
        )
        .unwrap();
        let count = captures[1].parse().unwrap();
        let from = captures[2].parse().unwrap();
        let to = captures[3].parse().unwrap();
        instructions.push(Instruction { count, from, to });
    }

    Ok(Crates {
        piles,
        instructions,
    })
}

pub fn part1(mut crates: Crates) -> Result<String> {
    for Instruction { count, from, to } in crates.instructions {
        for _ in 0..count {
            let c = crates.piles[from - 1].pop().unwrap();
            crates.piles[to - 1].push(c);
        }
    }
    Ok(crates
        .piles
        .iter()
        .map(|pile| pile.top().unwrap())
        .collect())
}

pub fn part2(mut crates: Crates) -> Result<String> {
    for Instruction { count, from, to } in crates.instructions {
        let mut tmp = vec![];
        for _ in 0..count {
            let c = crates.piles[from - 1].pop().unwrap();
            tmp.push(c);
        }
        for c in tmp.iter().copied().rev() {
            crates.piles[to - 1].push(c);
        }
    }
    Ok(crates
        .piles
        .iter()
        .map(|pile| pile.top().unwrap())
        .collect())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 5).unwrap()).unwrap()).unwrap(),
        "PSNRGBTFT"
    );
    assert_eq!(
        part2(parse(parse::data(2022, 5).unwrap()).unwrap()).unwrap(),
        "BNTZFPMMW"
    );
}
