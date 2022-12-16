#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

#[derive(Debug)]
pub struct Map {
    names: Vec<String>,
    connectivity: Vec<Vec<usize>>,
    paths: Vec<Vec<u64>>,
    flow: Vec<u16>,
    pos: usize,
    elephant: usize,
}

impl advent_of_code::graph::Graph<usize, usize> for Map {
    type Edges = Vec<usize>;

    fn edges(&self, v: usize) -> Self::Edges {
        self.connectivity[v].clone()
    }

    fn edge(&self, v: usize, e: usize) -> (usize, u64) {
        (e, 1)
    }
}

impl Map {
    fn len(&self) -> usize {
        self.flow.len()
    }

    fn room(&self, elephant: bool) -> &str {
        &self.names[self.pos(elephant)]
    }

    fn can_stay(&self, elephant: bool) -> bool {
        self.flow[self.pos(elephant)] > 0
    }

    fn done(&self) -> bool {
        self.flow.iter().copied().all(|f| f == 0)
    }

    fn pos(&self, elephant: bool) -> usize {
        if elephant {
            self.elephant
        } else {
            self.pos
        }
    }

    fn set_pos(&mut self, elephant: bool, pos: usize) {
        if elephant {
            self.elephant = pos;
        } else {
            self.pos = pos;
        }
    }

    fn flow(&self, elephant: bool) -> u16 {
        self.flow[self.pos(elephant)]
    }

    fn set_flow(&mut self, elephant: bool, flow: u16) {
        let pos = self.pos(elephant);
        self.flow[pos] = flow;
    }

    fn neighbors(&self, elephant: bool) -> &[usize] {
        &self.connectivity[self.pos(elephant)]
    }
}

pub fn parse(fh: File) -> Result<Map> {
    let mut room_names = HashMap::new();
    let mut connectivity = vec![];
    let mut flow = vec![];
    let room_idx = |s,
                    room_names: &mut HashMap<_, _>,
                    connectivity: &mut Vec<_>,
                    flow: &mut Vec<_>| {
        *room_names.entry(s).or_insert_with(|| {
            connectivity.push(vec![]);
            flow.push(0);
            flow.len() - 1
        })
    };
    let mut pos = None;
    for line in parse::raw_lines(fh) {
        let cap = regex_captures!(
            r"Valve ([^ ]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)",
            &line
        )
        .ok_or_else(|| anyhow!("failed to parse"))?;
        let node_name = cap[1].to_string();
        let node = room_idx(
            node_name.clone(),
            &mut room_names,
            &mut connectivity,
            &mut flow,
        );
        if node_name == "AA" {
            pos = Some(node);
        }
        flow[node] = cap[2].parse()?;
        connectivity[node] = cap[3]
            .split(", ")
            .map(|s| {
                room_idx(
                    s.to_string(),
                    &mut room_names,
                    &mut connectivity,
                    &mut flow,
                )
            })
            .collect();
    }
    let mut names = vec![];
    names.resize_with(flow.len(), Default::default);
    for (name, idx) in room_names {
        names[idx] = name;
    }
    let mut map = Map {
        names,
        connectivity,
        paths: vec![],
        flow,
        pos: pos.unwrap(),
        elephant: pos.unwrap(),
    };
    for i in 0..map.connectivity.len() {
        let mut paths = vec![0; map.connectivity.len()];
        let prevs = map.dijkstra_full(i);
        for (from, (_, distance)) in map.dijkstra_full(i) {
            paths[from] = distance;
        }
        map.paths.push(paths);
    }
    Ok(map)
}

pub fn part1(mut map: Map) -> Result<u64> {
    fn step(map: &mut Map, total: u64, time: u16) -> u64 {
        if time >= 30 || map.done() {
            return total;
        }

        let mut max = 0;
        if map.can_stay(false) {
            let stay_value = map.flow(false) as u64 * (29 - time as u64);
            let flow = map.flow(false);
            map.set_flow(false, 0);
            let value = step(map, total + stay_value, time + 1);
            if value > max {
                max = value;
            }
            map.set_flow(false, flow);
        } else {
            let pos = map.pos(false);
            for idx in 0..map.len() {
                if idx == pos || map.flow[idx] == 0 {
                    continue;
                }
                let distance = map.paths[pos][idx];
                map.set_pos(false, idx);
                let value = step(map, total, time + distance as u16);
                if value > max {
                    max = value;
                }
                map.set_pos(false, pos);
            }
        }
        max
    }

    Ok(step(&mut map, 0, 0))
}

pub fn part2(mut map: Map) -> Result<u64> {
    fn step(
        map: &mut Map,
        elephant: bool,
        total: u64,
        time: u16,
        transit: u64,
        elephant_transit: u64,
    ) -> u64 {
        if time >= 26 || map.done() {
            return total;
        }

        if elephant {
            let elephant_transit = elephant_transit.saturating_sub(1);
            if elephant_transit > 0 {
                return step(
                    map,
                    !elephant,
                    total,
                    if elephant { time + 1 } else { time },
                    transit,
                    elephant_transit,
                );
            }
        } else {
            let transit = transit.saturating_sub(1);
            if transit > 0 {
                return step(
                    map,
                    !elephant,
                    total,
                    if elephant { time + 1 } else { time },
                    transit,
                    elephant_transit,
                );
            }
        }

        let mut max = 0;
        if map.can_stay(elephant) {
            let stay_value = map.flow(elephant) as u64 * (25 - time as u64);
            let flow = map.flow(elephant);
            map.set_flow(elephant, 0);
            let value = step(
                map,
                !elephant,
                total + stay_value,
                if elephant { time + 1 } else { time },
                transit,
                elephant_transit,
            );
            if value > max {
                max = value;
            }
            map.set_flow(elephant, flow);
        } else {
            let pos = map.pos(elephant);
            for idx in 0..map.len() {
                if idx == pos || map.flow[idx] == 0 {
                    continue;
                }
                let distance = map.paths[pos][idx];
                map.set_pos(elephant, idx);
                let value = step(
                    map,
                    !elephant,
                    total,
                    if elephant { time + 1 } else { time },
                    if elephant { transit } else { distance },
                    if elephant { distance } else { elephant_transit },
                );
                if value > max {
                    max = value;
                }
                map.set_pos(elephant, pos);
            }
        }
        max
    }

    Ok(step(&mut map, false, 0, 0, 0, 0))
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 16).unwrap()).unwrap()).unwrap(),
        2359
    );
    assert_eq!(
        part2(parse(parse::data(2022, 16).unwrap()).unwrap()).unwrap(),
        2999
    );
}
