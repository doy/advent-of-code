#[allow(clippy::type_complexity)]
fn parse() -> anyhow::Result<(Vec<u8>, std::collections::HashMap<Vec<u8>, u8>)>
{
    let mut lines = data_lines!()?;
    let polymer = lines.next().unwrap();
    lines.next();

    let mut rules = std::collections::HashMap::new();
    for line in lines {
        let rule: Vec<_> = line.split(" -> ").collect();
        rules.insert(rule[0].as_bytes().to_vec(), rule[1].as_bytes()[0]);
    }
    Ok((polymer.as_bytes().to_vec(), rules))
}

fn process(
    polymer: &[u8],
    rules: &std::collections::HashMap<Vec<u8>, u8>,
) -> Vec<u8> {
    let mut insertions = vec![];
    for (i, elements) in polymer.windows(2).enumerate() {
        for (pattern, insert) in rules {
            if pattern == elements {
                insertions.push((i + 1, insert));
            }
        }
    }
    let mut polymer = polymer.to_vec();
    for (idx, c) in insertions.iter().rev() {
        polymer.insert(*idx, **c);
    }
    polymer
}

pub fn part1() -> anyhow::Result<i64> {
    let (mut polymer, rules) = parse()?;

    for _ in 0..10 {
        polymer = process(&polymer, &rules);
    }
    let mut elements = std::collections::HashMap::new();
    for element in polymer {
        let count = elements.entry(element).or_insert(0);
        *count += 1;
    }
    Ok(elements.values().max().unwrap() - elements.values().min().unwrap())
}

pub fn part2() -> anyhow::Result<i64> {
    let (polymer, rules) = parse()?;

    let mut pairs = std::collections::HashMap::new();
    for pair in polymer.windows(2) {
        let count = pairs.entry([pair[0], pair[1]]).or_insert(0);
        *count += 1;
    }

    for _ in 0..40 {
        let mut next = std::collections::HashMap::new();
        for (pair, count) in &mut pairs {
            let insert = rules[&pair[..]];
            let new_pair1 = [pair[0], insert];
            let new_pair2 = [insert, pair[1]];
            let next_count = next.entry(new_pair1).or_insert(0);
            *next_count += *count;
            let next_count = next.entry(new_pair2).or_insert(0);
            *next_count += *count;
        }
        pairs = next;
    }
    let mut elements = std::collections::HashMap::new();
    for (pair, count) in pairs {
        let element_count = elements.entry(pair[0]).or_insert(0);
        *element_count += count;
        let element_count = elements.entry(pair[1]).or_insert(0);
        *element_count += count;
    }
    let element_count = elements.entry(polymer[0]).or_insert(0);
    *element_count += 1;
    let element_count =
        elements.entry(polymer[polymer.len() - 1]).or_insert(0);
    *element_count += 1;
    Ok(elements.values().max().unwrap() / 2
        - elements.values().min().unwrap() / 2)
}

#[test]
fn test() {
    assert_eq!(part1().unwrap(), 2874);
    assert_eq!(part2().unwrap(), 5208377027195);
}
