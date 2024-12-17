use advent_of_code::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl TryFrom<u8> for Op {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Op::Adv,
            1 => Op::Bxl,
            2 => Op::Bst,
            3 => Op::Jnz,
            4 => Op::Bxc,
            5 => Op::Out,
            6 => Op::Bdv,
            7 => Op::Cdv,
            _ => unreachable!(),
        })
    }
}

impl From<Op> for u8 {
    fn from(value: Op) -> Self {
        match value {
            Op::Adv => 0,
            Op::Bxl => 1,
            Op::Bst => 2,
            Op::Jnz => 3,
            Op::Bxc => 4,
            Op::Out => 5,
            Op::Bdv => 6,
            Op::Cdv => 7,
        }
    }
}

impl From<Op> for i64 {
    fn from(value: Op) -> Self {
        match value {
            Op::Adv => 0,
            Op::Bxl => 1,
            Op::Bst => 2,
            Op::Jnz => 3,
            Op::Bxc => 4,
            Op::Out => 5,
            Op::Bdv => 6,
            Op::Cdv => 7,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Computer {
    a: i64,
    b: i64,
    c: i64,
    ip: usize,
    data: Vec<Op>,
}

impl Computer {
    fn run(&mut self, out: &mut Vec<u8>) {
        loop {
            match self.data.get(self.ip) {
                Some(Op::Adv) => {
                    self.a /= 2i64.pow(u32::try_from(self.combo()).unwrap())
                }
                Some(Op::Bxl) => {
                    self.b ^= self.literal();
                }
                Some(Op::Bst) => {
                    self.b = self.combo() % 8;
                }
                Some(Op::Jnz) => {
                    if self.a != 0 {
                        self.ip = usize::try_from(self.literal()).unwrap();
                        continue;
                    }
                }
                Some(Op::Bxc) => {
                    self.b ^= self.c;
                }
                Some(Op::Out) => {
                    out.push(u8::try_from(self.combo() % 8).unwrap());
                }
                Some(Op::Bdv) => {
                    self.b = self.a
                        / 2i64.pow(u32::try_from(self.combo()).unwrap())
                }
                Some(Op::Cdv) => {
                    self.c = self.a
                        / 2i64.pow(u32::try_from(self.combo()).unwrap())
                }
                None => break,
            }
            self.ip += 2;
        }
    }

    fn literal(&self) -> i64 {
        i64::from(self.data[self.ip + 1])
    }

    fn combo(&self) -> i64 {
        match i64::from(self.data[self.ip + 1]) {
            i @ 0..=3 => i,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

pub fn parse(fh: File) -> Result<Computer> {
    let mut lines = parse::raw_lines(fh);
    let a = lines
        .next()
        .unwrap()
        .trim()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let b = lines
        .next()
        .unwrap()
        .trim()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let c = lines
        .next()
        .unwrap()
        .trim()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let ip = 0;
    lines.next().unwrap();
    let data = lines
        .next()
        .unwrap()
        .trim()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| Op::try_from(s.parse::<u8>().unwrap()).unwrap())
        .collect();
    Ok(Computer { a, b, c, ip, data })
}

pub fn part1(mut computer: Computer) -> Result<i64> {
    let mut out = vec![];
    computer.run(&mut out);
    Ok(out.into_iter().fold(0, |acc, i| acc * 10 + i64::from(i)))
}

pub fn part2(computer: Computer) -> Result<i64> {
    let expected: Vec<_> =
        computer.data.iter().map(|op| u8::from(*op)).collect();
    if computer.data.len() == 6 {
        // example is brute-forceable, but doesn't follow the structure
        Ok((0..isize::MAX)
            .into_par_iter()
            .by_exponential_blocks()
            .find_first(|a| {
                let a = i64::try_from(*a).unwrap();
                let mut computer = computer.clone();
                computer.a = a;
                let mut out = vec![];
                computer.run(&mut out);
                out == expected
            })
            .unwrap()
            .try_into()
            .unwrap())
    } else {
        // real problem has a consistent structure in its input, but is too
        // big to brute force
        let bits: Vec<Option<bool>> = vec![None; expected.len() * 3 + 10];
        fn rec(
            expected: &[u8],
            bits: &[Option<bool>],
            i: usize,
        ) -> Vec<Vec<Option<bool>>> {
            if i >= expected.len() {
                return vec![bits.to_vec()];
            }

            let mut ret = vec![];
            let base = i * 3;
            for j in 0..8 {
                let mut bits = bits.to_vec();

                let lower = j;
                if bits[base] == Some(lower & 1 == 0) {
                    continue;
                }
                bits[base] = Some(lower & 1 == 1);
                if bits[base + 1] == Some((lower >> 1) & 1 == 0) {
                    continue;
                }
                bits[base + 1] = Some((lower >> 1) & 1 == 1);
                if bits[base + 2] == Some((lower >> 2) & 1 == 0) {
                    continue;
                }
                bits[base + 2] = Some((lower >> 2) & 1 == 1);

                let shift = base + (j ^ usize::from(expected[3]));
                let upper = j
                    ^ usize::from(expected[3])
                    ^ usize::from(expected[9])
                    ^ usize::from(expected[i]);
                if bits[shift] == Some(upper & 1 == 0) {
                    continue;
                }
                bits[shift] = Some(upper & 1 == 1);
                if bits[shift + 1] == Some((upper >> 1) & 1 == 0) {
                    continue;
                }
                bits[shift + 1] = Some((upper >> 1) & 1 == 1);
                if bits[shift + 2] == Some((upper >> 2) & 1 == 0) {
                    continue;
                }
                bits[shift + 2] = Some((upper >> 2) & 1 == 1);

                ret.extend_from_slice(&rec(expected, &bits, i + 1));
            }
            ret
        }
        Ok(rec(&expected, &bits, 0)
            .into_iter()
            .map(|bits| {
                bits.into_iter()
                    .rev()
                    .map(|b| b.map_or(0, |b| if b { 1 } else { 0 }))
                    .fold(0, |acc, b| acc * 2 + b)
            })
            .min()
            .unwrap())
    }
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 17).unwrap()).unwrap()).unwrap(),
        167430506
    );
    assert_eq!(
        part2(parse(parse::data(2024, 17).unwrap()).unwrap()).unwrap(),
        216148338630253
    );
}
