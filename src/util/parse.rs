use std::io::{BufRead as _, Read as _};

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
