use std::io::{BufRead as _, Read as _};

macro_rules! data {
    () => {{
        use anyhow::Context as _;
        let file = crate::util::src_file_to_data_file(&std::file!());
        std::fs::File::open(file.clone())
            .with_context(|| format!("couldn't find data file {}", file))
    }};
}

macro_rules! data_lines {
    () => {
        data!().map(|fh| crate::util::lines(fh))
    };
}

macro_rules! data_ints {
    () => {
        data!().map(|fh| crate::util::ints_by_line(fh))
    };
    ($sep:expr) => {
        data!().map(|fh| crate::util::ints_by_split(fh, $sep))
    };
}

macro_rules! data_bytes {
    () => {
        data!().map(|fh| crate::util::bytes(fh))
    };
}

macro_rules! data_str {
    () => {{
        data!().map(|fh| crate::util::string(fh))
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

pub fn ints_by_line(fh: std::fs::File) -> impl Iterator<Item = i64> {
    lines(fh).map(|l| l.parse().unwrap())
}

pub fn ints_by_split(
    fh: std::fs::File,
    sep: u8,
) -> impl Iterator<Item = i64> {
    let fh = std::io::BufReader::new(fh);
    fh.split(sep).filter_map(|res| {
        let res = res.unwrap();
        let s = std::str::from_utf8(&res).unwrap().trim();
        if s.is_empty() {
            None
        } else {
            Some(s.parse().unwrap())
        }
    })
}

pub fn bytes(fh: std::fs::File) -> impl Iterator<Item = u8> {
    fh.bytes().map(|res| res.unwrap())
}

pub fn string(fh: std::fs::File) -> String {
    let bytes: Vec<_> = bytes(fh).collect();
    std::string::String::from_utf8(bytes).unwrap()
}
