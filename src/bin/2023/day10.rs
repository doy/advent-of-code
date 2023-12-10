#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Tile {
    #[default]
    Floor,
    Start,
    UpDown,
    LeftRight,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Tile {
    fn points_left(&self) -> bool {
        matches!(
            self,
            Self::Start | Self::LeftRight | Self::UpLeft | Self::DownLeft
        )
    }

    fn points_right(&self) -> bool {
        matches!(
            self,
            Self::Start | Self::LeftRight | Self::UpRight | Self::DownRight
        )
    }

    fn points_up(&self) -> bool {
        matches!(
            self,
            Self::Start | Self::UpDown | Self::UpLeft | Self::UpRight
        )
    }

    fn points_down(&self) -> bool {
        matches!(
            self,
            Self::Start | Self::UpDown | Self::DownLeft | Self::DownRight
        )
    }

    fn connects(
        &self,
        other: &Self,
        direction: (Ordering, Ordering),
    ) -> bool {
        match direction {
            (Ordering::Less, Ordering::Equal) => {
                self.points_up() && other.points_down()
            }
            (Ordering::Greater, Ordering::Equal) => {
                self.points_down() && other.points_up()
            }
            (Ordering::Equal, Ordering::Less) => {
                self.points_left() && other.points_right()
            }
            (Ordering::Equal, Ordering::Greater) => {
                self.points_right() && other.points_left()
            }
            _ => false,
        }
    }
}

fn find_loop(map: &Grid<Tile>) -> (Vec<(Row, Col)>, Tile) {
    let mut cur = map
        .indexed_cells()
        .find_map(|(pos, tile)| {
            if *tile == Tile::Start {
                Some(pos)
            } else {
                None
            }
        })
        .unwrap();
    let mut pipe_loop = vec![];
    while pipe_loop.len() < 2
        || pipe_loop[0] != pipe_loop[pipe_loop.len() - 1]
    {
        pipe_loop.push(cur);
        for pos in map.adjacent(cur.0, cur.1, false) {
            if pipe_loop.len() > 1 && pos == pipe_loop[pipe_loop.len() - 2] {
                continue;
            }
            if map[cur.0][cur.1].connects(
                &map[pos.0][pos.1],
                (pos.0.cmp(&cur.0), pos.1.cmp(&cur.1)),
            ) {
                cur = pos;
                break;
            }
        }
    }
    pipe_loop.pop();
    let start_tile = match (
        pipe_loop[0].0.cmp(&pipe_loop[1].0),
        pipe_loop[0].1.cmp(&pipe_loop[1].1),
        pipe_loop[0].0.cmp(&pipe_loop[pipe_loop.len() - 1].0),
        pipe_loop[0].1.cmp(&pipe_loop[pipe_loop.len() - 1].1),
    ) {
        (Ordering::Less, _, Ordering::Greater, _)
        | (Ordering::Greater, _, Ordering::Less, _) => Tile::UpDown,
        (_, Ordering::Less, _, Ordering::Greater)
        | (_, Ordering::Greater, _, Ordering::Less) => Tile::LeftRight,
        (Ordering::Greater, _, _, Ordering::Greater)
        | (_, Ordering::Greater, Ordering::Greater, _) => Tile::UpLeft,
        (Ordering::Greater, _, _, Ordering::Less)
        | (_, Ordering::Less, Ordering::Greater, _) => Tile::UpRight,
        (Ordering::Less, _, _, Ordering::Greater)
        | (_, Ordering::Greater, Ordering::Less, _) => Tile::DownLeft,
        (Ordering::Less, _, _, Ordering::Less)
        | (_, Ordering::Less, Ordering::Less, _) => Tile::DownRight,
        _ => unreachable!(),
    };
    (pipe_loop, start_tile)
}

pub fn parse(fh: File) -> Result<Grid<Tile>> {
    Ok(parse::grid(parse::raw_lines(fh), |c, _, _| match c {
        b'.' => Tile::Floor,
        b'S' => Tile::Start,
        b'|' => Tile::UpDown,
        b'-' => Tile::LeftRight,
        b'L' => Tile::UpRight,
        b'J' => Tile::UpLeft,
        b'F' => Tile::DownRight,
        b'7' => Tile::DownLeft,
        _ => panic!("bad tile {c}"),
    }))
}

pub fn part1(map: Grid<Tile>) -> Result<i64> {
    Ok(i64::try_from(find_loop(&map).0.len()).unwrap() / 2)
}

pub fn part2(map: Grid<Tile>) -> Result<i64> {
    let (pipe_loop, start_tile) = find_loop(&map);
    let pipe_loop: HashSet<_> = pipe_loop.into_iter().collect();
    let mut total = 0;
    for ((row, col), tile) in map.indexed_cells() {
        if pipe_loop.contains(&(row, col)) {
            continue;
        }
        let mut count = 0;
        for offset in 0..=(row.0.min(col.0)) {
            let check_row = row - offset;
            let check_col = col - offset;
            if !pipe_loop.contains(&(check_row, check_col)) {
                continue;
            }
            let check_tile = map[check_row][check_col];
            let check_tile = if matches!(check_tile, Tile::Start) {
                start_tile
            } else {
                check_tile
            };
            if matches!(
                check_tile,
                Tile::UpDown
                    | Tile::LeftRight
                    | Tile::UpLeft
                    | Tile::DownRight
            ) {
                count += 1;
            }
        }
        if count % 2 == 1 {
            total += 1;
        }
    }
    Ok(total)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 10).unwrap()).unwrap()).unwrap(),
        7086
    );
    assert_eq!(
        part2(parse(parse::data(2023, 10).unwrap()).unwrap()).unwrap(),
        0
    );
}
