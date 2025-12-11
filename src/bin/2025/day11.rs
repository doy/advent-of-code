use advent_of_code::prelude::*;

pub struct Reactor {
    device_names: HashMap<String, usize>,
    connections: Vec<Vec<usize>>,
}

pub fn parse(fh: File) -> Result<Reactor> {
    let devices: Vec<_> = parse::raw_lines(fh)
        .map(|line| {
            let mut parts = line.split(": ");
            let device_name = parts.next().unwrap().to_string();
            let connections: Vec<_> = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|s| s.to_string())
                .collect();
            (device_name, connections)
        })
        .chain(std::iter::once(("out".to_string(), vec![])))
        .enumerate()
        .collect();
    let device_names: HashMap<_, _> = devices
        .iter()
        .map(|(i, (device_name, _))| (device_name.to_string(), *i))
        .collect();
    let connections_rev: Vec<Vec<_>> = devices
        .iter()
        .map(|(_, (_, connections))| {
            connections.iter().map(|name| device_names[name]).collect()
        })
        .collect();
    let connections: Vec<_> = (0..connections_rev.len())
        .map(|i| {
            connections_rev
                .iter()
                .enumerate()
                .filter_map(|(j, connections)| {
                    connections.contains(&i).then_some(j)
                })
                .collect()
        })
        .collect();
    Ok(Reactor {
        device_names,
        connections,
    })
}

impl Reactor {
    fn count_paths(
        &self,
        start: usize,
        end: usize,
        through: &[usize],
        cache: &mut [HashMap<Vec<usize>, i64>],
    ) -> i64 {
        if end == start {
            if through.is_empty() {
                1
            } else {
                0
            }
        } else if let Some(val) = cache[end].get(through) {
            *val
        } else {
            let total = self.connections[end]
                .iter()
                .copied()
                .map(|connected_device| {
                    let through = if through.contains(&connected_device) {
                        std::borrow::Cow::Owned(
                            through
                                .iter()
                                .copied()
                                .filter(|i| *i != connected_device)
                                .collect::<Vec<_>>(),
                        )
                    } else {
                        std::borrow::Cow::Borrowed(through)
                    };
                    self.count_paths(start, connected_device, &through, cache)
                })
                .sum();
            cache[end].insert(through.to_vec(), total);
            total
        }
    }
}

pub fn part1(reactor: Reactor) -> Result<i64> {
    let mut cache = vec![HashMap::new(); reactor.device_names.len()];
    Ok(reactor.count_paths(
        reactor.device_names["you"],
        reactor.device_names["out"],
        &[],
        &mut cache,
    ))
}

pub fn part2(reactor: Reactor) -> Result<i64> {
    let mut cache = vec![HashMap::new(); reactor.device_names.len()];
    Ok(reactor.count_paths(
        reactor.device_names["svr"],
        reactor.device_names["out"],
        &[reactor.device_names["dac"], reactor.device_names["fft"]],
        &mut cache,
    ))
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2025, 11).unwrap()).unwrap()).unwrap(),
        640
    );
    assert_eq!(
        part2(parse(parse::data(2025, 11).unwrap()).unwrap()).unwrap(),
        367579641755680
    );
}
