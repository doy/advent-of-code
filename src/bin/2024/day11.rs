use advent_of_code::prelude::*;

fn even_digits(i: i64) -> bool {
    i.ilog10() % 2 == 1
}

fn first_half(i: i64) -> i64 {
    let digits = i.ilog10() + 1;
    i / 10i64.pow(digits / 2)
}

fn last_half(i: i64) -> i64 {
    let digits = i.ilog10() + 1;
    i % 10i64.pow(digits / 2)
}

fn blink(
    stones: &[i64],
    depth: usize,
    cache: &mut HashMap<(i64, usize), i64>,
) -> i64 {
    if depth == 0 {
        return stones.len().try_into().unwrap();
    }
    let mut total = 0;
    for stone in stones.iter().copied() {
        if let Some(count) = cache.get(&(stone, depth)) {
            total += count;
        } else {
            let count = if stone == 0 {
                blink(&[1], depth - 1, cache)
            } else if even_digits(stone) {
                blink(
                    &[first_half(stone), last_half(stone)],
                    depth - 1,
                    cache,
                )
            } else {
                blink(&[stone * 2024], depth - 1, cache)
            };
            cache.insert((stone, depth), count);
            total += count;
        }
    }
    total
}

pub fn parse(fh: File) -> Result<Vec<i64>> {
    Ok(parse::split(fh, b' ').collect())
}

pub fn part1(stones: Vec<i64>) -> Result<i64> {
    Ok(blink(&stones, 25, &mut HashMap::new()))
}

pub fn part2(stones: Vec<i64>) -> Result<i64> {
    Ok(blink(&stones, 75, &mut HashMap::new()))
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 11).unwrap()).unwrap()).unwrap(),
        204022
    );
    assert_eq!(
        part2(parse(parse::data(2024, 11).unwrap()).unwrap()).unwrap(),
        241651071960597
    );
}
