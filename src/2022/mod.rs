use crate::prelude::*;

#[path = "1/mod.rs"]
mod day1;
#[path = "2/mod.rs"]
mod day2;
#[path = "3/mod.rs"]
mod day3;
// NEXT MOD

pub fn run(day: u8, puzzle: u8) -> Result<i64> {
    #[allow(clippy::match_single_binding)]
    match (day, puzzle) {
        (1, 1) => day1::part1(day1::parse(parse::data(2022, 1)?)?),
        (1, 2) => day1::part2(day1::parse(parse::data(2022, 1)?)?),
        (2, 1) => day2::part1(day2::parse(parse::data(2022, 2)?)?),
        (2, 2) => day2::part2(day2::parse(parse::data(2022, 2)?)?),
        (3, 1) => day3::part1(day3::parse(parse::data(2022, 3)?)?),
        (3, 2) => day3::part2(day3::parse(parse::data(2022, 3)?)?),
        // NEXT PART
        _ => Err(anyhow!("unknown puzzle {}-{}", day, puzzle)),
    }
}
