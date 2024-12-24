use advent_of_code::prelude::*;

pub struct Monkey {
    items: VecDeque<u64>,
    op: Box<dyn Fn(u64) -> u64>,
    divisor: u64,
    modulo: u64,
    test: u64,
    if_true: usize,
    if_false: usize,

    count: u64,
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
            r"^  Operation: new = old ([+*]) ([0-9]+|old)$",
            &line
        )
        .unwrap();
        let op = if &cap[2] == "old" {
            match &cap[1] {
                "+" => Box::new(move |x| x + x) as Box<dyn Fn(u64) -> u64>,
                "*" => Box::new(move |x| x * x),
                _ => unreachable!(),
            }
        } else {
            let n: u64 = cap[2].parse().unwrap();
            match &cap[1] {
                "+" => Box::new(move |x| x + n) as Box<dyn Fn(u64) -> u64>,
                "*" => Box::new(move |x| x * n),
                _ => unreachable!(),
            }
        };

        let line = it.next().unwrap();
        let cap = regex_captures!(r"^  Test: divisible by ([0-9]+)$", &line)
            .unwrap();
        let test = cap[1].parse().unwrap();

        let line = it.next().unwrap();
        let cap = regex_captures!(
            r"^    If true: throw to monkey ([0-9]+)$",
            &line
        )
        .unwrap();
        let if_true = cap[1].parse().unwrap();

        let line = it.next().unwrap();
        let cap = regex_captures!(
            r"^    If false: throw to monkey ([0-9]+)$",
            &line
        )
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

    fn inspect(&mut self) -> Option<(u64, usize)> {
        let item = self.items.pop_front()?;
        self.count += 1;
        let item = (self.op)(item);
        let item = item / self.divisor;
        let item = item % self.modulo;
        if item % self.test == 0 {
            Some((item, self.if_true))
        } else {
            Some((item, self.if_false))
        }
    }

    fn catch(&mut self, item: u64) {
        self.items.push_back(item);
    }

    fn set_reduce(&mut self, divisor: u64, modulo: u64) {
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
            let cap =
                regex_captures!(r"^Monkey ([0-9]+):$", &header).unwrap();
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

    fn monkey_business(&self) -> u64 {
        let mut counts: Vec<_> =
            self.monkeys.iter().map(|m| m.count).collect();
        counts.sort_unstable();
        counts[counts.len() - 1] * counts[counts.len() - 2]
    }

    fn set_reduce(&mut self, divisor: u64) {
        let modulo = self.monkeys.iter().map(|m| m.test).product();
        for monkey in &mut self.monkeys {
            monkey.set_reduce(divisor, modulo);
        }
    }
}

pub fn parse(fh: File) -> Result<Monkeys> {
    Ok(Monkeys::parse(parse::raw_lines(fh)))
}

pub fn part1(mut monkeys: Monkeys) -> Result<u64> {
    monkeys.set_reduce(3);
    for _ in 0..20 {
        monkeys.round();
    }
    Ok(monkeys.monkey_business())
}

pub fn part2(mut monkeys: Monkeys) -> Result<u64> {
    monkeys.set_reduce(1);
    for _ in 0..10_000 {
        monkeys.round();
    }
    Ok(monkeys.monkey_business())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 11).unwrap()).unwrap()).unwrap(),
        51075
    );
    assert_eq!(
        part2(parse(parse::data(2022, 11).unwrap()).unwrap()).unwrap(),
        11741456163
    );
}
