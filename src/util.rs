use anyhow::Context as _;
use std::io::BufRead as _;

pub fn read_ints(filename: &str) -> anyhow::Result<Vec<i32>> {
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
