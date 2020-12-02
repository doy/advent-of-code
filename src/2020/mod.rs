use anyhow::Context as _;
use std::io::BufRead as _;

pub fn run(day: u8, puzzle: u8) -> anyhow::Result<()> {
    match (day, puzzle) {
        (1, 1) => report_repair(),
        (1, 2) => report_repair_2(),
        _ => Err(anyhow::anyhow!("unknown puzzle {}-{}", day, puzzle)),
    }
}

fn report_repair() -> anyhow::Result<()> {
    let ints = read_ints("data/1.txt")?;
    for i in &ints {
        for j in &ints {
            if i + j == 2020 {
                println!("{} * {} = {}", i, j, i * j);
                return Ok(());
            }
        }
    }
    Err(anyhow::anyhow!("no numbers summing to 2020 found"))
}

fn report_repair_2() -> anyhow::Result<()> {
    let ints = read_ints("data/1.txt")?;
    for i in &ints {
        for j in &ints {
            for k in &ints {
                if i + j + k == 2020 {
                    println!("{} * {} * {} = {}", i, j, k, i * j * k);
                    return Ok(());
                }
            }
        }
    }
    Err(anyhow::anyhow!("no numbers summing to 2020 found"))
}

fn read_ints(filename: &str) -> anyhow::Result<Vec<i32>> {
    let f = std::fs::File::open(filename)
        .with_context(|| format!("couldn't find data file {}", filename))?;
    let f = std::io::BufReader::new(f);
    let ints: anyhow::Result<Vec<i32>> = f
        .lines()
        .map(|l| {
            l.context("failed to read a line")?
                .parse()
                .context("failed to parse line into an integer")
        })
        .collect();
    ints
}
