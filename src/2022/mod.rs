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
#[path = "8/mod.rs"]
mod day8;
#[path = "9/mod.rs"]
mod day9;
#[path = "10/mod.rs"]
mod day10;
#[path = "11/mod.rs"]
mod day11;
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
        (8, 1) => day8::part1(day8::parse(parse::data(2022, 8)?)?),
        (8, 2) => day8::part2(day8::parse(parse::data(2022, 8)?)?),
        (9, 1) => day9::part1(day9::parse(parse::data(2022, 9)?)?),
        (9, 2) => day9::part2(day9::parse(parse::data(2022, 9)?)?),
        (10, 1) => day10::part1(day10::parse(parse::data(2022, 10)?)?),
        (10, 2) => day10::part2(day10::parse(parse::data(2022, 10)?)?),
        (11, 1) => day11::part1(day11::parse(parse::data(2022, 11)?)?),
        (11, 2) => day11::part2(day11::parse(parse::data(2022, 11)?)?),
        // NEXT PART
        _ => Err(anyhow!("unknown puzzle {}-{}", day, puzzle)),
    }
}
