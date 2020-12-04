use anyhow::Context as _;

const REQUIRED_KEYS: &[&str] =
    &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn part1() -> anyhow::Result<()> {
    let batch = crate::util::read_file_str("data/4.txt")?;
    let mut valid = 0;
    for passport in parse(&batch)? {
        let mut cur_valid = true;
        for key in REQUIRED_KEYS {
            if !passport.contains_key(&key.to_string()) {
                cur_valid = false;
                break;
            }
        }
        if cur_valid {
            valid += 1;
        }
    }
    println!("{}", valid);
    Ok(())
}

pub fn part2() -> anyhow::Result<()> {
    let batch = crate::util::read_file_str("data/4.txt")?;
    let mut valid = 0;
    for passport in parse(&batch)? {
        let mut cur_valid = true;
        for key in REQUIRED_KEYS {
            match passport.get(&key.to_string()) {
                Some(val) => {
                    if !validate(key, val)? {
                        cur_valid = false;
                        break;
                    }
                }
                None => {
                    cur_valid = false;
                    break;
                }
            }
        }
        if cur_valid {
            valid += 1;
        }
    }
    println!("{}", valid);
    Ok(())
}

fn parse(
    batch: &str,
) -> anyhow::Result<Vec<std::collections::HashMap<String, String>>> {
    let mut res = vec![];
    let mut cur = std::collections::HashMap::new();
    for line in batch.lines() {
        if line.is_empty() {
            res.push(cur);
            cur = std::collections::HashMap::new();
            continue;
        }

        for field in line.split(' ') {
            let mut parts = field.split(':');
            let key = parts.next().with_context(|| {
                format!("failed to parse field '{}'", field)
            })?;
            let value = parts.next().with_context(|| {
                format!("failed to parse field '{}'", field)
            })?;
            cur.insert(key.to_string(), value.to_string());
        }
    }
    if !cur.is_empty() {
        res.push(cur);
    }
    Ok(res)
}

fn validate(key: &str, val: &str) -> anyhow::Result<bool> {
    match key {
        "byr" => match val.parse::<i32>() {
            Ok(year) => Ok(year >= 1920 && year <= 2002),
            Err(_) => Ok(false),
        },
        "iyr" => match val.parse::<i32>() {
            Ok(year) => Ok(year >= 2010 && year <= 2020),
            Err(_) => Ok(false),
        },
        "eyr" => match val.parse::<i32>() {
            Ok(year) => Ok(year >= 2020 && year <= 2030),
            Err(_) => Ok(false),
        },
        "hgt" => {
            if val.len() < 3 {
                Ok(false)
            } else if val.ends_with("in") {
                match val[0..val.len() - 2].parse::<i32>() {
                    Ok(inches) => Ok(inches >= 59 && inches <= 76),
                    Err(_) => Ok(false),
                }
            } else if val.ends_with("cm") {
                match val[0..val.len() - 2].parse::<i32>() {
                    Ok(inches) => Ok(inches >= 150 && inches <= 193),
                    Err(_) => Ok(false),
                }
            } else {
                Ok(false)
            }
        }
        "hcl" => Ok(val.len() == 7
            && val.starts_with('#')
            && val[1..]
                == val[1..]
                    .matches(|c: char| c.is_ascii_hexdigit())
                    .collect::<String>()),
        "ecl" => Ok(val == "amb"
            || val == "blu"
            || val == "brn"
            || val == "gry"
            || val == "grn"
            || val == "hzl"
            || val == "oth"),
        "pid" => Ok(val.len() == 9
            && val
                == val
                    .matches(|c: char| c.is_ascii_digit())
                    .collect::<String>()),
        _ => Err(anyhow::anyhow!("invalid key found: {}", key)),
    }
}
