use crate::prelude::*;

pub fn data(year: u16, day: u8) -> Result<File> {
    File::open(format!("data/{}/{}.txt", year, day)).map_err(|e| anyhow!(e))
}

pub fn raw_lines<R>(fh: R) -> impl Iterator<Item = String>
where
    R: std::io::Read,
{
    let fh = std::io::BufReader::new(fh);
    fh.lines().map(|res| res.unwrap())
}

pub fn lines<R, T>(fh: R) -> impl Iterator<Item = T>
where
    R: std::io::Read,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    raw_lines(fh).map(|s| s.trim().parse().unwrap())
}

pub fn fields2<T1, T2>(line: impl AsRef<str>) -> (T1, T2)
where
    T1: std::str::FromStr,
    T2: std::str::FromStr,
    <T1 as std::str::FromStr>::Err: std::fmt::Debug,
    <T2 as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut parts = line.as_ref().split_whitespace();
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

pub fn fields3<T1, T2, T3>(line: impl AsRef<str>) -> (T1, T2, T3)
where
    T1: std::str::FromStr,
    T2: std::str::FromStr,
    T3: std::str::FromStr,
    <T1 as std::str::FromStr>::Err: std::fmt::Debug,
    <T2 as std::str::FromStr>::Err: std::fmt::Debug,
    <T3 as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut parts = line.as_ref().split_whitespace();
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

pub struct Chunk<'a, I: Iterator<Item = String>> {
    it: &'a mut I,
}

impl<I: Iterator<Item = String>> Iterator for Chunk<'_, I> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(line) = self.it.next() {
            if line.is_empty() {
                return None;
            } else {
                return Some(line);
            }
        }
        None
    }
}

pub fn chunk<I>(it: &mut I) -> Chunk<'_, I>
where
    I: Iterator<Item = String>,
{
    Chunk { it }
}

pub fn split<R, T>(fh: R, sep: u8) -> impl Iterator<Item = T>
where
    R: std::io::Read,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let fh = std::io::BufReader::new(fh);
    fh.split(sep)
        .map(|res| String::from_utf8(res.unwrap()).unwrap())
        .map(|s| s.trim().parse().unwrap())
}

pub fn bytes<R: std::io::Read>(fh: R) -> impl Iterator<Item = u8> {
    fh.bytes().map(|res| res.unwrap())
}

pub fn string<R: std::io::Read>(fh: R) -> String {
    let bytes: Vec<_> = bytes(fh).collect();
    String::from_utf8(bytes).unwrap()
}

pub fn bool_grid(
    lines: impl Iterator<Item = String>,
    t: u8,
    f: u8,
) -> Grid<bool> {
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

pub fn digit_grid(lines: impl Iterator<Item = String>) -> Grid<u8> {
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

// false positive, doing its suggestion gives borrow checker errors
#[allow(clippy::redundant_closure)]
pub fn grid<F, T>(lines: impl Iterator<Item = String>, mut f: F) -> Grid<T>
where
    F: FnMut(u8, Pos) -> T,
    T: Default + Clone + Eq + PartialEq + std::hash::Hash,
{
    lines
        .enumerate()
        .map(|(row, s)| {
            s.as_bytes()
                .iter()
                .copied()
                .enumerate()
                .map(|(col, b)| f(b, Pos(Row(row), Col(col))))
                .collect::<Vec<_>>()
        })
        .collect()
}
