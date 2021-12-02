#[path = "1/mod.rs"]
mod day1;
#[path = "2/mod.rs"]
mod day2;
// NEXT MOD

pub fn run(day: u8, puzzle: u8) -> anyhow::Result<i64> {
    match (day, puzzle) {
        (1, 1) => day1::part1(),
        (1, 2) => day1::part2(),
        (2, 1) => day2::part1(),
        (2, 2) => day2::part2(),
        // NEXT PART
        _ => Err(anyhow::anyhow!("unknown puzzle {}-{}", day, puzzle)),
    }
}
