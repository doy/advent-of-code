use std::io::{BufRead as _, Read as _};

macro_rules! data {
    () => {{
        use anyhow::Context as _;
        let file = crate::util::parse::src_file_to_data_file(&std::file!());
        std::fs::File::open(file.clone())
            .with_context(|| format!("couldn't find data file {}", file))
    }};
}

macro_rules! data_lines {
    () => {
        data!().map(|fh| crate::util::parse::lines(fh))
    };
}

macro_rules! data_ints {
    () => {
        data!()
            .map(|fh| crate::util::parse::ints(crate::util::parse::lines(fh)))
    };
    ($sep:expr) => {
        data!().map(|fh| {
            crate::util::parse::ints(crate::util::parse::split(fh, $sep))
        })
    };
}

macro_rules! data_bytes {
    () => {
        data!().map(|fh| crate::util::parse::bytes(fh))
    };
}

macro_rules! data_str {
    () => {{
        data!().map(|fh| crate::util::parse::string(fh))
    }};
}

macro_rules! data_bool_grid {
    ($t:expr, $f:expr) => {{
        crate::util::parse::bool_grid(data_lines!().unwrap(), $t, $f)
    }};
}

macro_rules! data_digit_grid {
    () => {{
        crate::util::parse::digit_grid(data_lines!().unwrap())
    }};
}

pub fn src_file_to_data_file(file: &str) -> String {
    let parts: Vec<_> = file.split('/').collect();
    format!(
        "data/{}/{}.txt",
        parts[parts.len() - 3],
        parts[parts.len() - 2]
    )
}

pub fn lines(fh: std::fs::File) -> impl Iterator<Item = String> {
    let fh = std::io::BufReader::new(fh);
    fh.lines().map(|res| res.unwrap())
}

pub fn split(fh: std::fs::File, sep: u8) -> impl Iterator<Item = String> {
    let fh = std::io::BufReader::new(fh);
    fh.split(sep)
        .map(|res| String::from_utf8(res.unwrap()).unwrap())
}

pub fn ints(iter: impl Iterator<Item = String>) -> impl Iterator<Item = i64> {
    iter.map(|s| s.trim().parse().unwrap())
}

pub fn bytes(fh: std::fs::File) -> impl Iterator<Item = u8> {
    fh.bytes().map(|res| res.unwrap())
}

pub fn string(fh: std::fs::File) -> String {
    let bytes: Vec<_> = bytes(fh).collect();
    String::from_utf8(bytes).unwrap()
}

pub fn bool_grid(
    lines: impl Iterator<Item = String>,
    t: u8,
    f: u8,
) -> crate::util::grid::Grid<bool> {
    lines
        .map(|s| {
            s.as_bytes()
                .iter()
                .copied()
                .map(|b| {
                    if b == f {
                        false
                    } else if b == t {
                        true
                    } else {
                        panic!("unrecognized character {}", char::from(b))
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn digit_grid(
    lines: impl Iterator<Item = String>,
) -> crate::util::grid::Grid<u8> {
    lines
        .map(|s| {
            s.as_bytes()
                .iter()
                .copied()
                .map(|b| b - b'0')
                .collect::<Vec<_>>()
        })
        .collect()
}
