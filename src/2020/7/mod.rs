use anyhow::Context as _;

type Graph = std::collections::HashMap<String, Vec<(i64, String)>>;

pub fn part1() -> anyhow::Result<i64> {
    let input = crate::util::read_file_str("data/7.txt")?;
    let graph = parse(&input)?;
    let mut colors = 0;
    for color in graph.keys() {
        if bag_contains(&graph, color, "shiny gold")? {
            colors += 1;
        }
    }
    Ok(colors)
}

pub fn part2() -> anyhow::Result<i64> {
    let input = crate::util::read_file_str("data/7.txt")?;
    let graph = parse(&input)?;
    // subtract 1 to not count the shiny gold bag itself
    count_bags(&graph, "shiny gold").map(|i| i - 1)
}

fn parse(input: &str) -> anyhow::Result<Graph> {
    let mut graph = Graph::new();
    for line in input.lines() {
        let (k, v) = parse_line(line)?;
        graph.insert(k, v);
    }
    Ok(graph)
}

fn parse_line(line: &str) -> anyhow::Result<(String, Vec<(i64, String)>)> {
    let main_rx = regex::Regex::new(r"^(.*) bags contain (.*)\.$").unwrap();
    let contents_rx = regex::Regex::new(r"^([0-9]+) (.*) bags?").unwrap();

    let captures = main_rx
        .captures(line)
        .context("line failed to match regex")?;
    let color = captures.get(1).unwrap().as_str();
    let contents = captures.get(2).unwrap().as_str();
    if contents == "no other bags" {
        Ok((color.to_string(), vec![]))
    } else {
        Ok((
            color.to_string(),
            contents
                .split(", ")
                .map(|s| {
                    let captures = contents_rx
                        .captures(s)
                        .context("line failed to match regex")?;
                    Ok((
                        captures
                            .get(1)
                            .unwrap()
                            .as_str()
                            .parse()
                            .context("invalid number of bags")?,
                        captures.get(2).unwrap().as_str().to_string(),
                    ))
                })
                .collect::<anyhow::Result<_>>()?,
        ))
    }
}

fn bag_contains(
    graph: &Graph,
    start: &str,
    target: &str,
) -> anyhow::Result<bool> {
    let mut to_check = graph
        .get(&start.to_string())
        .context("failed to find starting color in graph")?
        .clone();
    while let Some((_, next)) = to_check.pop() {
        if next == target {
            return Ok(true);
        }
        to_check.extend(
            graph
                .get(&next)
                .context("failed to find next color in graph")?
                .iter()
                .cloned(),
        );
    }
    Ok(false)
}

fn count_bags(graph: &Graph, color: &str) -> anyhow::Result<i64> {
    Ok(1 + graph
        .get(&color.to_string())
        .context("failed to find starting color in graph")?
        .iter()
        .map(|(count, child)| Ok(count * count_bags(graph, child)?))
        .collect::<anyhow::Result<Vec<_>>>()?
        .iter()
        .sum::<i64>())
}
