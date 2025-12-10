#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

#[derive(Debug)]
pub struct Machine {
    lights: u16,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u64>,
}

impl std::str::FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let cap = regex_captures!(r"\[([.#]*)\] (.*) \{(.*)\}", s).unwrap();
        let lights = cap[1]
            .bytes()
            .rev()
            .map(|b| b == b'#')
            .fold(0, |acc, on| (acc << 1) | (if on { 1 } else { 0 }));
        let buttons = cap[2]
            .split(' ')
            .map(|button| {
                button
                    .strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect();
        let joltages =
            cap[3].split(',').map(|n| n.parse().unwrap()).collect();
        Ok(Self {
            lights,
            buttons,
            joltages,
        })
    }
}

impl Machine {
    fn min_buttons(&self) -> u64 {
        Lights {
            buttons: self.buttons.clone(),
        }
        .dijkstra(0, |lights| lights == self.lights)
        .unwrap()
        .0
    }

    fn min_joltages(&self) -> u64 {
        let mut buttons = nalgebra::DMatrix::from_element(
            self.buttons.len(),
            self.joltages.len(),
            0.0,
        );
        for (b, counters) in self.buttons.iter().enumerate() {
            for counter in counters {
                buttons[(b, *counter)] = 1.0;
            }
        }
        let rank = buttons.rank(0.00001);
        println!("buttons {buttons}");
        println!("buttons rank {}", buttons.rank(0.00001));
        if self.joltages.len() == rank {
            let buttons_inv = buttons.pseudo_inverse(0.00001).unwrap();
            println!("buttons inverse {buttons_inv}");
            let joltages = nalgebra::DMatrix::from_iterator(
                1,
                self.joltages.len(),
                self.joltages.iter().copied().map(|n| n as f64),
            );
            let button_presses = joltages * buttons_inv;
            println!("button presses {button_presses}");
            button_presses.iter().copied().map(|n| n as u64).sum()
        } else if self.joltages.len() > rank {
            99
        } else {
            89
        }
    }
}

struct Lights {
    buttons: Vec<Vec<usize>>,
}

impl advent_of_code::graph::Graph<u16, usize> for Lights {
    fn edges(&self, v: u16) -> impl IntoIterator<Item = usize> {
        0..self.buttons.len()
    }

    fn edge(&self, mut v: u16, e: usize) -> (u16, u64) {
        for light in &self.buttons[e] {
            v ^= 1 << light;
        }
        (v, 1)
    }
}

pub fn parse(fh: File) -> Result<impl Iterator<Item = Machine>> {
    Ok(parse::lines(fh))
}

pub fn part1(machines: impl Iterator<Item = Machine>) -> Result<i64> {
    Ok(machines
        .map(|machine| machine.min_buttons())
        .sum::<u64>()
        .try_into()
        .unwrap())
}

pub fn part2(machines: impl Iterator<Item = Machine>) -> Result<i64> {
    Ok(machines
        .map(|machine| machine.min_joltages())
        .inspect(|n| eprintln!("***{n}***"))
        .sum::<u64>()
        .try_into()
        .unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2025, 10).unwrap()).unwrap()).unwrap(),
        512,
    );
    // TODO
    // assert_eq!(
    //     part2(parse(parse::data(2025, 10).unwrap()).unwrap()).unwrap(),
    //     0
    // );
}
