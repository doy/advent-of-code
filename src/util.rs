use anyhow::Context as _;
use std::io::{BufRead as _, Read as _};

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

pub fn read_file(filename: &str) -> anyhow::Result<Vec<u8>> {
    let mut f = std::fs::File::open(filename)
        .with_context(|| format!("couldn't find data file {}", filename))?;
    let mut s = vec![];
    f.read_to_end(&mut s)
        .context("failed to read map contents")?;
    Ok(s)
}

pub fn read_file_str(filename: &str) -> anyhow::Result<String> {
    let mut f = std::fs::File::open(filename)
        .with_context(|| format!("couldn't find data file {}", filename))?;
    let mut s = String::new();
    f.read_to_string(&mut s)
        .context("failed to read map contents")?;
    Ok(s)
}
