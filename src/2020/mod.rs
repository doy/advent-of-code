#[path = "1/mod.rs"]
mod day1;

pub fn run(day: u8, puzzle: u8) -> anyhow::Result<()> {
    match (day, puzzle) {
        (1, 1) => day1::part1(),
        (1, 2) => day1::part2(),
        _ => Err(anyhow::anyhow!("unknown puzzle {}-{}", day, puzzle)),
    }
}
