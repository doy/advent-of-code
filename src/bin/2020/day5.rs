use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<impl Iterator<Item = u64>> {
    Ok(parse::raw_lines(fh).map(|line| seat_id(&line).unwrap()))
}

pub fn part1(ids: impl Iterator<Item = u64>) -> Result<u64> {
    let mut max = 0;
    for id in ids {
        if id > max {
            max = id;
        }
    }
    Ok(max)
}

pub fn part2(ids: impl Iterator<Item = u64>) -> Result<u64> {
    let mut seats = vec![false; 1024];
    for id in ids {
        seats[id as usize] = true;
    }
    let first = seats
        .iter()
        .position(|b| *b)
        .context("failed to find taken seat")?;
    let seat = seats
        .iter()
        .skip(first)
        .position(|b| !*b)
        .context("failed to find free seat")?
        + first;
    if !seats[seat - 1] || seats[seat] || !seats[seat + 1] {
        return Err(anyhow!("invalid seat found"));
    }
    Ok(seat.try_into()?)
}

fn seat_id(desc: &str) -> Result<u64> {
    if desc.len() != 10 {
        return Err(anyhow!("invalid desc {}", desc));
    }
    let row_desc = &desc[0..7];
    let col_desc = &desc[7..10];

    let mut min_row = 0;
    let mut max_row = 127;
    for c in row_desc.chars() {
        let mid = (max_row + min_row) / 2;
        match c {
            'F' => {
                max_row = mid;
            }
            'B' => {
                min_row = mid + 1;
            }
            _ => return Err(anyhow!("invalid desc {}", desc)),
        }
    }
    if min_row != max_row {
        return Err(anyhow!("bug"));
    }
    let row = min_row;

    let mut min_col = 0;
    let mut max_col = 7;
    for c in col_desc.chars() {
        let mid = (max_col + min_col) / 2;
        match c {
            'L' => {
                max_col = mid;
            }
            'R' => {
                min_col = mid + 1;
            }
            _ => return Err(anyhow!("invalid desc {}", desc)),
        }
    }
    if min_col != max_col {
        return Err(anyhow!("bug"));
    }
    let col = min_col;

    Ok(row * 8 + col)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2020, 5).unwrap()).unwrap()).unwrap(),
        978
    );
    assert_eq!(
        part2(parse(parse::data(2020, 5).unwrap()).unwrap()).unwrap(),
        727
    );
}
