use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Grid<u8>> {
    Ok(parse::grid(parse::raw_lines(fh), |c, _| c))
}

pub fn part1(schematic: Grid<u8>) -> Result<i64> {
    let mut numbers = vec![];
    for row in schematic.each_row() {
        let mut current_number = None;
        for col in schematic.each_col() {
            let c = schematic[row][col];
            if c.is_ascii_digit() {
                match current_number {
                    Some(n) => {
                        current_number = Some(n * 10 + i64::from(c - b'0'))
                    }
                    None => current_number = Some(i64::from(c - b'0')),
                }
            } else if let Some(n) = current_number.take() {
                numbers.push((n, row, col));
            }
        }
        if let Some(n) = current_number.take() {
            numbers.push((n, row, schematic.cols()));
        }
    }

    let mut total = 0;
    'number: for (n, row, col) in numbers {
        for offset in 0..=n.ilog10() {
            let col = Col(col.0 - usize::try_from(offset).unwrap() - 1);
            for pos in schematic.adjacent(Pos(row, col), true) {
                let c = schematic[pos];
                if !c.is_ascii_digit() && c != b'.' {
                    total += n;
                    continue 'number;
                }
            }
        }
    }

    Ok(total)
}

pub fn part2(schematic: Grid<u8>) -> Result<i64> {
    let mut numbers = vec![];
    for row in schematic.each_row() {
        let mut current_number = None;
        for col in schematic.each_col() {
            let c = schematic[row][col];
            if c.is_ascii_digit() {
                match current_number {
                    Some(n) => {
                        current_number = Some(n * 10 + i64::from(c - b'0'))
                    }
                    None => current_number = Some(i64::from(c - b'0')),
                }
            } else if let Some(n) = current_number.take() {
                numbers.push((n, row, col));
            }
        }
        if let Some(n) = current_number.take() {
            numbers.push((n, row, schematic.cols()));
        }
    }

    let mut gears: HashMap<_, HashSet<_>> = HashMap::new();
    for (n, nrow, ncol) in numbers {
        for offset in 0..=n.ilog10() {
            for pos in schematic.adjacent(
                Pos(nrow, Col(ncol.0 - usize::try_from(offset).unwrap() - 1)),
                true,
            ) {
                let c = schematic[pos];
                if c == b'*' {
                    gears.entry(pos).or_default().insert((n, nrow, ncol));
                }
            }
        }
    }

    Ok(gears
        .values()
        .filter_map(|s| {
            if s.len() == 2 {
                Some(s.iter().map(|(n, _, _)| n).product::<i64>())
            } else {
                None
            }
        })
        .sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 3).unwrap()).unwrap()).unwrap(),
        540212
    );
    assert_eq!(
        part2(parse(parse::data(2023, 3).unwrap()).unwrap()).unwrap(),
        87605697
    );
}
