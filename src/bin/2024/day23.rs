use advent_of_code::prelude::*;

pub fn parse(fh: File) -> Result<Vec<(String, String)>> {
    Ok(parse::raw_lines(fh)
        .map(|line| {
            let mut parts: Vec<_> = line.split('-').collect();
            parts.sort_unstable();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect())
}

pub fn part1(edges: Vec<(String, String)>) -> Result<i64> {
    let mut found = HashSet::new();
    for (one, two) in edges
        .iter()
        .filter(|(s1, s2)| s1.starts_with('t') || s2.starts_with('t'))
    {
        for (three, four) in edges.iter().filter(|(s1, s2)| {
            [one, two].contains(&s1) ^ [one, two].contains(&s2)
        }) {
            let mut to_find = if one == three {
                [two, four]
            } else if one == four {
                [two, three]
            } else if two == three {
                [one, four]
            } else if two == four {
                [one, three]
            } else {
                unreachable!()
            };
            to_find.sort_unstable();
            let [five, six] = to_find;
            if edges.iter().any(|(s1, s2)| s1 == five && s2 == six) {
                let mut triple = [(one, two), (three, four), (five, six)];
                triple.sort_unstable();
                found.insert(triple);
            }
        }
    }
    Ok(found.len().try_into().unwrap())
}

// i don't really have a good idea why this appears to work - it only finds a
// single maximal clique for each vertex, but there could in theory be quite
// a lot of maximal cliques that don't get found here.
pub fn part2(edges: Vec<(String, String)>) -> Result<i64> {
    let computers: HashSet<_> = edges
        .iter()
        .flat_map(|(one, two)| [one, two])
        .cloned()
        .collect();
    let mut computers: Vec<String> = computers.into_iter().collect();
    computers.sort_unstable();
    let computers = computers;

    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    for (one, two) in &edges {
        connections
            .entry(one.to_string())
            .or_default()
            .push(two.to_string());
        connections
            .entry(two.to_string())
            .or_default()
            .push(one.to_string());
    }
    let connections = connections;

    let mut cliques = vec![];
    for computer in &computers {
        let mut clique = vec![computer.clone()];
        for computer in &computers {
            if clique.contains(computer) {
                continue;
            }
            if clique.iter().all(|c| connections[computer].contains(c)) {
                clique.push(computer.clone());
            }
        }
        cliques.push(clique);
    }

    let mut max_clique = cliques
        .into_iter()
        .max_by_key(|clique| clique.len())
        .unwrap();
    max_clique.sort_unstable();
    println!("{}", max_clique.join(","));

    // TODO: this is arbitrary just to give the test suite something to check
    // probably refactor this at some point to let problems return strings
    Ok(max_clique
        .iter()
        .map(|c| computers.iter().position(|c2| c == c2).unwrap())
        .sum::<usize>()
        .try_into()
        .unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 23).unwrap()).unwrap()).unwrap(),
        1308
    );
    assert_eq!(
        part2(parse(parse::data(2024, 23).unwrap()).unwrap()).unwrap(),
        4358
    );
}
