pub fn parse(
    fh: std::fs::File,
) -> anyhow::Result<impl Iterator<Item = String>> {
    Ok(crate::util::parse::lines(fh))
}

pub fn part1(lines: impl Iterator<Item = String>) -> anyhow::Result<i64> {
    let mut total = 0;
    for line in lines {
        let mut open = vec![];
        for c in line.chars() {
            match c {
                '{' | '(' | '[' | '<' => {
                    open.push(c);
                }
                '}' | ')' | ']' | '>' => {
                    let c_open = open.pop();
                    if let Some(c_open) = c_open {
                        let expected = match c_open {
                            '{' => '}',
                            '(' => ')',
                            '[' => ']',
                            '<' => '>',
                            _ => unreachable!(),
                        };
                        if c != expected {
                            total += match c {
                                '}' => 1197,
                                ')' => 3,
                                ']' => 57,
                                '>' => 25137,
                                _ => unreachable!(),
                            };
                            break;
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }
    Ok(total)
}

pub fn part2(lines: impl Iterator<Item = String>) -> anyhow::Result<i64> {
    let mut scores = vec![];
    for line in lines {
        let mut open = vec![];
        let mut skip = false;
        for c in line.chars() {
            match c {
                '{' | '(' | '[' | '<' => {
                    open.push(c);
                }
                '}' | ')' | ']' | '>' => {
                    let c_open = open.pop();
                    if let Some(c_open) = c_open {
                        let expected = match c_open {
                            '{' => '}',
                            '(' => ')',
                            '[' => ']',
                            '<' => '>',
                            _ => unreachable!(),
                        };
                        if c != expected {
                            skip = true;
                            break;
                        }
                    } else {
                        skip = true;
                        break;
                    }
                }
                _ => {
                    skip = true;
                    break;
                }
            }
        }
        if !skip {
            scores.push(open.iter().rev().fold(0, |acc, c| {
                acc * 5
                    + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!(),
                    }
            }));
        }
    }
    scores.sort_unstable();
    Ok(scores[scores.len() / 2])
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(crate::util::data(2021, 10).unwrap()).unwrap()).unwrap(),
        166191
    );
    assert_eq!(
        part2(parse(crate::util::data(2021, 10).unwrap()).unwrap()).unwrap(),
        1152088313
    );
}
