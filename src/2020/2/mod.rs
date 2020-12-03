use anyhow::Context as _;
use std::io::BufRead as _;

struct Line {
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

pub fn part1() -> anyhow::Result<()> {
    let lines = read_lines()?;
    let count = lines.iter().filter(|l| l.valid_part_1()).count();
    println!("{}", count);
    Ok(())
}

pub fn part2() -> anyhow::Result<()> {
    let lines = read_lines()?;
    let count = lines.iter().filter(|l| l.valid_part_2()).count();
    println!("{}", count);
    Ok(())
}

fn read_lines() -> anyhow::Result<Vec<Line>> {
    let f = std::fs::File::open("data/2.txt")
        .context("couldn't find data file 2.txt")?;
    let f = std::io::BufReader::new(f);
    f.lines()
        .map(|l| Line::parse(&l.context("failed to read a line")?))
        .collect()
}
