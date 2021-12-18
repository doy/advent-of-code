#![allow(unused_macros)]
#![allow(dead_code)]

#[macro_use]
pub mod parse;

pub mod grid;

pub fn data(year: u16, day: u16) -> anyhow::Result<std::fs::File> {
    std::fs::File::open(format!("data/{}/{}.txt", year, day))
        .map_err(|e| anyhow::anyhow!(e))
}
