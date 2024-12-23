use advent_of_code::prelude::*;

fn mix(x: u32, y: u32) -> u32 {
    x ^ y
}

fn prune(x: u32) -> u32 {
    x % 16777216
}

fn next_secret(mut n: u32) -> u32 {
    n = prune(mix(n * 64, n));
    n = prune(mix(n / 32, n));
    n = prune(mix(n * 2048, n));
    n
}

pub fn parse(fh: File) -> Result<Vec<u32>> {
    Ok(parse::lines(fh).collect())
}

pub fn part1(secrets: Vec<u32>) -> Result<i64> {
    Ok(secrets
        .into_par_iter()
        .map(|mut n| {
            for _ in 0..2000 {
                n = next_secret(n);
            }
            i64::from(n)
        })
        .sum())
}

pub fn part2(secrets: Vec<u32>) -> Result<i64> {
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
                .map(|secret| u8::try_from(secret % 10).unwrap())
                .collect()
        })
        .collect();
    let changes: Vec<Vec<_>> = prices
        .par_iter()
        .map(|prices| {
            prices.windows(2).map(|pair| pair[1] - pair[0]).collect()
        })
        .collect();
    let mut price_map = [0i64; 19usize.pow(4)];
    for (changes, prices) in changes.iter().zip(prices.iter()) {
        let mut buyer_price_map = [0u8; 19usize.pow(4)];
        for (signal, price) in
            changes.windows(4).zip(prices.iter().copied().skip(4)).rev()
        {
            let key = usize::from(signal[0] + 9)
                + usize::from(signal[1] + 9) * 19
                + usize::from(signal[2] + 9) * 19 * 19
                + usize::from(signal[3] + 9) * 19 * 19 * 19;
            buyer_price_map[key] = price;
        }
        for (total, buyer) in
            price_map.iter_mut().zip(buyer_price_map.iter_mut())
        {
            *total += i64::from(*buyer);
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
