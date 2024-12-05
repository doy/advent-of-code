use advent_of_code::prelude::*;

pub struct Pages {
    rules: std::collections::BTreeMap<i64, std::collections::BTreeSet<i64>>,
    updates: Vec<Vec<i64>>,
}

impl Pages {
    fn is_ordered(&self, update: &[i64]) -> bool {
        let mut seen = std::collections::BTreeSet::new();
        for i in update {
            if let Some(afters) = self.rules.get(i) {
                if afters.iter().any(|after| seen.contains(after)) {
                    return false;
                }
            }
            seen.insert(i);
        }
        true
    }

    fn sort(&self, update: &mut [i64]) {
        update.sort_unstable_by(|a, b| {
            if let Some(afters) = self.rules.get(a) {
                if afters.contains(b) {
                    return Ordering::Less;
                }
            }
            Ordering::Greater
        });
    }
}

pub fn parse(fh: File) -> Result<Pages> {
    let mut lines = parse::raw_lines(fh);
    let rules =
        parse::chunk(&mut lines)
            .map(|s| {
                let mut parts = s.split('|');
                let before = parts.next().unwrap().parse().unwrap();
                let after = parts.next().unwrap().parse().unwrap();
                (before, after)
            })
            .fold(
                std::collections::BTreeMap::<
                    i64,
                    std::collections::BTreeSet<i64>,
                >::new(),
                |mut acc, (before, after)| {
                    let entry = acc.entry(before).or_default();
                    entry.insert(after);
                    acc
                },
            );
    let updates = parse::chunk(&mut lines)
        .map(|s| s.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();
    Ok(Pages { rules, updates })
}

pub fn part1(pages: Pages) -> Result<i64> {
    let mut total = 0;
    for update in &pages.updates {
        if pages.is_ordered(update) {
            total += update[update.len() / 2];
        }
    }

    Ok(total)
}

pub fn part2(pages: Pages) -> Result<i64> {
    let mut incorrect = vec![];
    for update in &pages.updates {
        if !pages.is_ordered(update) {
            incorrect.push(update.to_vec());
        }
    }

    let mut total = 0;
    for update in &mut incorrect {
        pages.sort(update);
        total += update[update.len() / 2];
    }

    Ok(total)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 5).unwrap()).unwrap()).unwrap(),
        4766
    );
    assert_eq!(
        part2(parse(parse::data(2024, 5).unwrap()).unwrap()).unwrap(),
        6257
    );
}
