#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

fn print_elves(elves: &HashSet<(IRow, ICol)>) {
    let min_row = elves.iter().map(|elf| elf.0).min().unwrap();
    let max_row = elves.iter().map(|elf| elf.0).max().unwrap();
    let min_col = elves.iter().map(|elf| elf.1).min().unwrap();
    let max_col = elves.iter().map(|elf| elf.1).max().unwrap();
    for row in (min_row.0..=max_row.0).map(IRow) {
        for col in (min_col.0..=max_col.0).map(ICol) {
            if elves.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!()
    }
}

pub fn parse(fh: File) -> Result<HashSet<(IRow, ICol)>> {
    let mut elves = HashSet::new();
    for (row, line) in parse::raw_lines(fh).enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    elves.insert((IRow(row as isize), ICol(col as isize)));
                }
                '.' => {}
                _ => panic!("unexpected char {}", c),
            }
        }
    }
    Ok(elves)
}

fn neighbors(elf: (IRow, ICol)) -> Vec<(IRow, ICol)> {
    vec![
        (elf.0 - 1, elf.1 - 1),
        (elf.0 - 1, elf.1),
        (elf.0 - 1, elf.1 + 1),
        (elf.0, elf.1 - 1),
        (elf.0, elf.1 + 1),
        (elf.0 + 1, elf.1 - 1),
        (elf.0 + 1, elf.1),
        (elf.0 + 1, elf.1 + 1),
    ]
}

pub fn part1(mut elves: HashSet<(IRow, ICol)>) -> Result<isize> {
    let mut possible: VecDeque<(IRow, ICol)> = VecDeque::new();
    possible.push_back((IRow(-1), ICol(0)));
    possible.push_back((IRow(1), ICol(0)));
    possible.push_back((IRow(0), ICol(-1)));
    possible.push_back((IRow(0), ICol(1)));
    for _ in 0..10 {
        let mut moves: HashSet<((IRow, ICol), (IRow, ICol))> = HashSet::new();
        for elf in &elves {
            if neighbors(*elf)
                .iter()
                .all(|neighbor| !elves.contains(neighbor))
            {
                continue;
            }
            for diff in &possible {
                let occupied = if diff.0 == IRow(0) {
                    elves.contains(&(elf.0 - 1, elf.1 + diff.1 .0))
                        || elves.contains(&(elf.0, elf.1 + diff.1 .0))
                        || elves.contains(&(elf.0 + 1, elf.1 + diff.1 .0))
                } else {
                    elves.contains(&(elf.0 + diff.0 .0, elf.1 - 1))
                        || elves.contains(&(elf.0 + diff.0 .0, elf.1))
                        || elves.contains(&(elf.0 + diff.0 .0, elf.1 + 1))
                };
                if !occupied {
                    moves.insert((
                        *elf,
                        (elf.0 + diff.0 .0, elf.1 + diff.1 .0),
                    ));
                    break;
                }
            }
        }
        let mut targets: HashMap<(IRow, ICol), usize> = HashMap::new();
        for (from, to) in &moves {
            *targets.entry(*to).or_default() += 1;
        }
        for (from, to) in &moves {
            if targets.get(to).copied().unwrap_or(0) == 1 {
                elves.remove(from);
                elves.insert(*to);
            }
        }

        let first = possible.pop_front().unwrap();
        possible.push_back(first);
    }
    let min_row = elves.iter().map(|elf| elf.0).min().unwrap();
    let max_row = elves.iter().map(|elf| elf.0).max().unwrap();
    let min_col = elves.iter().map(|elf| elf.1).min().unwrap();
    let max_col = elves.iter().map(|elf| elf.1).max().unwrap();
    let area = (max_row.0 - min_row.0 + 1) * (max_col.0 - min_col.0 + 1);
    Ok(area - elves.len() as isize)
}

pub fn part2(mut elves: HashSet<(IRow, ICol)>) -> Result<usize> {
    let mut possible: VecDeque<(IRow, ICol)> = VecDeque::new();
    possible.push_back((IRow(-1), ICol(0)));
    possible.push_back((IRow(1), ICol(0)));
    possible.push_back((IRow(0), ICol(-1)));
    possible.push_back((IRow(0), ICol(1)));
    let mut round = 0;
    loop {
        round += 1;
        let mut moves: HashSet<((IRow, ICol), (IRow, ICol))> = HashSet::new();
        for elf in &elves {
            if neighbors(*elf)
                .iter()
                .all(|neighbor| !elves.contains(neighbor))
            {
                continue;
            }
            for diff in &possible {
                let occupied = if diff.0 == IRow(0) {
                    elves.contains(&(elf.0 - 1, elf.1 + diff.1 .0))
                        || elves.contains(&(elf.0, elf.1 + diff.1 .0))
                        || elves.contains(&(elf.0 + 1, elf.1 + diff.1 .0))
                } else {
                    elves.contains(&(elf.0 + diff.0 .0, elf.1 - 1))
                        || elves.contains(&(elf.0 + diff.0 .0, elf.1))
                        || elves.contains(&(elf.0 + diff.0 .0, elf.1 + 1))
                };
                if !occupied {
                    moves.insert((
                        *elf,
                        (elf.0 + diff.0 .0, elf.1 + diff.1 .0),
                    ));
                    break;
                }
            }
        }
        let mut targets: HashMap<(IRow, ICol), usize> = HashMap::new();
        for (from, to) in &moves {
            *targets.entry(*to).or_default() += 1;
        }
        let mut moved = false;
        for (from, to) in &moves {
            if targets.get(to).copied().unwrap_or(0) == 1 {
                elves.remove(from);
                elves.insert(*to);
                moved = true;
            }
        }
        if !moved {
            break;
        }

        let first = possible.pop_front().unwrap();
        possible.push_back(first);
    }

    Ok(round)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 23).unwrap()).unwrap()).unwrap(),
        0
    );
    assert_eq!(
        part2(parse(parse::data(2022, 23).unwrap()).unwrap()).unwrap(),
        0
    );
}
