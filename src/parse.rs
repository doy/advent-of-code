use crate::prelude::*;

pub fn data(year: u16, day: u16) -> Result<File> {
    File::open(format!("data/{}/{}.txt", year, day)).map_err(|e| anyhow!(e))
}

pub fn lines<R: std::io::Read>(fh: R) -> impl Iterator<Item = String> {
    let fh = std::io::BufReader::new(fh);
    fh.lines().map(|res| res.unwrap())
}

pub struct Chunk<'a, I: Iterator<Item = String>> {
    it: &'a mut I,
}

impl<'a, I: Iterator<Item = String>> Iterator for Chunk<'a, I> {
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

pub fn split<R: std::io::Read>(
    fh: R,
    sep: u8,
) -> impl Iterator<Item = String> {
    let fh = std::io::BufReader::new(fh);
    fh.split(sep)
        .map(|res| String::from_utf8(res.unwrap()).unwrap())
}

pub fn ints(iter: impl Iterator<Item = String>) -> impl Iterator<Item = i64> {
    iter.map(|s| s.trim().parse().unwrap())
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
pub fn grid<F, T>(lines: impl Iterator<Item = String>, f: F) -> Grid<T>
where
    F: Fn(u8) -> T,
    T: Clone + Default + Eq + PartialEq,
{
    lines
        .map(|s| {
            s.as_bytes()
                .iter()
                .copied()
                .map(|b| f(b))
                .collect::<Vec<_>>()
        })
        .collect()
}
