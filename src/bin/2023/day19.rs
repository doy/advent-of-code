use advent_of_code::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Category {
    X,
    M,
    A,
    S,
}

impl std::str::FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => bail!("unknown category {s}"),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    Accept,
    Reject,
    Workflow(String),
}

impl std::str::FromStr for Target {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "R" => Self::Reject,
            "A" => Self::Accept,
            workflow => Self::Workflow(workflow.to_string()),
        })
    }
}

#[derive(Clone, Debug)]
pub struct CategoryRanges {
    x: std::ops::RangeInclusive<i64>,
    m: std::ops::RangeInclusive<i64>,
    a: std::ops::RangeInclusive<i64>,
    s: std::ops::RangeInclusive<i64>,
}

impl CategoryRanges {
    fn new(
        x: std::ops::RangeInclusive<i64>,
        m: std::ops::RangeInclusive<i64>,
        a: std::ops::RangeInclusive<i64>,
        s: std::ops::RangeInclusive<i64>,
    ) -> Self {
        Self { x, m, a, s }
    }

    fn is_empty(&self) -> bool {
        self.x.is_empty()
            || self.m.is_empty()
            || self.a.is_empty()
            || self.s.is_empty()
    }

    fn size(&self) -> i64 {
        (self.x.end() - self.x.start() + 1)
            * (self.m.end() - self.m.start() + 1)
            * (self.a.end() - self.a.start() + 1)
            * (self.s.end() - self.s.start() + 1)
    }

    fn with_x(self, x: std::ops::RangeInclusive<i64>) -> Self {
        Self {
            x,
            m: self.m,
            a: self.a,
            s: self.s,
        }
    }

    fn with_m(self, m: std::ops::RangeInclusive<i64>) -> Self {
        Self {
            x: self.x,
            m,
            a: self.a,
            s: self.s,
        }
    }

    fn with_a(self, a: std::ops::RangeInclusive<i64>) -> Self {
        Self {
            x: self.x,
            m: self.m,
            a,
            s: self.s,
        }
    }

    fn with_s(self, s: std::ops::RangeInclusive<i64>) -> Self {
        Self {
            x: self.x,
            m: self.m,
            a: self.a,
            s,
        }
    }
}

#[derive(Debug)]
pub struct Conditional {
    category: Category,
    less: bool,
    val: i64,
}

#[derive(Debug)]
pub struct Rule {
    conditional: Option<Conditional>,
    target: Target,
}

impl Rule {
    fn split(
        &self,
        range: CategoryRanges,
    ) -> (Option<CategoryRanges>, Option<(Target, CategoryRanges)>) {
        let Some(conditional) = &self.conditional else {
            return (None, Some((self.target.clone(), range)));
        };
        match conditional.category {
            Category::X => {
                let (remaining, matched) = if conditional.less {
                    (
                        range
                            .clone()
                            .with_x(conditional.val..=*range.x.end()),
                        range
                            .clone()
                            .with_x(*range.x.start()..=(conditional.val - 1)),
                    )
                } else {
                    (
                        range
                            .clone()
                            .with_x(*range.x.start()..=conditional.val),
                        range
                            .clone()
                            .with_x((conditional.val + 1)..=*range.x.end()),
                    )
                };
                (
                    if remaining.is_empty() {
                        None
                    } else {
                        Some(remaining)
                    },
                    if matched.is_empty() {
                        None
                    } else {
                        Some((self.target.clone(), matched))
                    },
                )
            }
            Category::M => {
                let (remaining, matched) = if conditional.less {
                    (
                        range
                            .clone()
                            .with_m(conditional.val..=*range.m.end()),
                        range
                            .clone()
                            .with_m(*range.m.start()..=(conditional.val - 1)),
                    )
                } else {
                    (
                        range
                            .clone()
                            .with_m(*range.m.start()..=conditional.val),
                        range
                            .clone()
                            .with_m((conditional.val + 1)..=*range.m.end()),
                    )
                };
                (
                    if remaining.is_empty() {
                        None
                    } else {
                        Some(remaining)
                    },
                    if matched.is_empty() {
                        None
                    } else {
                        Some((self.target.clone(), matched))
                    },
                )
            }
            Category::A => {
                let (remaining, matched) = if conditional.less {
                    (
                        range
                            .clone()
                            .with_a(conditional.val..=*range.a.end()),
                        range
                            .clone()
                            .with_a(*range.a.start()..=(conditional.val - 1)),
                    )
                } else {
                    (
                        range
                            .clone()
                            .with_a(*range.a.start()..=conditional.val),
                        range
                            .clone()
                            .with_a((conditional.val + 1)..=*range.a.end()),
                    )
                };
                (
                    if remaining.is_empty() {
                        None
                    } else {
                        Some(remaining)
                    },
                    if matched.is_empty() {
                        None
                    } else {
                        Some((self.target.clone(), matched))
                    },
                )
            }
            Category::S => {
                let (remaining, matched) = if conditional.less {
                    (
                        range
                            .clone()
                            .with_s(conditional.val..=*range.s.end()),
                        range
                            .clone()
                            .with_s(*range.s.start()..=(conditional.val - 1)),
                    )
                } else {
                    (
                        range
                            .clone()
                            .with_s(*range.s.start()..=conditional.val),
                        range
                            .clone()
                            .with_s((conditional.val + 1)..=*range.s.end()),
                    )
                };
                (
                    if remaining.is_empty() {
                        None
                    } else {
                        Some(remaining)
                    },
                    if matched.is_empty() {
                        None
                    } else {
                        Some((self.target.clone(), matched))
                    },
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct Workflows(HashMap<String, Vec<Rule>>);

impl Workflows {
    fn accept(&self, part: &Part) -> bool {
        let mut workflow = "in";
        loop {
            for rule in &self.0[workflow] {
                if let Some(conditional) = &rule.conditional {
                    let left = part.0[&conditional.category];
                    let right = conditional.val;
                    if conditional.less {
                        if left >= right {
                            continue;
                        }
                    } else if left <= right {
                        continue;
                    }
                }
                match &rule.target {
                    Target::Accept => return true,
                    Target::Reject => return false,
                    Target::Workflow(new_workflow) => workflow = new_workflow,
                }
                break;
            }
        }
    }
}

#[derive(Debug)]
pub struct Part(HashMap<Category, i64>);

impl Part {
    fn value(&self) -> i64 {
        self.0.values().sum()
    }
}

pub fn parse(fh: File) -> Result<(Workflows, Vec<Part>)> {
    let mut lines = parse::raw_lines(fh);
    let workflows = parse::chunk(&mut lines)
        .map(|line| {
            let cap = regex_captures!(r"(\w+)\{(.*)\}", &line).unwrap();
            let workflow = cap[1].to_string();
            let rules = cap[2]
                .split(',')
                .map(|rule| {
                    if rule.contains(':') {
                        let cap =
                            regex_captures!(r"(\w+)(<|>)(\d+):(\w+)", &rule)
                                .unwrap();
                        let category = cap[1].parse().unwrap();
                        let less = &cap[2] == "<";
                        let val = cap[3].parse().unwrap();
                        let target = cap[4].parse().unwrap();

                        Rule {
                            conditional: Some(Conditional {
                                category,
                                less,
                                val,
                            }),
                            target,
                        }
                    } else {
                        Rule {
                            conditional: None,
                            target: rule.parse().unwrap(),
                        }
                    }
                })
                .collect();
            (workflow, rules)
        })
        .collect();
    let parts = parse::chunk(&mut lines)
        .map(|line| {
            let line = &line[1..(line.len() - 1)];
            Part(
                line.split(',')
                    .map(|category| {
                        let mut parts = category.split('=');
                        let category = parts.next().unwrap().parse().unwrap();
                        let value = parts.next().unwrap().parse().unwrap();
                        (category, value)
                    })
                    .collect(),
            )
        })
        .collect();
    Ok((Workflows(workflows), parts))
}

pub fn part1((workflows, parts): (Workflows, Vec<Part>)) -> Result<i64> {
    Ok(parts
        .into_iter()
        .filter_map(|part| workflows.accept(&part).then(|| part.value()))
        .sum())
}

pub fn part2((workflows, _): (Workflows, Vec<Part>)) -> Result<i64> {
    let mut ranges = vec![(
        Target::Workflow("in".to_string()),
        CategoryRanges::new(1..=4000, 1..=4000, 1..=4000, 1..=4000),
    )];
    loop {
        let mut new_ranges = vec![];
        let mut done = true;
        for (workflow, range) in &ranges {
            let mut range = range.clone();
            let Target::Workflow(workflow) = workflow else {
                new_ranges.push((workflow.clone(), range));
                continue;
            };
            for rule in &workflows.0[workflow.as_str()] {
                let (old_range, new_range) = rule.split(range);
                if let Some(new_range) = new_range {
                    new_ranges.push(new_range);
                    done = false;
                }
                if let Some(old_range) = old_range {
                    range = old_range;
                } else {
                    break;
                }
            }
        }
        ranges = new_ranges;
        if done {
            break;
        }
    }
    Ok(ranges
        .into_iter()
        .filter_map(|(target, range)| {
            if target == Target::Accept {
                Some(range.size())
            } else {
                None
            }
        })
        .sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 19).unwrap()).unwrap()).unwrap(),
        449531
    );
    assert_eq!(
        part2(parse(parse::data(2023, 19).unwrap()).unwrap()).unwrap(),
        122756210763577
    );
}
