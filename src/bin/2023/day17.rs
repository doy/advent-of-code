use advent_of_code::prelude::*;

fn add_offset(pos: Pos, offset: IPos, max: Size) -> Option<Pos> {
    if let (Some(row), Some(col)) = (
        pos.0 .0.checked_add_signed(offset.0 .0),
        pos.1 .0.checked_add_signed(offset.1 .0),
    ) {
        let row = Row(row);
        let col = Col(col);
        if row < max.0 && col < max.1 {
            return Some(Pos(row, col));
        }
    }
    None
}

#[derive(Debug)]
pub struct Crucible {
    map: Grid<u8>,
}

impl advent_of_code::graph::Graph<(Pos, Option<Direction>, u8), Pos>
    for Crucible
{
    type Edges = Vec<Pos>;

    fn edges(&self, v: (Pos, Option<Direction>, u8)) -> Self::Edges {
        let (pos, direction, length) = v;

        if let Some(direction) = direction {
            let mut edges: Vec<_> = direction
                .turns()
                .into_iter()
                .filter_map(|direction| {
                    let offset = direction.offset();
                    add_offset(pos, offset, self.map.size())
                })
                .collect();
            if length + 1 < 3 {
                let offset = direction.offset();
                if let Some(pos) = add_offset(pos, offset, self.map.size()) {
                    edges.push(pos);
                }
            }
            edges
        } else {
            [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
            .into_iter()
            .filter_map(|direction| {
                let offset = direction.offset();
                add_offset(pos, offset, self.map.size())
            })
            .collect()
        }
    }

    fn edge(
        &self,
        v: (Pos, Option<Direction>, u8),
        e: Pos,
    ) -> ((Pos, Option<Direction>, u8), u64) {
        let (pos, direction, length) = v;
        let new_pos = e;
        let new_direction = Direction::from_pos(pos, new_pos);

        (
            (
                new_pos,
                Some(new_direction),
                if direction == Some(new_direction) {
                    length + 1
                } else {
                    0
                },
            ),
            u64::from(self.map[new_pos]),
        )
    }
}

#[derive(Debug)]
pub struct UltraCrucible {
    map: Grid<u8>,
}

impl advent_of_code::graph::Graph<(Pos, Option<Direction>, u8), Pos>
    for UltraCrucible
{
    type Edges = Vec<Pos>;

    fn edges(&self, v: (Pos, Option<Direction>, u8)) -> Self::Edges {
        let (pos, direction, length) = v;

        if let Some(direction) = direction {
            let mut edges = vec![];
            if length + 1 >= 4 {
                edges.extend(direction.turns().into_iter().filter_map(
                    |direction| {
                        let offset = direction.offset();
                        add_offset(pos, offset, self.map.size())
                    },
                ));
            }
            if length + 1 < 10 {
                let offset = direction.offset();
                if let Some(pos) = add_offset(pos, offset, self.map.size()) {
                    edges.push(pos);
                }
            }
            edges
        } else {
            [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
            .into_iter()
            .filter_map(|direction| {
                let offset = direction.offset();
                add_offset(pos, offset, self.map.size())
            })
            .collect()
        }
    }

    fn edge(
        &self,
        v: (Pos, Option<Direction>, u8),
        e: Pos,
    ) -> ((Pos, Option<Direction>, u8), u64) {
        let (pos, direction, length) = v;
        let new_pos = e;
        let new_direction = Direction::from_pos(pos, new_pos);

        (
            (
                new_pos,
                Some(new_direction),
                if direction == Some(new_direction) {
                    length + 1
                } else {
                    0
                },
            ),
            u64::from(self.map[new_pos]),
        )
    }
}

pub fn parse(fh: File) -> Result<Grid<u8>> {
    Ok(parse::digit_grid(parse::raw_lines(fh)))
}

pub fn part1(map: Grid<u8>) -> Result<i64> {
    let crucible = Crucible { map };
    let (weight, _) =
        crucible.dijkstra((Pos(Row(0), Col(0)), None, 0), |(pos, _, _)| {
            pos.0 == crucible.map.rows() - 1
                && pos.1 == crucible.map.cols() - 1
        });
    Ok(weight.try_into().unwrap())
}

pub fn part2(map: Grid<u8>) -> Result<i64> {
    let crucible = UltraCrucible { map };
    let (weight, _) = crucible.dijkstra(
        (Pos(Row(0), Col(0)), None, 0),
        |(pos, _, length)| {
            pos.0 == crucible.map.rows() - 1
                && pos.1 == crucible.map.cols() - 1
                && length + 1 >= 4
        },
    );
    Ok(weight.try_into().unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 17).unwrap()).unwrap()).unwrap(),
        1076
    );
    assert_eq!(
        part2(parse(parse::data(2023, 17).unwrap()).unwrap()).unwrap(),
        1219
    );
}
