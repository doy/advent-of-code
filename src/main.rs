#![allow(clippy::collapsible_if)]

#[macro_use]
mod util;

#[path = "2020/mod.rs"]
mod year2020;
#[path = "2021/mod.rs"]
mod year2021;

#[derive(Debug, structopt::StructOpt)]
#[structopt(about = "Advent of Code")]
enum Opt {
    #[structopt(name = "2020")]
    Year2020 { day: u8, puzzle: u8 },
    #[structopt(name = "2021")]
    Year2021 { day: u8, puzzle: u8 },
}

#[paw::main]
fn main(opt: Opt) {
    let res = match opt {
        Opt::Year2020 { day, puzzle } => crate::year2020::run(day, puzzle),
        Opt::Year2021 { day, puzzle } => crate::year2021::run(day, puzzle),
    };
    match res {
        Ok(answer) => {
            println!("{}", answer);
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
