use advent_of_code::prelude::*;

const EPS: f64 = 0.000001;

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

fn find_basis(
    buttons: &nalgebra::DMatrix<f64>,
) -> (
    nalgebra::DMatrix<f64>,
    Vec<usize>,
    nalgebra::DMatrix<f64>,
    Vec<usize>,
) {
    let rank = buttons.rank(EPS);
    let mut basis = buttons.clone();
    let mut removed_rows = vec![];
    let mut extra_cols = vec![];

    for row in (0..buttons.nrows()).rev() {
        if basis.clone().remove_row(row).rank(EPS) == rank {
            basis = basis.remove_row(row);
            removed_rows.push(row);
        }
    }

    let mut extra_col = basis.clone();
    for col in (0..basis.ncols()).rev() {
        if basis.clone().remove_column(col).rank(EPS) == rank {
            basis = basis.remove_column(col);
            extra_cols.insert(0, col);
        } else {
            extra_col = extra_col.remove_column(col);
        }
    }

    (basis, removed_rows, extra_col, extra_cols)
}

fn all_possibilities(
    dim: usize,
    max: u64,
) -> Box<dyn Iterator<Item = Vec<u64>>> {
    let mut iter = Box::new(std::iter::once(vec![]))
        as Box<dyn Iterator<Item = Vec<u64>>>;
    for _ in 0..dim {
        iter = Box::new(iter.flat_map(move |v| {
            (0..=max).map(move |n| {
                let mut v = v.clone();
                v.push(n);
                v
            })
        }))
    }
    iter
}

fn check_solution(
    buttons: &nalgebra::DMatrix<f64>,
    joltages: &nalgebra::DMatrix<f64>,
    missing: &[u64],
    extra_cols: &[usize],
    solution: &nalgebra::DMatrix<f64>,
) -> bool {
    let mut solution = solution.clone();
    for (i, col) in extra_cols.iter().enumerate() {
        solution = solution.insert_row(*col, missing[i] as f64);
    }
    buttons * &solution == *joltages
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
            self.joltages.len(),
            self.buttons.len(),
            0.0,
        );
        for (b, counters) in self.buttons.iter().enumerate() {
            for counter in counters {
                buttons[(*counter, b)] = 1.0;
            }
        }

        let (basis, removed_rows, extra, extra_cols) = find_basis(&buttons);
        let lu = basis.clone().lu();

        let full_joltages = nalgebra::DMatrix::from_iterator(
            self.joltages.len(),
            1,
            self.joltages.iter().map(|n| *n as f64),
        );
        let mut joltages = full_joltages.clone();
        for row in &removed_rows {
            joltages = joltages.remove_row(*row);
        }

        if extra.is_empty() {
            lu.solve(&joltages)
                .unwrap()
                .iter()
                .copied()
                .sum::<f64>()
                .round() as u64
        } else {
            all_possibilities(extra.ncols(), joltages.max().round() as u64)
                .filter_map(|missing| {
                    let joltages = &joltages
                        - (&extra
                            * nalgebra::DMatrix::from_iterator(
                                missing.len(),
                                1,
                                missing.iter().copied().map(|n| n as f64),
                            ));
                    if joltages.iter().any(|f| f.round() < 0.0) {
                        return None;
                    }
                    let solution = lu.solve(&joltages).unwrap();
                    if solution.iter().any(|f| {
                        f.round() < 0.0 || (f - f.round()).abs() > EPS
                    }) {
                        None
                    } else {
                        let solution = nalgebra::DMatrix::from_iterator(
                            solution.nrows(),
                            solution.ncols(),
                            solution.iter().map(|f| f.round()),
                        );
                        assert!(check_solution(
                            &buttons,
                            &full_joltages,
                            &missing,
                            &extra_cols,
                            &solution,
                        ));
                        Some(
                            solution.iter().copied().sum::<f64>().round()
                                as u64
                                + missing.iter().sum::<u64>(),
                        )
                    }
                })
                .min()
                .unwrap()
        }
    }
}

struct Lights {
    buttons: Vec<Vec<usize>>,
}

impl advent_of_code::graph::Graph<u16, usize> for Lights {
    fn edges(&self, _: u16) -> impl IntoIterator<Item = usize> {
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
    assert_eq!(
        part2(parse(parse::data(2025, 10).unwrap()).unwrap()).unwrap(),
        19857,
    );
}
