#[derive(Debug, structopt::StructOpt)]
#[structopt(about = "Advent of Code")]
pub struct Opt {
    pub day: u8,
    pub puzzle: u8,
}

#[macro_export]
macro_rules! day {
    ($year:expr, $day:expr, $puzzle:expr, $mod:ident) => {{
        let data = $mod::parse(parse::data($year, $day)?)?;
        match $puzzle {
            1 => println!("{}", $mod::part1(data)?),
            2 => println!("{}", $mod::part2(data)?),
            _ => {
                panic!("unknown puzzle {} for day {}", $puzzle, $day)
            }
        }
    }};
}
