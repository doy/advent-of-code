use advent_of_code::prelude::*;

pub struct Map {
    grid: Grid<u8>,
    start: Pos,
    end: Pos,
}

impl advent_of_code::graph::Graph<Pos, Pos> for Map {
    fn edges(&self, v: Pos) -> impl IntoIterator<Item = Pos> {
        self.grid.adjacent(v, false)
    }

    fn edge(&self, v: Pos, e: Pos) -> (Pos, u64) {
        (
            e,
            if self.grid[e] >= self.grid[v].saturating_sub(1) {
                1
            } else {
                (self.grid.rows().0 * self.grid.cols().0)
                    .try_into()
                    .unwrap()
            },
        )
    }
}

pub fn parse(fh: File) -> Result<Map> {
    let mut start = None;
    let mut end = None;
    let grid = parse::grid(parse::raw_lines(fh), |c, pos| match c {
        b'a'..=b'z' => c - b'a',
        b'S' => {
            start = Some(pos);
            0
        }
        b'E' => {
            end = Some(pos);
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
    Ok(map.dijkstra(map.end, |v| v == map.start).unwrap().0)
}

pub fn part2(map: Map) -> Result<u64> {
    Ok(map
        .dijkstra(map.end, |v| map.grid[v.0][v.1] == 0)
        .unwrap()
        .0)
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
