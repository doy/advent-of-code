fn small(s: &str) -> bool {
    s.chars().all(|c| c.is_lowercase())
}

fn single_small(path: &[String]) -> bool {
    path.iter().filter(|s| small(s)).count()
        == path
            .iter()
            .filter(|s| small(s))
            .collect::<std::collections::HashSet<_>>()
            .len()
}

fn paths_from1(
    graph: &std::collections::HashMap<
        String,
        std::collections::HashSet<String>,
    >,
    path: Vec<String>,
) -> i64 {
    let mut total = 0;
    for neighbor in graph[&path[path.len() - 1]].iter() {
        if small(neighbor) && path.contains(neighbor) {
            continue;
        }
        if neighbor == "end" {
            total += 1;
        } else {
            let mut path = path.clone();
            path.push(neighbor.to_string());
            total += paths_from1(graph, path);
        }
    }
    total
}

fn paths_from2(
    graph: &std::collections::HashMap<
        String,
        std::collections::HashSet<String>,
    >,
    path: Vec<String>,
) -> i64 {
    let mut total = 0;
    for neighbor in graph[&path[path.len() - 1]].iter() {
        if neighbor == "start" {
            continue;
        }
        if small(neighbor) && path.contains(neighbor) && !single_small(&path)
        {
            continue;
        }
        if neighbor == "end" {
            total += 1;
        } else {
            let mut path = path.clone();
            path.push(neighbor.to_string());
            total += paths_from2(graph, path);
        }
    }
    total
}

pub fn part1() -> anyhow::Result<i64> {
    let mut graph = std::collections::HashMap::new();
    for line in data_lines!()? {
        let line = line?;
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
    Ok(paths_from1(&graph, vec!["start".to_string()]))
}

pub fn part2() -> anyhow::Result<i64> {
    let mut graph = std::collections::HashMap::new();
    for line in data_lines!()? {
        let line = line?;
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
    Ok(paths_from2(&graph, vec!["start".to_string()]))
}
