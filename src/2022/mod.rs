use crate::prelude::*;

#[path = "1/mod.rs"]
mod day1;
// NEXT MOD

pub fn run(day: u8, puzzle: u8) -> Result<i64> {
    #[allow(clippy::match_single_binding)]
    match (day, puzzle) {
        (1, 1) => day1::part1(day1::parse(parse::data(2022, 1)?)?),
        (1, 2) => day1::part2(day1::parse(parse::data(2022, 1)?)?),
        // NEXT PART
        _ => Err(anyhow!("unknown puzzle {}-{}", day, puzzle)),
    }
}
