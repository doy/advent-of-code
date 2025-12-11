use advent_of_code::prelude::*;

#[derive(Debug)]
pub struct Instruction {
    direction: Direction,
    distance: isize,
    real_direction: Direction,
    real_distance: isize,
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let cap = regex_captures!(
            r"^(U|D|L|R) ([0-9]+) \(#([0-9a-f]{5})([0-9a-f])\)",
            s
        )
        .ok_or_else(|| anyhow::anyhow!("failed to parse line"))?;

        let direction = match &cap[1] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => bail!("unknown direction {}", &cap[1]),
        };
        let distance = cap[2].parse()?;
        let real_direction = match &cap[4] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => bail!("unknown real direction in {}", &cap[4]),
        };
        let real_distance = isize::from_str_radix(&cap[3], 16)?;

        Ok(Self {
            direction,
            distance,
            real_direction,
            real_distance,
        })
    }
}

pub fn parse(fh: File) -> Result<Vec<Instruction>> {
    Ok(parse::lines(fh).collect())
}

pub fn part1(instructions: Vec<Instruction>) -> Result<i64> {
    let mut map: Grid<bool> = Grid::default();
    let mut pos = IPos(IRow(0), ICol(0));
    let mut corners = vec![pos];
    for instruction in &instructions {
        let offset = instruction.direction.offset();
        pos = IPos(
            pos.0 + offset.0 * instruction.distance,
            pos.1 + offset.1 * instruction.distance,
        );
        corners.push(pos);
    }
    let min_row = corners.iter().map(|IPos(row, _)| row).min().unwrap();
    let min_col = corners.iter().map(|IPos(_, col)| col).min().unwrap();

    let starting_pos =
        Pos(Row(min_row.0.abs_diff(0)), Col(min_col.0.abs_diff(0)));
    let mut pos = starting_pos;
    for instruction in &instructions {
        let offset = instruction.direction.offset();
        for _ in 0..instruction.distance {
            pos = Pos(
                Row(pos.0.0.checked_add_signed(offset.0.0).unwrap()),
                Col(pos.1.0.checked_add_signed(offset.1.0).unwrap()),
            );
            map.grow(Size(pos.0 + 1, pos.1 + 1));
            map[pos.0][pos.1] = true;
        }
    }

    let trench: HashSet<_> = map
        .indexed_cells()
        .filter_map(|(pos, dug)| if *dug { Some(pos) } else { None })
        .collect();
    let mut internal_cell = None;
    for pos in map.adjacent(starting_pos, true) {
        if trench.contains(&pos) {
            continue;
        }
        let mut count = 0;
        for offset in 0..=(pos.0.0.min(pos.1.0)) {
            let check_row = pos.0 - offset;
            let check_col = pos.1 - offset;
            if trench.contains(&Pos(check_row, check_col)) {
                count += 1;
            }
        }
        if count % 2 == 1 {
            internal_cell = Some(pos);
        }
    }
    let internal_cell = internal_cell.unwrap();
    map.flood_fill(internal_cell, &true, |_, c| !*c, true);

    Ok(map.cells().filter(|dug| **dug).count().try_into().unwrap())
}

pub fn part2(instructions: Vec<Instruction>) -> Result<i64> {
    let mut pos = (IRow(0), ICol(0));

    let mut vertices = vec![pos];
    let mut last_left = false;
    for pair in instructions.windows(2) {
        let instruction = &pair[0];
        let next_instruction = &pair[1];
        let mut distance = instruction.real_distance + 1;
        if last_left {
            distance -= 1;
        }
        if instruction.real_direction.turn_left()
            == next_instruction.real_direction
        {
            distance -= 1;
            last_left = true;
        } else {
            last_left = false;
        }
        let offset = instruction.real_direction.offset();
        pos = (pos.0 + offset.0 * distance, pos.1 + offset.1 * distance);
        vertices.push(pos);
    }

    let mut area = 0;
    for i in 0..vertices.len() {
        let next = if i == vertices.len() - 1 { 0 } else { i + 1 };
        area += vertices[i].0.0 * vertices[next].1.0
            - vertices[next].0.0 * vertices[i].1.0;
    }
    area /= 2;

    Ok(area.abs().try_into().unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2023, 18).unwrap()).unwrap()).unwrap(),
        36807
    );
    assert_eq!(
        part2(parse(parse::data(2023, 18).unwrap()).unwrap()).unwrap(),
        48797603984357
    );
}
