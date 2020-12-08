#![allow(clippy::collapsible_if)]

mod util;
#[path = "2020/mod.rs"]
mod year2020;

#[derive(Debug, structopt::StructOpt)]
#[structopt(about = "Advent of Code")]
enum Opt {
    #[structopt(name = "2020")]
    Year2020 { day: u8, puzzle: u8 },
}

#[paw::main]
fn main(opt: Opt) {
    let res = match opt {
        Opt::Year2020 { day, puzzle } => crate::year2020::run(day, puzzle),
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
