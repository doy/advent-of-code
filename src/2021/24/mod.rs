#![allow(dead_code)]
#![allow(unused_variables)]

// i have no idea what i'm doing here, this code doesn't do anything useful

use crate::prelude::*;
use std::rc::Rc;

#[derive(Debug)]
enum Register {
    W,
    X,
    Y,
    Z,
}
use Register::*;

impl Register {
    fn lvalue<'a>(&self, alu: &'a mut Alu) -> &'a mut Rc<Expr> {
        match self {
            W => &mut alu.w,
            X => &mut alu.x,
            Y => &mut alu.y,
            Z => &mut alu.z,
        }
    }

    fn rvalue(&self, alu: &Alu) -> Rc<Expr> {
        match self {
            W => Rc::clone(&alu.w),
            X => Rc::clone(&alu.x),
            Y => Rc::clone(&alu.y),
            Z => Rc::clone(&alu.z),
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

#[derive(Debug)]
enum Value {
    Register(Register),
    Number(i64),
}

impl Value {
    fn rvalue(&self, alu: &Alu) -> Rc<Expr> {
        match self {
            Self::Register(r) => r.rvalue(alu),
            Self::Number(n) => Rc::new(Expr::Num(*n)),
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

#[derive(Debug, PartialEq, Eq)]
enum Expr {
    Num(i64),
    Inp(usize),
    Add(Rc<Expr>, Rc<Expr>),
    Mul(Rc<Expr>, Rc<Expr>),
    Div(Rc<Expr>, Rc<Expr>),
    Mod(Rc<Expr>, Rc<Expr>),
    Eql(Rc<Expr>, Rc<Expr>),
}

impl Expr {
    fn possible(&self) -> std::ops::RangeInclusive<i64> {
        match self {
            Self::Num(n) => *n..=*n,
            Self::Inp(_) => (1..=9),
            Self::Add(a, b) => {
                let ap = a.possible();
                let bp = b.possible();
                (ap.start() + bp.start())..=(ap.end() + bp.end())
            }
            Self::Mul(a, b) => {
                let ap = a.possible();
                let bp = b.possible();
                let a1 = ap.start();
                let a2 = ap.end();
                let b1 = bp.start();
                let b2 = bp.end();
                (a1 * b1).min(a1 * b2).min(a2 * b1).min(a2 * b2)
                    ..=(a1 * b1).max(a1 * b2).max(a2 * b1).max(a2 * b2)
            }
            Self::Div(a, b) => {
                let ap = a.possible();
                let bp = b.possible();
                let a1 = ap.start();
                let a2 = ap.end();
                let b1 = bp.start();
                let b2 = bp.end();
                // TODO
                (-a1.abs().max(a2.abs()))..=(a1.abs().max(a2.abs()))
            }
            Self::Mod(_, b) => 0..=*b.possible().end(),
            Self::Eql(..) => (0..=1),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Num(n) => write!(f, "{}", n),
            Self::Inp(i) => write!(f, "I{}", i),
            Self::Add(a, b) => write!(f, "({} + {})", a, b),
            Self::Mul(a, b) => write!(f, "({} * {})", a, b),
            Self::Div(a, b) => write!(f, "({} / {})", a, b),
            Self::Mod(a, b) => write!(f, "({} % {})", a, b),
            Self::Eql(a, b) => write!(f, "({} == {})", a, b),
        }
    }
}

#[derive(Debug)]
pub struct Alu {
    w: Rc<Expr>,
    x: Rc<Expr>,
    y: Rc<Expr>,
    z: Rc<Expr>,

    inp_idx: usize,
}

impl Alu {
    fn new() -> Self {
        Self {
            w: Rc::new(Expr::Num(0)),
            x: Rc::new(Expr::Num(0)),
            y: Rc::new(Expr::Num(0)),
            z: Rc::new(Expr::Num(0)),

            inp_idx: 0,
        }
    }

    fn run(&mut self, lines: impl IntoIterator<Item = String>) {
        for (i, line) in lines.into_iter().enumerate() {
            // eprintln!("{}: {}", i, self);
            let captures = regex_captures!(
                r"(inp|add|mul|div|mod|eql) ([wxyz])(?: ([wxyz]|-?\d+))?",
                &line
            )
            .unwrap();
            match &captures[1] {
                "inp" => {
                    self.inp(captures[2].parse().unwrap());
                }
                "add" => {
                    self.add(
                        captures[2].parse().unwrap(),
                        captures[3].parse().unwrap(),
                    );
                }
                "mul" => {
                    self.mul(
                        captures[2].parse().unwrap(),
                        captures[3].parse().unwrap(),
                    );
                }
                "div" => {
                    self.div(
                        captures[2].parse().unwrap(),
                        captures[3].parse().unwrap(),
                    );
                }
                "mod" => {
                    self.modulo(
                        captures[2].parse().unwrap(),
                        captures[3].parse().unwrap(),
                    );
                }
                "eql" => {
                    self.eql(
                        captures[2].parse().unwrap(),
                        captures[3].parse().unwrap(),
                    );
                }
                _ => panic!("unknown opcode: {}", &captures[1]),
            }
        }
    }

    fn inp(&mut self, a: Register) {
        *a.lvalue(self) = Rc::new(Expr::Inp(self.inp_idx));
        self.inp_idx += 1;
    }

    fn add(&mut self, a: Register, b: Value) {
        *a.lvalue(self) = Rc::new(Expr::Add(a.rvalue(self), b.rvalue(self)));
    }

    fn mul(&mut self, a: Register, b: Value) {
        *a.lvalue(self) = Rc::new(Expr::Mul(a.rvalue(self), b.rvalue(self)));
    }

    fn div(&mut self, a: Register, b: Value) {
        *a.lvalue(self) = Rc::new(Expr::Div(a.rvalue(self), b.rvalue(self)));
    }

    fn modulo(&mut self, a: Register, b: Value) {
        *a.lvalue(self) = Rc::new(Expr::Mod(a.rvalue(self), b.rvalue(self)));
    }

    fn eql(&mut self, a: Register, b: Value) {
        *a.lvalue(self) = Rc::new(Expr::Eql(a.rvalue(self), b.rvalue(self)));
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

fn simplify(r: &Rc<Expr>) -> Rc<Expr> {
    let simpler = match &**r {
        Expr::Num(_) | Expr::Inp(_) => {
            return Rc::clone(r);
        }
        Expr::Add(a, b) => {
            if let (Expr::Num(a_num), Expr::Num(b_num)) = (&**a, &**b) {
                Rc::new(Expr::Num(a_num + b_num))
            } else if **a == Expr::Num(0) {
                simplify(b)
            } else if **b == Expr::Num(0) {
                simplify(a)
            } else if let (Expr::Num(a_num), Expr::Add(aa, ab)) = (&**a, &**b)
            {
                if let Expr::Num(aa_num) = &**aa {
                    Rc::new(Expr::Add(
                        simplify(ab),
                        Rc::new(Expr::Num(a_num + aa_num)),
                    ))
                } else if let Expr::Num(ab_num) = &**ab {
                    Rc::new(Expr::Add(
                        simplify(aa),
                        Rc::new(Expr::Num(a_num + ab_num)),
                    ))
                } else {
                    Rc::new(Expr::Add(simplify(a), simplify(b)))
                }
            } else if let (Expr::Add(aa, ab), Expr::Num(b_num)) = (&**a, &**b)
            {
                if let Expr::Num(aa_num) = &**aa {
                    Rc::new(Expr::Add(
                        simplify(ab),
                        Rc::new(Expr::Num(b_num + aa_num)),
                    ))
                } else if let Expr::Num(ab_num) = &**ab {
                    Rc::new(Expr::Add(
                        simplify(aa),
                        Rc::new(Expr::Num(b_num + ab_num)),
                    ))
                } else {
                    Rc::new(Expr::Add(simplify(a), simplify(b)))
                }
            } else {
                Rc::new(Expr::Add(simplify(a), simplify(b)))
            }
        }
        Expr::Mul(a, b) => {
            if let (Expr::Num(a_num), Expr::Num(b_num)) = (&**a, &**b) {
                Rc::new(Expr::Num(a_num * b_num))
            } else if **a == Expr::Num(0) || **b == Expr::Num(0) {
                Rc::new(Expr::Num(0))
            } else if **a == Expr::Num(1) {
                simplify(b)
            } else if **b == Expr::Num(1) {
                simplify(a)
            } else {
                Rc::new(Expr::Mul(simplify(a), simplify(b)))
            }
        }
        Expr::Div(a, b) => {
            if let (Expr::Num(a_num), Expr::Num(b_num)) = (&**a, &**b) {
                Rc::new(Expr::Num(a_num / b_num))
            } else if **a == Expr::Num(0) {
                Rc::new(Expr::Num(0))
            } else if **b == Expr::Num(1) {
                simplify(a)
            } else {
                (|| {
                    if let (Expr::Add(aa, ab), Expr::Num(b_num)) =
                        (&**a, &**b)
                    {
                        if let Expr::Mul(ma, mb) = &**aa {
                            if let Expr::Num(mb_num) = &**mb {
                                if b_num == mb_num {
                                    return Rc::new(Expr::Add(
                                        simplify(ma),
                                        Rc::new(Expr::Div(
                                            simplify(ab),
                                            simplify(b),
                                        )),
                                    ));
                                }
                            }
                        }
                    }
                    if let Expr::Num(b_num) = &**b {
                        let ap = a.possible();
                        if *ap.start() >= 0 && *ap.end() < *b_num {
                            return simplify(a);
                        }
                    }
                    Rc::new(Expr::Div(simplify(a), simplify(b)))
                })()
            }
        }
        Expr::Mod(a, b) => {
            if let (Expr::Num(a_num), Expr::Num(b_num)) = (&**a, &**b) {
                Rc::new(Expr::Num(a_num % b_num))
            } else if **a == Expr::Num(0) {
                Rc::new(Expr::Num(0))
            } else {
                (|| {
                    if let (Expr::Add(aa, ab), Expr::Num(b_num)) =
                        (&**a, &**b)
                    {
                        if let Expr::Mul(ma, mb) = &**aa {
                            if let Expr::Num(mb_num) = &**mb {
                                if b_num == mb_num {
                                    return Rc::new(Expr::Mod(
                                        simplify(ab),
                                        simplify(b),
                                    ));
                                }
                            }
                        }
                    }
                    if let Expr::Num(b_num) = &**b {
                        let ap = a.possible();
                        if *ap.start() >= 0 && *ap.end() < *b_num {
                            return simplify(a);
                        }
                    }
                    Rc::new(Expr::Mod(simplify(a), simplify(b)))
                })()
            }
        }
        Expr::Eql(a, b) => {
            if let (Expr::Num(a_num), Expr::Num(b_num)) = (&**a, &**b) {
                Rc::new(Expr::Num(if a_num == b_num { 1 } else { 0 }))
            } else {
                let ap = a.possible();
                let bp = b.possible();
                if ap.end() < bp.start() || bp.end() < ap.start() {
                    Rc::new(Expr::Num(0))
                } else {
                    Rc::new(Expr::Eql(simplify(a), simplify(b)))
                }
            }
        }
    };
    let possible = simpler.possible();
    if possible.start() == possible.end() {
        Rc::new(Expr::Num(*possible.start()))
    } else {
        simpler
    }
}

pub fn parse(fh: File) -> Result<impl Iterator<Item = String>> {
    Ok(parse::lines(fh))
}

pub fn part1(lines: impl Iterator<Item = String>) -> Result<i64> {
    let mut alu = Alu::new();
    alu.run(lines);
    let mut z = Rc::clone(&alu.z);
    let mut i = 0;
    loop {
        i += 1;
        eprintln!("{}", i);
        let simpler = simplify(&z);
        if simpler == z {
            break;
        }
        z = simpler;
    }
    println!("{}", z);
    todo!()
}

pub fn part2(_: impl Iterator<Item = String>) -> Result<i64> {
    todo!()
}

#[test]
fn test() {
    // assert_eq!(
    //     part1(parse(parse::data(2021, 24).unwrap()).unwrap()).unwrap(),
    //     0
    // );
    // assert_eq!(
    //     part2(parse(parse::data(2021, 24).unwrap()).unwrap()).unwrap(),
    //     0
    // );
}
