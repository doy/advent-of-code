use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Inp(Register),
    Add(Register, Value),
    Mul(Register, Value),
    Div(Register, Value),
    Mod(Register, Value),
    Eql(Register, Value),
}

#[derive(Debug, Clone, Copy)]
pub enum Register {
    W,
    X,
    Y,
    Z,
}
use Register::*;

impl Register {
    fn lvalue<'a>(&self, alu: &'a mut Alu) -> &'a mut i64 {
        match self {
            W => &mut alu.w,
            X => &mut alu.x,
            Y => &mut alu.y,
            Z => &mut alu.z,
        }
    }

    fn rvalue(&self, alu: &Alu) -> i64 {
        match self {
            W => alu.w,
            X => alu.x,
            Y => alu.y,
            Z => alu.z,
        }
    }
}

impl std::str::FromStr for Register {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "w" => Ok(W),
            "x" => Ok(X),
            "y" => Ok(Y),
            "z" => Ok(Z),
            _ => Err(anyhow::anyhow!(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Register(Register),
    Number(i64),
}

impl Value {
    fn rvalue(&self, alu: &Alu) -> i64 {
        match self {
            Self::Register(r) => r.rvalue(alu),
            Self::Number(n) => *n,
        }
    }
}

impl std::str::FromStr for Value {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "w" => Ok(Self::Register(W)),
            "x" => Ok(Self::Register(X)),
            "y" => Ok(Self::Register(Y)),
            "z" => Ok(Self::Register(Z)),
            _ => Ok(Self::Number(s.parse()?)),
        }
    }
}

#[derive(Debug)]
pub struct Alu {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Alu {
    fn new() -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn run(&mut self, ops: impl IntoIterator<Item = Op>, inp: i64) {
        self.inp(W, Value::Number(inp));
        for op in ops {
            match op {
                Op::Inp(_) => {
                    break;
                }
                Op::Add(a, b) => {
                    self.add(a, b);
                }
                Op::Mul(a, b) => {
                    self.mul(a, b);
                }
                Op::Div(a, b) => {
                    self.div(a, b);
                }
                Op::Mod(a, b) => {
                    self.modulo(a, b);
                }
                Op::Eql(a, b) => {
                    self.eql(a, b);
                }
            }
        }
    }

    fn inp(&mut self, a: Register, b: Value) {
        *a.lvalue(self) = b.rvalue(self)
    }

    fn add(&mut self, a: Register, b: Value) {
        *a.lvalue(self) += b.rvalue(self);
    }

    fn mul(&mut self, a: Register, b: Value) {
        *a.lvalue(self) *= b.rvalue(self);
    }

    fn div(&mut self, a: Register, b: Value) {
        *a.lvalue(self) /= b.rvalue(self);
    }

    fn modulo(&mut self, a: Register, b: Value) {
        *a.lvalue(self) %= b.rvalue(self);
    }

    fn eql(&mut self, a: Register, b: Value) {
        *a.lvalue(self) = i64::from(a.rvalue(self) == b.rvalue(self));
    }
}

impl std::fmt::Display for Alu {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "Alu {{")?;
        writeln!(f, "    w: {}", self.w)?;
        writeln!(f, "    x: {}", self.x)?;
        writeln!(f, "    y: {}", self.y)?;
        writeln!(f, "    z: {}", self.z)?;
        write!(f, "}}")?;
        Ok(())
    }
}

fn step(
    alu: &mut Alu,
    ops: &mut impl Iterator<Item = Op>,
    val: &mut i64,
    inp: i64,
) -> bool {
    if !(1..=9).contains(&inp) {
        return false;
    }
    *val *= 10;
    *val += inp;
    alu.run(ops, inp);
    true
}

fn run(inp: &[i64], ops: &[Op]) -> Option<i64> {
    let mut val = 0;
    let mut alu = Alu::new();
    let mut ops = ops.iter().copied().skip(1);
    let mut ops = ops.by_ref();

    if !step(&mut alu, &mut ops, &mut val, inp[0]) {
        return None;
    }
    if !step(&mut alu, &mut ops, &mut val, inp[1]) {
        return None;
    }
    if !step(&mut alu, &mut ops, &mut val, inp[2]) {
        return None;
    }
    let z = alu.z;
    if !step(&mut alu, &mut ops, &mut val, (z % 26) - 6) {
        return None;
    }
    if !step(&mut alu, &mut ops, &mut val, inp[3]) {
        return None;
    }
    let z = alu.z;
    if !step(&mut alu, &mut ops, &mut val, (z % 26) - 4) {
        return None;
    }
    if !step(&mut alu, &mut ops, &mut val, inp[4]) {
        return None;
    }
    if !step(&mut alu, &mut ops, &mut val, inp[5]) {
        return None;
    }
    if !step(&mut alu, &mut ops, &mut val, inp[6]) {
        return None;
    }
    let z = alu.z;
    if !step(&mut alu, &mut ops, &mut val, z % 26) {
        return None;
    }
    let z = alu.z;
    if !step(&mut alu, &mut ops, &mut val, z % 26) {
        return None;
    }
    let z = alu.z;
    if !step(&mut alu, &mut ops, &mut val, (z % 26) - 3) {
        return None;
    }
    let z = alu.z;
    if !step(&mut alu, &mut ops, &mut val, (z % 26) - 9) {
        return None;
    }
    let z = alu.z;
    if !step(&mut alu, &mut ops, &mut val, (z % 26) - 9) {
        return None;
    }

    if alu.z == 0 {
        Some(val)
    } else {
        None
    }
}

pub fn parse(fh: File) -> Result<impl Iterator<Item = Op>> {
    Ok(parse::lines(fh).map(|line| {
        let captures = regex_captures!(
            r"(inp|add|mul|div|mod|eql) ([wxyz])(?: ([wxyz]|-?\d+))?",
            &line
        )
        .unwrap();
        match &captures[1] {
            "inp" => Op::Inp(captures[2].parse().unwrap()),
            "add" => Op::Add(
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
            ),
            "mul" => Op::Mul(
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
            ),
            "div" => Op::Div(
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
            ),
            "mod" => Op::Mod(
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
            ),
            "eql" => Op::Eql(
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
            ),
            _ => panic!("unknown opcode: {}", &captures[1]),
        }
    }))
}

pub fn part1(ops: impl Iterator<Item = Op>) -> Result<i64> {
    let ops: Vec<_> = ops.collect();
    for d1 in (1..=9).rev() {
        for d2 in (1..=9).rev() {
            for d3 in (1..=9).rev() {
                for d5 in (1..=9).rev() {
                    for d7 in (1..=9).rev() {
                        for d8 in (1..=9).rev() {
                            for d9 in (1..=9).rev() {
                                let inp = &[d1, d2, d3, d5, d7, d8, d9];
                                let ret = run(inp, &ops);
                                if let Some(n) = ret {
                                    return Ok(n);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    panic!("not found");
}

pub fn part2(ops: impl Iterator<Item = Op>) -> Result<i64> {
    let ops: Vec<_> = ops.collect();
    for d1 in 1..=9 {
        for d2 in 1..=9 {
            for d3 in 1..=9 {
                for d5 in 1..=9 {
                    for d7 in 1..=9 {
                        for d8 in 1..=9 {
                            for d9 in 1..=9 {
                                let inp = &[d1, d2, d3, d5, d7, d8, d9];
                                let ret = run(inp, &ops);
                                if let Some(n) = ret {
                                    return Ok(n);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    panic!("not found");
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2021, 24).unwrap()).unwrap()).unwrap(),
        99299513899971
    );
    assert_eq!(
        part2(parse(parse::data(2021, 24).unwrap()).unwrap()).unwrap(),
        93185111127911
    );
}
