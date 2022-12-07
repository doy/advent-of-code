use crate::prelude::*;

#[path = "1/mod.rs"]
mod day1;
#[path = "2/mod.rs"]
mod day2;
#[path = "3/mod.rs"]
mod day3;
#[path = "4/mod.rs"]
mod day4;
#[path = "5/mod.rs"]
mod day5;
#[path = "6/mod.rs"]
mod day6;
#[path = "7/mod.rs"]
mod day7;
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
        (4, 1) => day4::part1(day4::parse(parse::data(2022, 4)?)?),
        (4, 2) => day4::part2(day4::parse(parse::data(2022, 4)?)?),
        (5, 1) => day5::part1(day5::parse(parse::data(2022, 5)?)?),
        (5, 2) => day5::part2(day5::parse(parse::data(2022, 5)?)?),
        (6, 1) => day6::part1(day6::parse(parse::data(2022, 6)?)?),
        (6, 2) => day6::part2(day6::parse(parse::data(2022, 6)?)?),
        (7, 1) => day7::part1(day7::parse(parse::data(2022, 7)?)?),
        (7, 2) => day7::part2(day7::parse(parse::data(2022, 7)?)?),
        // NEXT PART
        _ => Err(anyhow!("unknown puzzle {}-{}", day, puzzle)),
    }
}
