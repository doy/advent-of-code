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
#[path = "12/mod.rs"]
mod day12;
#[path = "13/mod.rs"]
mod day13;
#[path = "14/mod.rs"]
mod day14;
#[path = "15/mod.rs"]
mod day15;
#[path = "16/mod.rs"]
mod day16;
#[path = "17/mod.rs"]
mod day17;
#[path = "18/mod.rs"]
mod day18;
// NEXT MOD

pub fn run(day: u8, puzzle: u8) -> anyhow::Result<i64> {
    match (day, puzzle) {
        (1, 1) => day1::part1(day1::parse(crate::util::data(2021, 1)?)?),
        (1, 2) => day1::part2(day1::parse(crate::util::data(2021, 1)?)?),
        (2, 1) => day2::part1(day2::parse(crate::util::data(2021, 2)?)?),
        (2, 2) => day2::part2(day2::parse(crate::util::data(2021, 2)?)?),
        (3, 1) => day3::part1(day3::parse(crate::util::data(2021, 3)?)?),
        (3, 2) => day3::part2(day3::parse(crate::util::data(2021, 3)?)?),
        (4, 1) => day4::part1(day4::parse(crate::util::data(2021, 4)?)?),
        (4, 2) => day4::part2(day4::parse(crate::util::data(2021, 4)?)?),
        (5, 1) => day5::part1(day5::parse(crate::util::data(2021, 5)?)?),
        (5, 2) => day5::part2(day5::parse(crate::util::data(2021, 5)?)?),
        (6, 1) => day6::part1(day6::parse(crate::util::data(2021, 6)?)?),
        (6, 2) => day6::part2(day6::parse(crate::util::data(2021, 6)?)?),
        (7, 1) => day7::part1(day7::parse(crate::util::data(2021, 7)?)?),
        (7, 2) => day7::part2(day7::parse(crate::util::data(2021, 7)?)?),
        (8, 1) => day8::part1(day8::parse(crate::util::data(2021, 8)?)?),
        (8, 2) => day8::part2(day8::parse(crate::util::data(2021, 8)?)?),
        (9, 1) => day9::part1(day9::parse(crate::util::data(2021, 9)?)?),
        (9, 2) => day9::part2(day9::parse(crate::util::data(2021, 9)?)?),
        (10, 1) => day10::part1(day10::parse(crate::util::data(2021, 10)?)?),
        (10, 2) => day10::part2(day10::parse(crate::util::data(2021, 10)?)?),
        (11, 1) => day11::part1(day11::parse(crate::util::data(2021, 11)?)?),
        (11, 2) => day11::part2(day11::parse(crate::util::data(2021, 11)?)?),
        (12, 1) => day12::part1(day12::parse(crate::util::data(2021, 12)?)?),
        (12, 2) => day12::part2(day12::parse(crate::util::data(2021, 12)?)?),
        (13, 1) => day13::part1(day13::parse(crate::util::data(2021, 13)?)?),
        (13, 2) => day13::part2(day13::parse(crate::util::data(2021, 13)?)?),
        (14, 1) => day14::part1(day14::parse(crate::util::data(2021, 14)?)?),
        (14, 2) => day14::part2(day14::parse(crate::util::data(2021, 14)?)?),
        (15, 1) => day15::part1(day15::parse(crate::util::data(2021, 15)?)?),
        (15, 2) => day15::part2(day15::parse(crate::util::data(2021, 15)?)?),
        (16, 1) => day16::part1(day16::parse(crate::util::data(2021, 16)?)?),
        (16, 2) => day16::part2(day16::parse(crate::util::data(2021, 16)?)?),
        (17, 1) => day17::part1(day17::parse(crate::util::data(2021, 17)?)?),
        (17, 2) => day17::part2(day17::parse(crate::util::data(2021, 17)?)?),
        (18, 1) => day18::part1(day18::parse(crate::util::data(2021, 18)?)?),
        (18, 2) => day18::part2(day18::parse(crate::util::data(2021, 18)?)?),
        // NEXT PART
        _ => Err(anyhow::anyhow!("unknown puzzle {}-{}", day, puzzle)),
    }
}
