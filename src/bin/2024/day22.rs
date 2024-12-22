use advent_of_code::prelude::*;

fn mix(x: i64, y: i64) -> i64 {
    x ^ y
}

fn prune(x: i64) -> i64 {
    x % 16777216
}

fn next_secret(mut n: i64) -> i64 {
    n = prune(mix(n * 64, n));
    n = prune(mix(n / 32, n));
    n = prune(mix(n * 2048, n));
    n
}

pub fn parse(fh: File) -> Result<impl Iterator<Item = i64>> {
    Ok(parse::lines(fh))
}

pub fn part1(secrets: impl Iterator<Item = i64>) -> Result<i64> {
    Ok(secrets
        .map(|mut n| {
            for _ in 0..2000 {
                n = next_secret(n);
            }
            n
        })
        .sum())
}

pub fn part2(secrets: impl Iterator<Item = i64>) -> Result<i64> {
    let secrets: Vec<Vec<i64>> = secrets
        .map(|mut secret| {
            std::iter::once(secret)
                .chain(
                    std::iter::from_fn(|| {
                        secret = next_secret(secret);
                        Some(secret)
                    })
                    .take(2000),
                )
                .collect()
        })
        .collect();
    let prices: Vec<Vec<_>> = secrets
        .iter()
        .map(|secrets| secrets.iter().map(|secret| secret % 10).collect())
        .collect();
    let changes: Vec<Vec<_>> = prices
        .iter()
        .map(|prices| {
            prices.windows(2).map(|pair| pair[1] - pair[0]).collect()
        })
        .collect();
    let mut signals = HashMap::new();
    for (changes, prices) in changes.iter().zip(prices.iter()) {
        let mut buyer_signals = HashMap::new();
        for (signal, price) in changes.windows(4).zip(prices.iter().skip(4)) {
            let entry = buyer_signals.entry(signal).or_default();
            if *entry == 0 {
                *entry = *price;
            }
        }
        for (signal, price) in buyer_signals {
            *signals.entry(signal).or_default() += price;
        }
    }
    Ok(signals.values().copied().max().unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 22).unwrap()).unwrap()).unwrap(),
        16299144133
    );
    assert_eq!(
        part2(parse(parse::data(2024, 22).unwrap()).unwrap()).unwrap(),
        1896
    );
}
