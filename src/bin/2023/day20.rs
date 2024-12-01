use advent_of_code::prelude::*;

pub enum ModuleType {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

pub struct Module {
    name: String,
    ty: ModuleType,
    destinations: Vec<String>,
    low_pulses: i64,
    high_pulses: i64,
}

impl std::str::FromStr for Module {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");
        let module_name = parts.next().unwrap();
        let destinations = parts.next().unwrap();
        let destinations: Vec<_> =
            destinations.split(", ").map(|s| s.to_string()).collect();
        let (ty, name) = match module_name.chars().next().unwrap() {
            '&' => (
                ModuleType::Conjunction(HashMap::new()),
                module_name.chars().skip(1).collect(),
            ),
            '%' => (
                ModuleType::FlipFlop(false),
                module_name.chars().skip(1).collect(),
            ),
            _ => {
                if module_name == "broadcaster" {
                    (ModuleType::Broadcast, module_name.to_string())
                } else {
                    bail!("failed to parse module {module_name}")
                }
            }
        };

        Ok(Self {
            name,
            ty,
            destinations,
            low_pulses: 0,
            high_pulses: 0,
        })
    }
}

pub struct Network {
    modules: HashMap<String, Module>,
}

impl Network {
    fn pulse(&mut self) {
        let mut pulses = VecDeque::new();
        pulses.push_front(("".to_string(), "broadcaster".to_string(), false));
        while let Some((src, dest, high)) = pulses.pop_back() {
            let module = self.modules.get_mut(&dest).unwrap();

            if high {
                module.high_pulses += 1;
            } else {
                module.low_pulses += 1;
            }

            let mut pulse = None;
            match module.ty {
                ModuleType::Broadcast => {
                    pulse = Some(high);
                }
                ModuleType::FlipFlop(ref mut on) => {
                    if !high {
                        *on = !*on;
                        pulse = Some(*on);
                    }
                }
                ModuleType::Conjunction(ref mut inputs) => {
                    *inputs.get_mut(&src).unwrap() = high;
                    pulse = Some(!inputs.values().all(|high| *high));
                }
            }

            if let Some(pulse) = pulse {
                for new_dest in module.destinations.clone() {
                    pulses.push_back((dest.to_string(), new_dest, pulse));
                }
            }
        }
    }
}

impl std::str::FromStr for Network {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut modules: HashMap<_, _> = s
            .trim()
            .split('\n')
            .map(|s| {
                let module: Module = s.parse()?;
                Ok::<_, Self::Err>((module.name.clone(), module))
            })
            .collect::<Result<_, _>>()?;
        let module_names: Vec<_> = modules.keys().cloned().collect();
        for name in module_names {
            let destinations = modules[&name].destinations.clone();
            for dest in destinations {
                if let Some(ModuleType::Conjunction(ref mut inputs)) =
                    modules.get_mut(&dest).map(|module| &mut module.ty)
                {
                    inputs.insert(name.clone(), false);
                }
            }
        }
        let destinations: HashSet<String> = modules
            .values()
            .flat_map(|module| module.destinations.iter())
            .cloned()
            .collect();
        for destination in destinations {
            if !modules.contains_key(&destination) {
                modules.insert(
                    destination.clone(),
                    Module {
                        name: destination,
                        ty: ModuleType::Broadcast,
                        destinations: vec![],
                        low_pulses: 0,
                        high_pulses: 0,
                    },
                );
            }
        }
        Ok(Self { modules })
    }
}

pub fn parse(mut fh: File) -> Result<Network> {
    let mut s = String::new();
    fh.read_to_string(&mut s)?;
    s.parse()
}

pub fn part1(mut network: Network) -> Result<i64> {
    for _ in 0..1000 {
        network.pulse();
    }
    Ok(network
        .modules
        .values()
        .map(|module| module.low_pulses)
        .sum::<i64>()
        * network
            .modules
            .values()
            .map(|module| module.high_pulses)
            .sum::<i64>())
}

pub fn part2(_: Network) -> Result<i64> {
    // let mut count = 0;
    // loop {
    //     network.pulse();
    //     count += 1;
    //     if network.modules.get("rx").unwrap().low_pulses > 0 {
    //         return Ok(count);
    //     }
    // }
    todo!()
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 20).unwrap()).unwrap()).unwrap(),
        681194780
    );
    assert_eq!(
        part2(parse(parse::data(2023, 20).unwrap()).unwrap()).unwrap(),
        0
    );
}
