use anyhow::Context as _;
use std::convert::TryInto as _;

pub fn part1() -> anyhow::Result<i64> {
    let input = data_str!()?;
    let mut max = 0;
    for line in input.lines() {
        let id = seat_id(line)?;
        if id > max {
            max = id;
        }
    }
    Ok(max)
}

pub fn part2() -> anyhow::Result<i64> {
    let mut seats = vec![false; 1024];
    let input = data_str!()?;
    for line in input.lines() {
        let id = seat_id(line)?;
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
        return Err(anyhow::anyhow!("invalid seat found"));
    }
    Ok(seat.try_into()?)
}

fn seat_id(desc: &str) -> anyhow::Result<i64> {
    if desc.len() != 10 {
        return Err(anyhow::anyhow!("invalid desc {}", desc));
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
            _ => return Err(anyhow::anyhow!("invalid desc {}", desc)),
        }
    }
    if min_row != max_row {
        return Err(anyhow::anyhow!("bug"));
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
            _ => return Err(anyhow::anyhow!("invalid desc {}", desc)),
        }
    }
    if min_col != max_col {
        return Err(anyhow::anyhow!("bug"));
    }
    let col = min_col;

    Ok(row * 8 + col)
}

#[test]
fn test() {
    assert_eq!(part1().unwrap(), 978);
    assert_eq!(part2().unwrap(), 727);
}
