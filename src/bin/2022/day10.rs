use advent_of_code::prelude::*;

pub struct Cpu {
    x: i64,
    history: Vec<i64>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            x: 1,
            history: vec![1],
        }
    }

    fn step(&mut self, op: Op) {
        match op {
            Op::Noop => {
                self.history.push(self.x);
            }
            Op::Addx(v) => {
                self.history.push(self.x);
                self.x += v;
                self.history.push(self.x);
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Op {
    Noop,
    Addx(i64),
}

pub fn parse(fh: File) -> Result<impl Iterator<Item = Op>> {
    Ok(parse::raw_lines(fh).map(|line| {
        if line == "noop" {
            Op::Noop
        } else if let Some(v) = line.strip_prefix("addx ") {
            Op::Addx(v.parse().unwrap())
        } else {
            panic!("failed to parse {}", line)
        }
    }))
}

pub fn part1(ops: impl Iterator<Item = Op>) -> Result<i64> {
    let mut cpu = Cpu::new();
    for op in ops {
        cpu.step(op);
    }

    let mut total = 0;
    for cycle in [20, 60, 100, 140, 180, 220] {
        let strength = i64::try_from(cycle).unwrap() * cpu.history[cycle - 1];
        total += strength;
    }
    Ok(total)
}

pub fn part2(ops: impl Iterator<Item = Op>) -> Result<String> {
    let mut cpu = Cpu::new();
    for op in ops {
        cpu.step(op);
    }
    let mut s = String::new();
    for row in cpu.history.chunks(40) {
        if row.len() < 40 {
            break;
        }
        for (x, pos) in row.iter().enumerate() {
            if i64::try_from(x).unwrap().abs_diff(*pos) <= 1 {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    Ok(ocr(&s))
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 10).unwrap()).unwrap()).unwrap(),
        15680
    );
    assert_eq!(
        part2(parse(parse::data(2022, 10).unwrap()).unwrap()).unwrap(),
        "ZFBFHGUP"
    );
}
