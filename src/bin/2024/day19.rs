use advent_of_code::prelude::*;

#[derive(Debug)]
pub struct Towels {
    towels: HashSet<Vec<u8>>,
    designs: Vec<Vec<u8>>,
}

impl Towels {
    fn can_make<'a>(
        &self,
        design: &'a [u8],
        cache: &mut HashMap<&'a [u8], bool>,
    ) -> bool {
        if cache.contains_key(design) {
            return cache[design];
        }
        for towel in &self.towels {
            if design.starts_with(towel)
                && self.can_make(&design[towel.len()..], cache)
            {
                cache.insert(design, true);
                return true;
            }
        }
        cache.insert(design, false);
        false
    }

    fn count_arrangements<'a>(
        &self,
        design: &'a [u8],
        cache: &mut HashMap<&'a [u8], i64>,
    ) -> i64 {
        if cache.contains_key(design) {
            return cache[design];
        }
        let mut total = 0;
        for towel in &self.towels {
            if design.starts_with(towel) {
                total +=
                    self.count_arrangements(&design[towel.len()..], cache);
            }
        }
        cache.insert(design, total);
        total
    }
}

pub fn parse(fh: File) -> Result<Towels> {
    let mut lines = parse::raw_lines(fh);
    let towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string().into_bytes())
        .collect();
    lines.next().unwrap();
    let designs = lines.map(|s| s.into_bytes()).collect();
    Ok(Towels { towels, designs })
}

pub fn part1(towels: Towels) -> Result<i64> {
    Ok(towels
        .designs
        .par_iter()
        .map(|design| {
            let mut cache = HashMap::new();
            cache.insert(&[][..], true);
            if towels.can_make(design, &mut cache) {
                1
            } else {
                0
            }
        })
        .sum())
}

pub fn part2(towels: Towels) -> Result<i64> {
    Ok(towels
        .designs
        .par_iter()
        .map(|design| {
            let mut cache = HashMap::new();
            cache.insert(&[][..], 1);
            towels.count_arrangements(design, &mut cache)
        })
        .sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 19).unwrap()).unwrap()).unwrap(),
        360
    );
    assert_eq!(
        part2(parse(parse::data(2024, 19).unwrap()).unwrap()).unwrap(),
        577474410989846
    );
}
