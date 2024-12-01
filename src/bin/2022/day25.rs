use advent_of_code::prelude::*;

fn from_snafu(line: String) -> i64 {
    let mut total = 0;
    for c in line.chars() {
        let digit = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("unknown char {}", c),
        };
        total *= 5;
        total += digit;
    }
    total
}

fn to_snafu(mut n: i64) -> String {
    let mut s = String::new();
    while n > 0 {
        let rem = n % 5;
        let c = match rem {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => unreachable!(),
        };
        s.insert(0, c);
        n = (n + 2) / 5;
    }
    s
}

pub fn parse(fh: File) -> Result<impl Iterator<Item = String>> {
    Ok(parse::raw_lines(fh))
}

pub fn part1(lines: impl Iterator<Item = String>) -> Result<String> {
    Ok(to_snafu(lines.map(from_snafu).sum()))
}

pub fn part2(_: impl Iterator<Item = String>) -> Result<i64> {
    todo!()
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 25).unwrap()).unwrap()).unwrap(),
        "2-20=01--0=0=0=2-120"
    );
    // assert_eq!(
    //     part2(parse(parse::data(2022, 25).unwrap()).unwrap()).unwrap(),
    //     0
    // );
}
