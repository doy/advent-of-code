use advent_of_code::prelude::*;

mod day1;
mod day2;
// NEXT MOD

#[paw::main]
fn main(opt: Opt) -> Result<()> {
    #[allow(clippy::match_single_binding)]
    match opt.day {
        1 => advent_of_code::day!(2024, opt.day, opt.puzzle, day1),
        2 => advent_of_code::day!(2024, opt.day, opt.puzzle, day2),
        // NEXT PART
        _ => panic!("unknown day {}", opt.day),
    }
    #[allow(unreachable_code)]
    Ok(())
}
