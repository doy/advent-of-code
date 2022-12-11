#![allow(dead_code)]
#![allow(unused_variables)]

use crate::prelude::*;

pub struct Monkey {
    items: VecDeque<i64>,
    op: Box<dyn Fn(i64) -> i64>,
    divisor: i64,
    modulo: i64,
    test: i64,
    if_true: usize,
    if_false: usize,

    count: i64,
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .finish()
    }
}

impl Monkey {
    fn parse(mut it: impl Iterator<Item = String>) -> Monkey {
        let line = it.next().unwrap();
        let cap = regex_captures!(r"^  Starting items: ([0-9, ]*)$", &line)
            .unwrap();
        let items = cap[1].split(", ").map(|s| s.parse().unwrap()).collect();

        let line = it.next().unwrap();
        let cap = regex_captures!(
            r"^  Operation: new = old ([+*/-]) (\d+|old)$",
            &line
        )
        .unwrap();
        let op = if &cap[2] == "old" {
            match &cap[1] {
                "+" => Box::new(move |x| x + x) as Box<dyn Fn(i64) -> i64>,
                "-" => Box::new(move |_| 0.into()),
                "*" => Box::new(move |x| x * x),
                "/" => Box::new(move |_| 1.into()),
                _ => panic!("unknown op {}", &cap[1]),
            }
        } else {
            let n: i64 = cap[2].parse().unwrap();
            match &cap[1] {
                "+" => Box::new(move |x| x + n) as Box<dyn Fn(i64) -> i64>,
                "-" => Box::new(move |x| x - n),
                "*" => Box::new(move |x| x * n),
                "/" => Box::new(move |x| x / n),
                _ => panic!("unknown op {}", &cap[1]),
            }
        };

        let line = it.next().unwrap();
        let cap =
            regex_captures!(r"^  Test: divisible by (\d+)$", &line).unwrap();
        let test = cap[1].parse().unwrap();

        let line = it.next().unwrap();
        let cap =
            regex_captures!(r"^    If true: throw to monkey (\d+)$", &line)
                .unwrap();
        let if_true = cap[1].parse().unwrap();

        let line = it.next().unwrap();
        let cap =
            regex_captures!(r"^    If false: throw to monkey (\d+)$", &line)
                .unwrap();
        let if_false = cap[1].parse().unwrap();

        assert!(it.next().is_none());

        Self {
            items,
            op,
            divisor: 0,
            modulo: 0,
            test,
            if_true,
            if_false,
            count: 0,
        }
    }

    fn inspect(&mut self) -> Option<(i64, usize)> {
        let Some(item) = self.items.pop_front()
        else { return None };
        self.count += 1;
        let item = (self.op)(item);
        let item = item / self.divisor;
        let item = item % self.modulo;
        if item % self.test == 0.into() {
            Some((item, self.if_true))
        } else {
            Some((item, self.if_false))
        }
    }

    fn catch(&mut self, item: i64) {
        self.items.push_back(item);
    }

    fn set_reduce(&mut self, divisor: i64, modulo: i64) {
        self.divisor = divisor;
        self.modulo = modulo;
    }
}

#[derive(Debug)]
pub struct Monkeys {
    monkeys: Vec<Monkey>,
}

impl Monkeys {
    fn parse(it: impl Iterator<Item = String>) -> Self {
        let mut monkeys = vec![];
        let mut it = it.peekable();
        while it.peek().is_some() {
            let header = it.next().unwrap();
            let cap = regex_captures!(r"^Monkey (\d+):$", &header).unwrap();
            let monkey_idx: usize = cap[1].parse().unwrap();
            assert_eq!(monkey_idx, monkeys.len());
            monkeys.push(Monkey::parse(parse::chunk(&mut it)));
        }
        Self { monkeys }
    }

    fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            while let Some((item, to)) = self.monkeys[i].inspect() {
                self.monkeys[to].catch(item);
            }
        }
    }

    fn monkey_business(&self) -> i64 {
        let mut counts: Vec<_> =
            self.monkeys.iter().map(|m| m.count).collect();
        counts.sort_unstable();
        counts[counts.len() - 1] * counts[counts.len() - 2]
    }

    fn set_reduce(&mut self, divisor: i64) {
        let modulo = self.monkeys.iter().map(|m| m.test).product();
        for monkey in &mut self.monkeys {
            monkey.set_reduce(divisor, modulo);
        }
    }
}

pub fn parse(fh: File) -> Result<Monkeys> {
    Ok(Monkeys::parse(parse::lines(fh)))
}

pub fn part1(mut monkeys: Monkeys) -> Result<i64> {
    monkeys.set_reduce(3);
    for i in 0..20 {
        monkeys.round();
    }
    Ok(monkeys.monkey_business())
}

pub fn part2(mut monkeys: Monkeys) -> Result<i64> {
    monkeys.set_reduce(1);
    for i in 0..10_000 {
        monkeys.round();
    }
    Ok(monkeys.monkey_business())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 11).unwrap()).unwrap()).unwrap(),
        0
    );
    assert_eq!(
        part2(parse(parse::data(2022, 11).unwrap()).unwrap()).unwrap(),
        0
    );
}
