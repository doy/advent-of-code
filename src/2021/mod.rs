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
        // NEXT PART
        _ => Err(anyhow::anyhow!("unknown puzzle {}-{}", day, puzzle)),
    }
}
