use anyhow::Context as _;
use std::io::BufRead as _;

pub fn run(day: u8, puzzle: u8) -> anyhow::Result<()> {
    match (day, puzzle) {
        (1, 1) => report_repair(),
        _ => Err(anyhow::anyhow!("unknown puzzle {}-{}", day, puzzle)),
    }
}

fn report_repair() -> anyhow::Result<()> {
    let f = std::fs::File::open("data/1-1.txt")
        .context("couldn't find data file")?;
    let f = std::io::BufReader::new(f);
    let ints: anyhow::Result<Vec<i32>> = f
        .lines()
        .map(|l| {
            l.context("failed to read a line")?
                .parse()
                .context("failed to parse line into an integer")
        })
        .collect();
    let ints = ints?;
    for i in &ints {
        for j in &ints {
            if i + j == 2020 {
                println!("{} + {} = {}", i, j, i * j);
                return Ok(());
            }
        }
    }
    Err(anyhow::anyhow!("no numbers summing to 2020 found"))
}
