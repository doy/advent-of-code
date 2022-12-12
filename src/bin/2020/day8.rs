use advent_of_code::prelude::*;

#[derive(Clone, Copy)]
enum OpType {
    Nop,
    Acc,
    Jmp,
}

impl std::str::FromStr for OpType {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "nop" => Self::Nop,
            "acc" => Self::Acc,
            "jmp" => Self::Jmp,
            _ => return Err(anyhow!("invalid optype {}", s)),
        })
    }
}

#[derive(Clone, Copy)]
pub struct Op {
    ty: OpType,
    arg: i64,
}

impl std::str::FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let captures = regex_captures!(r"^([^ ]*) ((?:-|\+)[0-9]+)$", s)
            .context("failed to parse line")?;
        let ty = captures.get(1).unwrap().as_str().parse()?;
        let arg = captures
            .get(2)
            .unwrap()
            .as_str()
            .parse()
            .context("invalid arg")?;
        Ok(Self { ty, arg })
    }
}

pub fn parse(fh: File) -> Result<Vec<Op>> {
    Ok(parse::lines(fh).collect())
}

pub fn part1(opcodes: Vec<Op>) -> Result<i64> {
    let (acc, success) = run(&opcodes)?;
    if success {
        return Err(anyhow!("unexpectedly succeeded"));
    }
    Ok(acc)
}

pub fn part2(opcodes: Vec<Op>) -> Result<i64> {
    for i in 0..opcodes.len() {
        match opcodes[i].ty {
            OpType::Nop => {
                let mut attempt = opcodes.clone();
                attempt[i].ty = OpType::Jmp;
                let (acc, success) = run(&attempt)?;
                if success {
                    return Ok(acc);
                }
            }
            OpType::Acc => {}
            OpType::Jmp => {
                let mut attempt = opcodes.clone();
                attempt[i].ty = OpType::Nop;
                let (acc, success) = run(&attempt)?;
                if success {
                    return Ok(acc);
                }
            }
        }
    }
    Err(anyhow!("failed to find corrupted opcode"))
}

fn run(opcodes: &[Op]) -> Result<(i64, bool)> {
    let mut seen = vec![false; opcodes.len()];
    let mut pc = 0;
    let mut acc = 0;
    loop {
        if pc >= opcodes.len() {
            return Ok((acc, true));
        } else if seen[pc] {
            return Ok((acc, false));
        }
        seen[pc] = true;

        match opcodes[pc].ty {
            OpType::Nop => {
                pc += 1;
            }
            OpType::Acc => {
                acc += opcodes[pc].arg;
                pc += 1;
            }
            OpType::Jmp => {
                let arg = opcodes[pc].arg;
                if arg >= 0 {
                    if arg as usize > opcodes.len()
                        || pc > opcodes.len() - arg as usize
                    {
                        return Err(anyhow!("invalid jmp"));
                    }
                    pc += arg as usize;
                } else {
                    if pc < (-arg as usize) {
                        return Err(anyhow!("invalid jmp"));
                    }
                    pc -= -arg as usize;
                }
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2020, 8).unwrap()).unwrap()).unwrap(),
        1928
    );
    assert_eq!(
        part2(parse(parse::data(2020, 8).unwrap()).unwrap()).unwrap(),
        1319
    );
}
