use anyhow::Context as _;
use std::convert::TryInto as _;

pub struct Line {
    c: char,
    n1: usize,
    n2: usize,
    password: String,
}

impl Line {
    fn parse(line: &str) -> anyhow::Result<Self> {
        let rx = regex::Regex::new(r"^([0-9]+)-([0-9]+) (.): (.*)$").unwrap();
        let captures =
            rx.captures(line).context("line failed to match regex")?;
        let c = captures
            .get(3)
            .unwrap()
            .as_str()
            .parse()
            .context("invalid policy char")?;
        let n1 = captures
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .context("invalid policy lower bound")?;
        let n2 = captures
            .get(2)
            .unwrap()
            .as_str()
            .parse()
            .context("invalid policy upper bound")?;
        let password = captures.get(4).unwrap().as_str().to_string();
        Ok(Self {
            c,
            n1,
            n2,
            password,
        })
    }

    fn valid_part_1(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.c).count();
        count >= self.n1 && count <= self.n2
    }

    fn valid_part_2(&self) -> bool {
        (self.password.chars().nth(self.n1 - 1) == Some(self.c))
            ^ (self.password.chars().nth(self.n2 - 1) == Some(self.c))
    }
}

pub fn parse(
    fh: std::fs::File,
) -> anyhow::Result<impl Iterator<Item = Line>> {
    Ok(crate::util::parse::lines(fh).map(|line| Line::parse(&line).unwrap()))
}

pub fn part1(lines: impl Iterator<Item = Line>) -> anyhow::Result<i64> {
    let count = lines.filter(|l| l.valid_part_1()).count();
    Ok(count.try_into()?)
}

pub fn part2(lines: impl Iterator<Item = Line>) -> anyhow::Result<i64> {
    let count = lines.filter(|l| l.valid_part_2()).count();
    Ok(count.try_into()?)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(crate::util::data(2020, 2).unwrap()).unwrap()).unwrap(),
        638
    );
    assert_eq!(
        part2(parse(crate::util::data(2020, 2).unwrap()).unwrap()).unwrap(),
        699
    );
}
