use advent_of_code::prelude::*;

fn small(s: &str) -> bool {
    s.bytes().all(|c| c.is_ascii_lowercase())
}

fn single_small<'a>(path: impl Iterator<Item = &'a str>) -> bool {
    let mut set = HashSet::new();
    for s in path {
        if !small(s) {
            continue;
        }
        if set.contains(s) {
            return false;
        }
        set.insert(s);
    }
    true
}

fn paths_from1<'a>(
    graph: &'a HashMap<String, HashSet<String>>,
    path: &mut Vec<&'a str>,
) -> u64 {
    let mut total = 0;
    for neighbor in graph[path[path.len() - 1]].iter() {
        if small(neighbor) && path.contains(&neighbor.as_ref()) {
            continue;
        }
        if neighbor == "end" {
            total += 1;
        } else {
            path.push(neighbor);
            total += paths_from1(graph, path);
            path.pop();
        }
    }
    total
}

fn paths_from2<'a>(
    graph: &'a HashMap<String, HashSet<String>>,
    path: &mut Vec<&'a str>,
) -> u64 {
    let mut total = 0;
    for neighbor in graph[path[path.len() - 1]].iter() {
        if neighbor == "start" {
            continue;
        }
        if small(neighbor)
            && path.contains(&neighbor.as_ref())
            && !single_small(path.iter().copied())
        {
            continue;
        }
        if neighbor == "end" {
            total += 1;
        } else {
            path.push(neighbor);
            total += paths_from2(graph, path);
            path.pop();
        }
    }
    total
}

pub fn parse(fh: File) -> Result<HashMap<String, HashSet<String>>> {
    let mut graph = HashMap::new();
    for line in parse::raw_lines(fh) {
        let nodes: Vec<_> = line.split('-').map(|s| s.to_string()).collect();
        let edges =
            graph.entry(nodes[0].clone()).or_insert_with(HashSet::new);
        edges.insert(nodes[1].clone());
        let edges =
            graph.entry(nodes[1].clone()).or_insert_with(HashSet::new);
        edges.insert(nodes[0].clone());
    }
    Ok(graph)
}

pub fn part1(graph: HashMap<String, HashSet<String>>) -> Result<u64> {
    Ok(paths_from1(&graph, &mut vec!["start"]))
}

pub fn part2(graph: HashMap<String, HashSet<String>>) -> Result<u64> {
    Ok(paths_from2(&graph, &mut vec!["start"]))
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2021, 12).unwrap()).unwrap()).unwrap(),
        3230
    );
    assert_eq!(
        part2(parse(parse::data(2021, 12).unwrap()).unwrap()).unwrap(),
        83475
    );
}
