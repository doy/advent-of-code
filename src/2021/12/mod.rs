fn small(s: &str) -> bool {
    s.bytes().all(|c| c.is_ascii_lowercase())
}

fn single_small<'a>(path: impl Iterator<Item = &'a str>) -> bool {
    let mut set = std::collections::HashSet::new();
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
    graph: &'a std::collections::HashMap<
        String,
        std::collections::HashSet<String>,
    >,
    path: &mut Vec<&'a str>,
) -> i64 {
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
    graph: &'a std::collections::HashMap<
        String,
        std::collections::HashSet<String>,
    >,
    path: &mut Vec<&'a str>,
) -> i64 {
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

pub fn part1() -> anyhow::Result<i64> {
    let mut graph = std::collections::HashMap::new();
    for line in data_lines!()? {
        let nodes: Vec<String> =
            line.split('-').map(|s| s.to_string()).collect();
        let edges = graph
            .entry(nodes[0].clone())
            .or_insert_with(std::collections::HashSet::new);
        edges.insert(nodes[1].clone());
        let edges = graph
            .entry(nodes[1].clone())
            .or_insert_with(std::collections::HashSet::new);
        edges.insert(nodes[0].clone());
    }
    Ok(paths_from1(&graph, &mut vec!["start"]))
}

pub fn part2() -> anyhow::Result<i64> {
    let mut graph = std::collections::HashMap::new();
    for line in data_lines!()? {
        let nodes: Vec<String> =
            line.split('-').map(|s| s.to_string()).collect();
        let edges = graph
            .entry(nodes[0].clone())
            .or_insert_with(std::collections::HashSet::new);
        edges.insert(nodes[1].clone());
        let edges = graph
            .entry(nodes[1].clone())
            .or_insert_with(std::collections::HashSet::new);
        edges.insert(nodes[0].clone());
    }
    Ok(paths_from2(&graph, &mut vec!["start"]))
}

#[test]
fn test() {
    assert_eq!(part1().unwrap(), 3230);
    assert_eq!(part2().unwrap(), 83475);
}
