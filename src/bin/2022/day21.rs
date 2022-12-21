#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

#[derive(Clone)]
pub enum Monkey {
    Num(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Human,
}

pub struct NamedMonkey(String, Monkey);

impl std::str::FromStr for NamedMonkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = regex_captures!(r"^(\w+): (\d+|\w+ [+*/-] \w+)$", s)
            .ok_or_else(|| anyhow!("failed to parse"))?;
        let name = cap[1].to_string();
        if let Ok(n) = cap[2].parse::<i64>() {
            Ok(NamedMonkey(name, Monkey::Num(n)))
        } else {
            let mut parts = cap[2].split_whitespace();
            let monkey1 = parts.next().unwrap();
            let op = parts.next().unwrap();
            let monkey2 = parts.next().unwrap();
            match op {
                "+" => Ok(NamedMonkey(
                    name,
                    Monkey::Add(monkey1.to_string(), monkey2.to_string()),
                )),
                "-" => Ok(NamedMonkey(
                    name,
                    Monkey::Sub(monkey1.to_string(), monkey2.to_string()),
                )),
                "*" => Ok(NamedMonkey(
                    name,
                    Monkey::Mul(monkey1.to_string(), monkey2.to_string()),
                )),
                "/" => Ok(NamedMonkey(
                    name,
                    Monkey::Div(monkey1.to_string(), monkey2.to_string()),
                )),
                _ => unreachable!(),
            }
        }
    }
}

impl Monkey {
    fn eval(&self, monkeys: &HashMap<String, Monkey>) -> Option<i64> {
        match self {
            Self::Num(n) => Some(*n),
            Self::Add(n, m) => {
                monkeys.get(n).unwrap().eval(monkeys).and_then(|n| {
                    monkeys.get(m).unwrap().eval(monkeys).map(|m| n + m)
                })
            }
            Self::Sub(n, m) => {
                monkeys.get(n).unwrap().eval(monkeys).and_then(|n| {
                    monkeys.get(m).unwrap().eval(monkeys).map(|m| n - m)
                })
            }
            Self::Mul(n, m) => {
                monkeys.get(n).unwrap().eval(monkeys).and_then(|n| {
                    monkeys.get(m).unwrap().eval(monkeys).map(|m| n * m)
                })
            }
            Self::Div(n, m) => {
                monkeys.get(n).unwrap().eval(monkeys).and_then(|n| {
                    monkeys.get(m).unwrap().eval(monkeys).map(|m| n / m)
                })
            }
            Self::Human => None,
        }
    }

    fn is_num(&self) -> bool {
        matches!(self, Monkey::Num(_))
    }

    fn invert(&self, monkeys: &HashMap<String, Monkey>, mut val: i64) -> i64 {
        let mut monkey = self;
        loop {
            match monkey {
                Monkey::Num(n) => panic!("found num?"),
                Monkey::Add(n, m) => {
                    let monkey1 = monkeys.get(n).unwrap();
                    let monkey2 = monkeys.get(m).unwrap();
                    if let Monkey::Num(n) = monkey1 {
                        monkey = monkey2;
                        val -= n;
                    } else if let Monkey::Num(n) = monkey2 {
                        monkey = monkey1;
                        val -= n;
                    } else {
                        panic!("not simplified?");
                    }
                }
                Monkey::Sub(n, m) => {
                    let monkey1 = monkeys.get(n).unwrap();
                    let monkey2 = monkeys.get(m).unwrap();
                    if let Monkey::Num(n) = monkey1 {
                        monkey = monkey2;
                        val = n - val;
                    } else if let Monkey::Num(n) = monkey2 {
                        monkey = monkey1;
                        val += n;
                    } else {
                        panic!("not simplified?");
                    }
                }
                Monkey::Mul(n, m) => {
                    let monkey1 = monkeys.get(n).unwrap();
                    let monkey2 = monkeys.get(m).unwrap();
                    if let Monkey::Num(n) = monkey1 {
                        monkey = monkey2;
                        val /= n;
                    } else if let Monkey::Num(n) = monkey2 {
                        monkey = monkey1;
                        val /= n;
                    } else {
                        panic!("not simplified?");
                    }
                }
                Monkey::Div(n, m) => {
                    let monkey1 = monkeys.get(n).unwrap();
                    let monkey2 = monkeys.get(m).unwrap();
                    if let Monkey::Num(n) = monkey1 {
                        monkey = monkey2;
                        val = n / val;
                    } else if let Monkey::Num(n) = monkey2 {
                        monkey = monkey1;
                        val *= n;
                    } else {
                        panic!("not simplified?");
                    }
                }
                Monkey::Human => return val,
            }
        }
    }
}

pub fn parse(fh: File) -> Result<HashMap<String, Monkey>> {
    Ok(parse::lines(fh)
        .map(|NamedMonkey(name, monkey)| (name, monkey))
        .collect())
}

pub fn part1(monkeys: HashMap<String, Monkey>) -> Result<i64> {
    Ok(monkeys
        .get("root")
        .ok_or_else(|| anyhow!("couldn't find root"))?
        .eval(&monkeys)
        .unwrap())
}

pub fn part2(mut monkeys: HashMap<String, Monkey>) -> Result<i64> {
    monkeys.insert("humn".to_string(), Monkey::Human);
    simplify(&mut monkeys);
    let root = monkeys
        .get("root")
        .ok_or_else(|| anyhow!("couldn't find root"))?;
    let (monkey1, monkey2) = match root {
        Monkey::Add(n, m) => {
            (monkeys.get(n).unwrap(), monkeys.get(m).unwrap())
        }
        Monkey::Sub(n, m) => {
            (monkeys.get(n).unwrap(), monkeys.get(m).unwrap())
        }
        Monkey::Mul(n, m) => {
            (monkeys.get(n).unwrap(), monkeys.get(m).unwrap())
        }
        Monkey::Div(n, m) => {
            (monkeys.get(n).unwrap(), monkeys.get(m).unwrap())
        }
        _ => panic!("wrong root"),
    };
    let (val, monkey_human) = if let Monkey::Num(n) = monkey1 {
        assert!(!matches!(monkey2, Monkey::Num(_)));
        (n, monkey2)
    } else if let Monkey::Num(n) = monkey2 {
        assert!(!matches!(monkey1, Monkey::Num(_)));
        (n, monkey1)
    } else {
        panic!("wrong monkeys")
    };
    Ok(monkey_human.invert(&monkeys, *val))
}

fn simplify(monkeys: &mut HashMap<String, Monkey>) {
    loop {
        let mut found = None;
        for (name, monkey) in monkeys.iter() {
            if matches!(monkey, Monkey::Num(_) | Monkey::Human) {
                continue;
            }
            if let Some(n) = monkey.eval(monkeys) {
                found = Some((name.to_string(), n));
                break;
            }
        }
        if let Some((name, n)) = found {
            *monkeys.get_mut(&name).unwrap() = Monkey::Num(n);
        } else {
            break;
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 21).unwrap()).unwrap()).unwrap(),
        10037517593724
    );
    assert_eq!(
        part2(parse(parse::data(2022, 21).unwrap()).unwrap()).unwrap(),
        3272260914328
    );
}
