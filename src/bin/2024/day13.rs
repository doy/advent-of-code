use advent_of_code::prelude::*;

#[derive(Debug)]
pub struct Game {
    a: Pos,
    b: Pos,
    prize: Pos,
}

pub fn parse(fh: File) -> Result<Vec<Game>> {
    let mut games = vec![];
    let lines = parse::raw_lines(fh);
    let mut lines = lines.peekable();
    while lines.peek().is_some() {
        let mut chunk = parse::chunk(&mut lines);
        let a_line = chunk.next().unwrap();
        let b_line = chunk.next().unwrap();
        let prize_line = chunk.next().unwrap();
        assert!(chunk.next().is_none());

        let a_cap =
            regex_captures!(r"^Button A: X\+([0-9]+), Y\+([0-9]+)$", &a_line)
                .unwrap();
        let b_cap =
            regex_captures!(r"^Button B: X\+([0-9]+), Y\+([0-9]+)$", &b_line)
                .unwrap();
        let prize_cap =
            regex_captures!(r"^Prize: X=([0-9]+), Y=([0-9]+)$", &prize_line)
                .unwrap();
        games.push(Game {
            a: Pos(
                Row(a_cap[1].parse().unwrap()),
                Col(a_cap[2].parse().unwrap()),
            ),
            b: Pos(
                Row(b_cap[1].parse().unwrap()),
                Col(b_cap[2].parse().unwrap()),
            ),
            prize: Pos(
                Row(prize_cap[1].parse().unwrap()),
                Col(prize_cap[2].parse().unwrap()),
            ),
        })
    }
    Ok(games)
}

pub fn part1(games: Vec<Game>) -> Result<i64> {
    let mut total = 0;
    for game in games {
        let a = i64::try_from(game.a.0 .0).unwrap();
        let b = i64::try_from(game.b.0 .0).unwrap();
        let c = i64::try_from(game.a.1 .0).unwrap();
        let d = i64::try_from(game.b.1 .0).unwrap();
        let x = i64::try_from(game.prize.0 .0).unwrap();
        let y = i64::try_from(game.prize.1 .0).unwrap();
        let det = a * d - b * c;
        if det == 0
            || (d * x - b * y) % det != 0
            || (a * y - c * x) % det != 0
        {
            continue;
        }
        let a_presses = (d * x - b * y) / det;
        let b_presses = (a * y - c * x) / det;
        total += a_presses * 3 + b_presses;
    }
    Ok(total)
}

pub fn part2(games: Vec<Game>) -> Result<i64> {
    let mut total = 0;
    for game in games {
        let a = i64::try_from(game.a.0 .0).unwrap();
        let b = i64::try_from(game.b.0 .0).unwrap();
        let c = i64::try_from(game.a.1 .0).unwrap();
        let d = i64::try_from(game.b.1 .0).unwrap();
        let x = i64::try_from(game.prize.0 .0 + 10000000000000).unwrap();
        let y = i64::try_from(game.prize.1 .0 + 10000000000000).unwrap();
        let det = a * d - b * c;
        if det == 0
            || (d * x - b * y) % det != 0
            || (a * y - c * x) % det != 0
        {
            continue;
        }
        let a_presses = (d * x - b * y) / det;
        let b_presses = (a * y - c * x) / det;
        total += a_presses * 3 + b_presses;
    }
    Ok(total)
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2024, 13).unwrap()).unwrap()).unwrap(),
        38839
    );
    assert_eq!(
        part2(parse(parse::data(2024, 13).unwrap()).unwrap()).unwrap(),
        75200131617108
    );
}
