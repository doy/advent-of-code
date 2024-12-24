use advent_of_code::prelude::*;

pub struct Network {
    directions: Vec<Direction>,
    graph: HashMap<String, (String, String)>,
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    loop {
        let r = a % b;
        if r == 0 {
            return b;
        }
        a = b;
        b = r;
    }
}

pub fn parse(fh: File) -> Result<Network> {
    let mut lines = parse::raw_lines(fh);
    let directions = lines.next().unwrap();
    lines.next().unwrap();
    Ok(Network {
        directions: directions
            .chars()
            .map(|c| {
                if c == 'L' {
                    Direction::Left
                } else {
                    Direction::Right
                }
            })
            .collect(),
        graph: lines
            .map(|line| {
                let cap =
                    regex_captures!(r"([^ ]+) = \(([^,]+), ([^)]+)\)", &line)
                        .unwrap();
                (cap[1].to_string(), (cap[2].to_string(), cap[3].to_string()))
            })
            .collect(),
    })
}

pub fn part1(network: Network) -> Result<i64> {
    let mut vertex = "AAA".to_string();
    let mut distance = 0;

    while vertex != "ZZZ" {
        let next = network.graph[&vertex].clone();
        vertex = match network.directions[distance % network.directions.len()]
        {
            Direction::Left => next.0,
            Direction::Right => next.1,
            Direction::Up | Direction::Down => unreachable!(),
        };
        distance += 1;
    }

    Ok(distance.try_into().unwrap())
}

pub fn part2(network: Network) -> Result<i64> {
    let vertices: Vec<String> = network
        .graph
        .keys()
        .filter(|v| v.ends_with('A'))
        .cloned()
        .collect();

    let mut cycles: HashMap<_, HashMap<(String, usize), Vec<usize>>> =
        HashMap::new();
    for start_vertex in vertices {
        let mut seen = HashMap::new();
        let mut vertex = start_vertex.clone();
        let mut distance = 0;

        loop {
            if vertex.ends_with('Z') {
                let entry: &mut Vec<_> = seen
                    .entry((
                        vertex.clone(),
                        distance % network.directions.len(),
                    ))
                    .or_default();
                if entry.len() >= 2 {
                    break;
                }
                entry.push(distance);
            }
            let next = network.graph[&vertex].clone();
            vertex = match network.directions
                [distance % network.directions.len()]
            {
                Direction::Left => next.0,
                Direction::Right => next.1,
                Direction::Up | Direction::Down => unreachable!(),
            };
            distance += 1;
        }
        cycles.insert(
            start_vertex,
            seen.into_iter()
                .filter(|(_, distances)| distances.len() == 2)
                .collect(),
        );
    }

    // this is pretty dumb but it looks like we're supposed to notice that
    // the input data is very specifically shaped to make this easy
    let cycles: Vec<_> = cycles
        .values()
        .map(|cycle| cycle.values().next().unwrap()[0])
        .collect();
    let gcd = cycles.iter().copied().reduce(gcd).unwrap();
    Ok(i64::try_from(
        gcd * cycles.iter().copied().map(|n| n / gcd).product::<usize>(),
    )
    .unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 8).unwrap()).unwrap()).unwrap(),
        11309
    );
    assert_eq!(
        part2(parse(parse::data(2023, 8).unwrap()).unwrap()).unwrap(),
        13740108158591
    );
}
