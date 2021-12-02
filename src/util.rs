use anyhow::Context as _;
use std::io::{BufRead as _, Read as _};

macro_rules! data {
    () => {{
        use anyhow::Context as _;
        let file = crate::util::src_file_to_data_file(&std::file!());
        std::fs::File::open(file.clone())
            .with_context(|| format!("couldn't find data file {}", file))
    }};
}

macro_rules! data_ints {
    () => {
        crate::util::read_ints(&crate::util::src_file_to_data_file(
            &std::file!(),
        ))
    };
}

macro_rules! data_bytes {
    () => {
        crate::util::read_file(&crate::util::src_file_to_data_file(
            &std::file!(),
        ))
    };
}

macro_rules! data_str {
    () => {
        crate::util::read_file_str(&crate::util::src_file_to_data_file(
            &std::file!(),
        ))
    };
}

pub fn src_file_to_data_file(file: &str) -> String {
    let parts: Vec<_> = file.split('/').collect();
    format!(
        "data/{}/{}.txt",
        parts[parts.len() - 3],
        parts[parts.len() - 2]
    )
}

pub fn read_ints(filename: &str) -> anyhow::Result<Vec<i64>> {
    let f = std::fs::File::open(filename)
        .with_context(|| format!("couldn't find data file {}", filename))?;
    let f = std::io::BufReader::new(f);
    let ints: anyhow::Result<Vec<_>> = f
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
