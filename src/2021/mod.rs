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
// NEXT MOD

pub fn run(day: u8, puzzle: u8) -> anyhow::Result<i64> {
    match (day, puzzle) {
        (1, 1) => day1::part1(),
        (1, 2) => day1::part2(),
        (2, 1) => day2::part1(),
        (2, 2) => day2::part2(),
        (3, 1) => day3::part1(),
        (3, 2) => day3::part2(),
        (4, 1) => day4::part1(),
        (4, 2) => day4::part2(),
        (5, 1) => day5::part1(),
        (5, 2) => day5::part2(),
        (6, 1) => day6::part1(),
        (6, 2) => day6::part2(),
        (7, 1) => day7::part1(),
        (7, 2) => day7::part2(),
        (8, 1) => day8::part1(),
        (8, 2) => day8::part2(),
        (9, 1) => day9::part1(),
        (9, 2) => day9::part2(),
        (10, 1) => day10::part1(),
        (10, 2) => day10::part2(),
        (11, 1) => day11::part1(),
        (11, 2) => day11::part2(),
        (12, 1) => day12::part1(),
        (12, 2) => day12::part2(),
        (13, 1) => day13::part1(),
        (13, 2) => day13::part2(),
        (14, 1) => day14::part1(),
        (14, 2) => day14::part2(),
        (15, 1) => day15::part1(),
        (15, 2) => day15::part2(),
        (16, 1) => day16::part1(),
        (16, 2) => day16::part2(),
        // NEXT PART
        _ => Err(anyhow::anyhow!("unknown puzzle {}-{}", day, puzzle)),
    }
}
