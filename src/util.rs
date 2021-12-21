pub mod grid;
pub mod parse;

pub fn data(year: u16, day: u16) -> anyhow::Result<std::fs::File> {
    std::fs::File::open(format!("data/{}/{}.txt", year, day))
        .map_err(|e| anyhow::anyhow!(e))
}
