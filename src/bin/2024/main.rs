use advent_of_code::prelude::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
// NEXT MOD

#[paw::main]
fn main(opt: Opt) -> Result<()> {
    #[allow(clippy::match_single_binding)]
    match opt.day {
        1 => advent_of_code::day!(2024, opt.day, opt.puzzle, day1),
        2 => advent_of_code::day!(2024, opt.day, opt.puzzle, day2),
        3 => advent_of_code::day!(2024, opt.day, opt.puzzle, day3),
        4 => advent_of_code::day!(2024, opt.day, opt.puzzle, day4),
        5 => advent_of_code::day!(2024, opt.day, opt.puzzle, day5),
        6 => advent_of_code::day!(2024, opt.day, opt.puzzle, day6),
        7 => advent_of_code::day!(2024, opt.day, opt.puzzle, day7),
        8 => advent_of_code::day!(2024, opt.day, opt.puzzle, day8),
        9 => advent_of_code::day!(2024, opt.day, opt.puzzle, day9),
        10 => advent_of_code::day!(2024, opt.day, opt.puzzle, day10),
        11 => advent_of_code::day!(2024, opt.day, opt.puzzle, day11),
        12 => advent_of_code::day!(2024, opt.day, opt.puzzle, day12),
        // NEXT PART
        _ => panic!("unknown day {}", opt.day),
    }
    #[allow(unreachable_code)]
    Ok(())
}
