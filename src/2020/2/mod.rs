use anyhow::Context as _;
use std::io::BufRead as _;

struct Line {
    c: char,
    min_times: usize,
    max_times: usize,
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
        let min_times = captures
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .context("invalid policy lower bound")?;
        let max_times = captures
            .get(2)
            .unwrap()
            .as_str()
            .parse()
            .context("invalid policy upper bound")?;
        let password = captures.get(4).unwrap().as_str().to_string();
        Ok(Self {
            c,
            min_times,
            max_times,
            password,
        })
    }

    fn valid(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.c).count();
        count >= self.min_times && count <= self.max_times
    }
}

pub fn part1() -> anyhow::Result<()> {
    let f = std::fs::File::open("data/2.txt")
        .context("couldn't find data file 2.txt")?;
    let f = std::io::BufReader::new(f);
    let lines = f
        .lines()
        .map(|l| Line::parse(&l.context("failed to read a line")?))
        .collect::<anyhow::Result<Vec<Line>>>()?;
    let count = lines.iter().filter(|l| l.valid()).count();
    println!("{}", count);
    Ok(())
}

pub fn part2() -> anyhow::Result<()> {
    todo!()
}
