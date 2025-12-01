use advent_of_code::prelude::*;

fn even_digits(i: i64) -> bool {
    advent_of_code::num::digits(i).is_multiple_of(2)
}

fn first_half(i: i64) -> i64 {
    i / 10i64.pow(advent_of_code::num::digits(i) / 2)
}

fn last_half(i: i64) -> i64 {
    i % 10i64.pow(advent_of_code::num::digits(i) / 2)
}

fn blink(
    stone: i64,
    depth: usize,
    cache: &mut HashMap<(i64, usize), i64>,
) -> i64 {
    if depth == 0 {
        1
    } else if let Some(count) = cache.get(&(stone, depth)) {
        *count
    } else {
        let count = if stone == 0 {
            blink(1, depth - 1, cache)
        } else if even_digits(stone) {
            blink(first_half(stone), depth - 1, cache)
                + blink(last_half(stone), depth - 1, cache)
        } else {
            blink(stone * 2024, depth - 1, cache)
        };
        cache.insert((stone, depth), count);
        count
    }
}

pub fn parse(fh: File) -> Result<Vec<i64>> {
    Ok(parse::split(fh, b' ').collect())
}

pub fn part1(stones: Vec<i64>) -> Result<i64> {
    let mut cache = HashMap::new();
    Ok(stones
        .into_iter()
        .map(|stone| blink(stone, 25, &mut cache))
        .sum())
}

pub fn part2(stones: Vec<i64>) -> Result<i64> {
    let mut cache = HashMap::new();
    Ok(stones
        .into_iter()
        .map(|stone| blink(stone, 75, &mut cache))
        .sum())
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
