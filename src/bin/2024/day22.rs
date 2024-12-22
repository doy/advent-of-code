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

pub fn parse(fh: File) -> Result<Vec<i64>> {
    Ok(parse::lines(fh).collect())
}

pub fn part1(secrets: Vec<i64>) -> Result<i64> {
    Ok(secrets
        .into_par_iter()
        .map(|mut n| {
            for _ in 0..2000 {
                n = next_secret(n);
            }
            n
        })
        .sum())
}

pub fn part2(secrets: Vec<i64>) -> Result<i64> {
    let prices: Vec<Vec<_>> = secrets
        .into_par_iter()
        .map(|mut secret| {
            std::iter::once(secret)
                .chain(
                    std::iter::from_fn(|| {
                        secret = next_secret(secret);
                        Some(secret)
                    })
                    .take(2000),
                )
                .map(|secret| secret % 10)
                .collect()
        })
        .collect();
    let changes: Vec<Vec<_>> = prices
        .par_iter()
        .map(|prices| {
            prices.windows(2).map(|pair| pair[1] - pair[0]).collect()
        })
        .collect();
    let mut price_map = [0; 19usize.pow(4)];
    for (changes, prices) in changes.iter().zip(prices.iter()) {
        let mut buyer_price_map = [0; 19usize.pow(4)];
        for (signal, price) in
            changes.windows(4).zip(prices.iter().copied().skip(4)).rev()
        {
            let key = usize::try_from(
                (signal[0] + 9)
                    + (signal[1] + 9) * 19
                    + (signal[2] + 9) * 19 * 19
                    + (signal[3] + 9) * 19 * 19 * 19,
            )
            .unwrap();
            buyer_price_map[key] = price;
        }
        for (total, buyer) in
            price_map.iter_mut().zip(buyer_price_map.iter_mut())
        {
            *total += *buyer;
        }
    }
    Ok(price_map.iter().copied().max().unwrap())
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
