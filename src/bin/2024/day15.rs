use advent_of_code::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
enum Cell {
    #[default]
    Floor,
    Box,
    BoxLeft,
    BoxRight,
    Wall,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Floor => '.',
                Self::Box => 'O',
                Self::BoxLeft => '[',
                Self::BoxRight => ']',
                Self::Wall => '#',
            }
        )
    }
}

pub struct Warehouse {
    map: Grid<Cell>,
    robot: Pos,
}

impl Warehouse {
    fn make_big(&mut self) {
        self.robot = Pos(self.robot.0, self.robot.1 * 2);
        self.map = self
            .map
            .indexed_cells()
            .flat_map(|(pos, cell)| {
                [
                    (
                        Pos(pos.0, pos.1 * 2),
                        match cell {
                            Cell::Floor => Cell::Floor,
                            Cell::Box => Cell::BoxLeft,
                            Cell::Wall => Cell::Wall,
                            Cell::BoxLeft | Cell::BoxRight => unreachable!(),
                        },
                    ),
                    (
                        Pos(pos.0, pos.1 * 2 + 1),
                        match cell {
                            Cell::Floor => Cell::Floor,
                            Cell::Box => Cell::BoxRight,
                            Cell::Wall => Cell::Wall,
                            Cell::BoxLeft | Cell::BoxRight => unreachable!(),
                        },
                    ),
                ]
            })
            .collect();
    }

    fn box_left(&self, pos: Pos) -> Pos {
        match self.map[pos] {
            Cell::BoxLeft => pos,
            Cell::BoxRight => Pos(pos.0, pos.1 - 1),
            _ => unreachable!(),
        }
    }
    fn box_right(&self, pos: Pos) -> Pos {
        match self.map[pos] {
            Cell::BoxLeft => Pos(pos.0, pos.1 + 1),
            Cell::BoxRight => pos,
            _ => unreachable!(),
        }
    }

    fn mv(&mut self, direction: Direction) {
        let next_pos =
            direction.move_checked(self.robot, self.map.size()).unwrap();
        match self.map[next_pos] {
            Cell::Floor => self.robot = next_pos,
            Cell::Box => {
                let mut box_end = next_pos;
                while self.map[box_end] == Cell::Box {
                    box_end = direction
                        .move_checked(box_end, self.map.size())
                        .unwrap();
                }
                match self.map[box_end] {
                    Cell::Floor => {
                        self.map[next_pos] = Cell::Floor;
                        self.map[box_end] = Cell::Box;
                        self.robot = next_pos;
                    }
                    Cell::Wall => {}
                    Cell::Box | Cell::BoxLeft | Cell::BoxRight => {
                        unreachable!()
                    }
                }
            }
            Cell::BoxLeft | Cell::BoxRight => {
                let mut box_end = self.box_left(next_pos);
                if direction.horizontal() {
                    while self.map[box_end] == Cell::BoxLeft
                        || self.map[box_end] == Cell::BoxRight
                    {
                        box_end = direction
                            .move_checked(box_end, self.map.size())
                            .unwrap();
                    }
                    match self.map[box_end] {
                        Cell::Floor => {
                            let mut pos = next_pos;
                            while pos != box_end {
                                self.map[pos] = match self.map[pos] {
                                    Cell::BoxLeft => Cell::BoxRight,
                                    Cell::BoxRight => Cell::BoxLeft,
                                    _ => unreachable!(),
                                };
                                pos = direction
                                    .move_checked(pos, self.map.size())
                                    .unwrap();
                            }
                            self.map[box_end] = self.map[next_pos];
                            self.map[next_pos] = Cell::Floor;
                            self.robot = next_pos;
                        }
                        Cell::Wall => {}
                        Cell::Box | Cell::BoxLeft | Cell::BoxRight => {
                            unreachable!()
                        }
                    }
                } else {
                    let mut to_check = VecDeque::new();
                    to_check.push_front(self.box_left(next_pos));
                    let mut boxes_to_move = vec![];
                    while let Some(pos) = to_check.pop_back() {
                        boxes_to_move.push(pos);
                        for next in [
                            direction
                                .move_checked(pos, self.map.size())
                                .unwrap(),
                            direction
                                .move_checked(
                                    self.box_right(pos),
                                    self.map.size(),
                                )
                                .unwrap(),
                        ] {
                            match self.map[next] {
                                Cell::Floor => {}
                                Cell::BoxLeft | Cell::BoxRight => {
                                    to_check.push_front(self.box_left(next))
                                }
                                Cell::Wall => return,
                                Cell::Box => unreachable!(),
                            }
                        }
                    }
                    for pos in boxes_to_move.into_iter().rev() {
                        let src_left = pos;
                        let src_right = Direction::Right
                            .move_checked(pos, self.map.size())
                            .unwrap();
                        let dest_left = direction
                            .move_checked(src_left, self.map.size())
                            .unwrap();
                        let dest_right = direction
                            .move_checked(src_right, self.map.size())
                            .unwrap();
                        self.map[src_left] = Cell::Floor;
                        self.map[src_right] = Cell::Floor;
                        self.map[dest_left] = Cell::BoxLeft;
                        self.map[dest_right] = Cell::BoxRight;
                    }
                    self.robot = next_pos;
                }
            }
            Cell::Wall => {}
        }
    }

    fn coord_sum(&self) -> i64 {
        self.map
            .indexed_cells()
            .filter(|(_, cell)| {
                **cell == Cell::Box || **cell == Cell::BoxLeft
            })
            .map(|(pos, _)| i64::try_from(pos.0 .0 * 100 + pos.1 .0).unwrap())
            .sum()
    }
}

pub fn parse(fh: File) -> Result<(Warehouse, Vec<Direction>)> {
    let mut lines = parse::raw_lines(fh);
    let mut robot = Pos::default();
    let map = parse::grid(parse::chunk(&mut lines), |c, pos| match c {
        b'.' => Cell::Floor,
        b'O' => Cell::Box,
        b'#' => Cell::Wall,
        b'@' => {
            robot = pos;
            Cell::Floor
        }
        _ => unreachable!(),
    });
    let mut moves = vec![];
    for line in lines {
        moves.extend(
            line.trim().bytes().map(|c| Direction::try_from(c).unwrap()),
        );
    }
    Ok((Warehouse { map, robot }, moves))
}

pub fn part1(
    (mut warehouse, moves): (Warehouse, Vec<Direction>),
) -> Result<i64> {
    for direction in moves {
        warehouse.mv(direction);
    }
    Ok(warehouse.coord_sum())
}

pub fn part2(
    (mut warehouse, moves): (Warehouse, Vec<Direction>),
) -> Result<i64> {
    warehouse.make_big();
    for direction in moves {
        warehouse.mv(direction);
    }
    Ok(warehouse.coord_sum())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 15).unwrap()).unwrap()).unwrap(),
        1371036
    );
    assert_eq!(
        part2(parse(parse::data(2024, 15).unwrap()).unwrap()).unwrap(),
        1392847
    );
}
