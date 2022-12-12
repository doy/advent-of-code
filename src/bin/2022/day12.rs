#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

pub struct Map {
    grid: Grid<u8>,
    start: (Row, Col),
    end: (Row, Col),
}

impl advent_of_code::graph::Graph<(Row, Col), (Row, Col)> for Map {
    type Edges = advent_of_code::grid::Adjacent;

    fn edges(&self, v: (Row, Col)) -> Self::Edges {
        self.grid.adjacent(v.0, v.1, false)
    }

    fn edge(&self, v: (Row, Col), e: (Row, Col)) -> ((Row, Col), u64) {
        (
            e,
            if self.grid[e.0][e.1] <= self.grid[v.0][v.1] + 1 {
                1
            } else {
                999_999_999
            },
        )
    }
}

pub fn parse(fh: File) -> Result<Map> {
    let mut start = None;
    let mut end = None;
    let grid = parse::grid(parse::raw_lines(fh), |c, row, col| match c {
        b'a'..=b'z' => c - b'a',
        b'S' => {
            start = Some((row, col));
            0
        }
        b'E' => {
            end = Some((row, col));
            b'z' - b'a'
        }
        _ => panic!("unknown map char '{c}'"),
    });
    Ok(Map {
        grid,
        start: start.expect("start not found"),
        end: end.expect("end not found"),
    })
}

pub fn part1(map: Map) -> Result<u64> {
    Ok(map.dijkstra(map.start, map.end).0)
}

pub fn part2(map: Map) -> Result<u64> {
    Ok(map
        .grid
        .indexed_cells()
        .filter_map(|(pos, height)| {
            if *height == 0 {
                Some(map.dijkstra(pos, map.end).0)
            } else {
                None
            }
        })
        .min()
        .unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 12).unwrap()).unwrap()).unwrap(),
        504
    );
    assert_eq!(
        part2(parse(parse::data(2022, 12).unwrap()).unwrap()).unwrap(),
        500
    );
}
