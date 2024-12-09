use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Vec<u32>> {
    let mut on = true;
    let mut id = 0;
    let mut disk = vec![];
    for c in parse::bytes(fh) {
        if c == b'\n' {
            break;
        }
        let len = c - b'0';
        if on {
            disk.extend(std::iter::repeat(id).take(len.into()));
            id += 1;
        } else {
            disk.extend(std::iter::repeat(u32::MAX).take(len.into()));
        }
        on = !on;
    }
    Ok(disk)
}

pub fn part1(mut disk: Vec<u32>) -> Result<i64> {
    let mut start = 0;
    let mut end = disk.len() - 1;
    loop {
        let new_start = start
            + disk[start..=end]
                .iter()
                .position(|c| *c == u32::MAX)
                .unwrap();
        let new_end = start
            + disk[start..=end]
                .iter()
                .rposition(|c| *c != u32::MAX)
                .unwrap();
        if new_start > new_end {
            break;
        }
        start = new_start;
        end = new_end;
        disk[start] = disk[end];
        disk[end] = u32::MAX;
    }
    let mut total = 0;
    for (i, id) in disk
        .into_iter()
        .take_while(|id| *id != u32::MAX)
        .enumerate()
    {
        total += i64::try_from(i).unwrap() * i64::from(id);
    }
    Ok(total)
}

pub fn part2(mut disk: Vec<u32>) -> Result<i64> {
    let mut id = disk[disk.len() - 1];
    loop {
        let file_pos = disk.iter().position(|c| *c == id).unwrap();
        let file_len = disk[file_pos..]
            .iter()
            .position(|c| *c != id)
            .unwrap_or(disk[file_pos..].len());
        let mut hole_pos = usize::MAX;
        let mut hole_len = 0;
        for (i, id) in disk[..file_pos].iter().copied().enumerate() {
            if id == u32::MAX {
                if hole_pos == usize::MAX {
                    hole_pos = i;
                    hole_len = 1;
                } else {
                    hole_len += 1;
                }
            } else {
                hole_pos = usize::MAX;
            }
            if hole_len == file_len {
                break;
            }
        }
        if hole_len == file_len {
            for i in 0..hole_len {
                disk[hole_pos + i] = disk[file_pos + i];
                disk[file_pos + i] = u32::MAX;
            }
        }
        if id == 0 {
            break;
        }
        id -= 1;
    }
    let mut total = 0;
    for (i, id) in disk.into_iter().enumerate() {
        if id != u32::MAX {
            total += i64::try_from(i).unwrap() * i64::from(id);
        }
    }
    Ok(total)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 9).unwrap()).unwrap()).unwrap(),
        6398608069280
    );
    assert_eq!(
        part2(parse(parse::data(2024, 9).unwrap()).unwrap()).unwrap(),
        6427437134372
    );
}
