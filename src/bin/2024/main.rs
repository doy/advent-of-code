use advent_of_code::prelude::*;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
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
        13 => advent_of_code::day!(2024, opt.day, opt.puzzle, day13),
        14 => advent_of_code::day!(2024, opt.day, opt.puzzle, day14),
        15 => advent_of_code::day!(2024, opt.day, opt.puzzle, day15),
        16 => advent_of_code::day!(2024, opt.day, opt.puzzle, day16),
        17 => advent_of_code::day!(2024, opt.day, opt.puzzle, day17),
        18 => advent_of_code::day!(2024, opt.day, opt.puzzle, day18),
        19 => advent_of_code::day!(2024, opt.day, opt.puzzle, day19),
        20 => advent_of_code::day!(2024, opt.day, opt.puzzle, day20),
        21 => advent_of_code::day!(2024, opt.day, opt.puzzle, day21),
        22 => advent_of_code::day!(2024, opt.day, opt.puzzle, day22),
        23 => advent_of_code::day!(2024, opt.day, opt.puzzle, day23),
        // NEXT PART
        _ => panic!("unknown day {}", opt.day),
    }
    #[allow(unreachable_code)]
    Ok(())
}
