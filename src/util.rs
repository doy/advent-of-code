#![allow(unused_macros)]
#![allow(dead_code)]

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

macro_rules! data_bool_map {
    ($t:expr, $f:expr) => {{
        crate::util::bool_map(data_lines!().unwrap(), $t, $f)
    }};
}

macro_rules! data_digit_grid {
    () => {{
        crate::util::digit_map(data_lines!().unwrap())
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

pub fn bool_map(
    lines: impl Iterator<Item = String>,
    t: u8,
    f: u8,
) -> Vec<Vec<bool>> {
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
                .collect()
        })
        .collect()
}

pub fn digit_map(lines: impl Iterator<Item = String>) -> Vec<Vec<u8>> {
    lines
        .map(|s| s.as_bytes().iter().copied().map(|b| b - b'0').collect())
        .collect()
}

pub struct Adjacent {
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
    diagonal: bool,
    pos: u8,
}

impl Iterator for Adjacent {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.pos >= 9 {
                return None;
            }
            let pos_x = self.pos / 3;
            let pos_y = self.pos - pos_x * 3;
            self.pos += 1;
            if pos_x == 0 && self.x == 0
                || pos_y == 0 && self.y == 0
                || pos_x == 2 && self.x == self.max_x
                || pos_y == 2 && self.y == self.max_y
                || pos_x == 1 && pos_y == 1
                || (!self.diagonal
                    && ((pos_x == pos_y)
                        || (pos_x == 2 && pos_y == 0)
                        || (pos_x == 0 && pos_y == 2)))
            {
                continue;
            }
            return Some((
                self.x + usize::from(pos_x) - 1,
                self.y + usize::from(pos_y) - 1,
            ));
        }
    }
}

pub fn adjacent(
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
    diagonal: bool,
) -> Adjacent {
    Adjacent {
        x,
        y,
        max_x,
        max_y,
        diagonal,
        pos: 0,
    }
}
