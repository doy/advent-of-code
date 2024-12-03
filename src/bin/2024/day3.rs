use advent_of_code::prelude::*;

static RX: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
    Regex::new(r"(mul|do|don't)\((?:([0-9]+),([0-9]+))?\)").unwrap()
});

pub fn parse(fh: File) -> Result<String> {
    Ok(parse::string(fh))
}

pub fn part1(insns: String) -> Result<i64> {
    let mut total = 0;
    for cap in RX.captures_iter(&insns) {
        if cap.get(1).unwrap().as_str() != "mul" {
            continue;
        }
        let x: i64 = cap.get(2).unwrap().as_str().parse().unwrap();
        let y: i64 = cap.get(3).unwrap().as_str().parse().unwrap();
        total += x * y;
    }
    Ok(total)
}

pub fn part2(insns: String) -> Result<i64> {
    let mut total = 0;
    let mut enabled = true;
    for cap in RX.captures_iter(&insns) {
        match cap.get(1).unwrap().as_str() {
            "mul" => {
                if enabled {
                    let x: i64 =
                        cap.get(2).unwrap().as_str().parse().unwrap();
                    let y: i64 =
                        cap.get(3).unwrap().as_str().parse().unwrap();
                    total += x * y;
                }
            }
            "do" => {
                if cap.get(2).is_none() && cap.get(3).is_none() {
                    enabled = true;
                }
            }
            "don't" => {
                if cap.get(2).is_none() && cap.get(3).is_none() {
                    enabled = false;
                }
            }
            _ => {}
        }
    }
    Ok(total)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 3).unwrap()).unwrap()).unwrap(),
        166630675
    );
    assert_eq!(
        part2(parse(parse::data(2024, 3).unwrap()).unwrap()).unwrap(),
        93465710
    );
}
