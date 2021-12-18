use std::io::BufRead as _;

#[derive(Debug)]
struct Board {
    numbers: Vec<u8>,
    marked: Vec<bool>,
}

impl Board {
    fn new(numbers: Vec<u8>) -> Self {
        let len = numbers.len();
        Self {
            numbers,
            marked: vec![false; len],
        }
    }

    fn won(&self) -> bool {
        let wins = [
            [0, 1, 2, 3, 4],
            [5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14],
            [15, 16, 17, 18, 19],
            [20, 21, 22, 23, 24],
            [0, 5, 10, 15, 20],
            [1, 6, 11, 16, 21],
            [2, 7, 12, 17, 22],
            [3, 8, 13, 18, 23],
            [4, 9, 14, 19, 24],
        ];
        wins.iter().any(|win| win.iter().all(|i| self.marked[*i]))
    }

    fn mark(&mut self, called: u8) {
        for (i, n) in self.numbers.iter().enumerate() {
            if called == *n {
                self.marked[i] = true;
            }
        }
    }

    fn value(&self) -> i64 {
        self.marked
            .iter()
            .zip(self.numbers.iter())
            .filter_map(
                |(marked, n)| if !*marked { Some(*n as i64) } else { None },
            )
            .sum()
    }
}

#[derive(Debug)]
pub struct Game {
    inputs: Vec<u8>,
    boards: Vec<Board>,
}

impl Game {
    fn parse<T: std::io::Read>(input: T) -> anyhow::Result<Self> {
        let mut input = std::io::BufReader::new(input);
        let mut line = String::new();
        input.read_line(&mut line)?;
        let inputs = line
            .trim()
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<u8>, _>>()?;

        let mut boards = vec![];
        loop {
            if let Ok(0) = input.read_line(&mut line) {
                break;
            }
            let mut numbers = vec![];
            for _ in 0..5 {
                line.clear();
                input.read_line(&mut line)?;
                numbers.extend(
                    line.trim()
                        .split_whitespace()
                        .map(|s| s.parse())
                        .collect::<Result<Vec<u8>, _>>()?,
                );
            }
            boards.push(Board::new(numbers));
        }

        Ok(Self { inputs, boards })
    }

    fn find_first_winner(self) -> Option<(u8, Board)> {
        let Self { inputs, mut boards } = self;
        let mut won = None;
        for n in inputs {
            for (i, board) in boards.iter_mut().enumerate() {
                board.mark(n);
                if board.won() {
                    won = Some((n, i));
                    break;
                }
            }
            if won.is_some() {
                break;
            }
        }
        won.map(|(n, i)| (n, boards.swap_remove(i)))
    }

    fn find_last_winner(self) -> Option<(u8, Board)> {
        let Self { inputs, mut boards } = self;
        let mut last_won = None;
        for n in inputs {
            let mut won = vec![];
            for (i, board) in boards.iter_mut().enumerate() {
                board.mark(n);
                if board.won() {
                    won.push(i);
                }
            }
            if boards.len() == won.len() {
                last_won = Some((n, won[0]));
                break;
            }
            for i in won.into_iter().rev() {
                boards.swap_remove(i);
            }
            if boards.is_empty() {
                break;
            }
        }
        last_won.map(|(n, i)| (n, boards.swap_remove(i)))
    }
}

pub fn parse(fh: std::fs::File) -> anyhow::Result<Game> {
    Game::parse(fh)
}

pub fn part1(game: Game) -> anyhow::Result<i64> {
    if let Some((n, board)) = game.find_first_winner() {
        Ok((n as i64) * board.value())
    } else {
        anyhow::bail!("couldn't find winner")
    }
}

pub fn part2(game: Game) -> anyhow::Result<i64> {
    if let Some((n, board)) = game.find_last_winner() {
        Ok((n as i64) * board.value())
    } else {
        anyhow::bail!("couldn't find winner")
    }
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(crate::util::data(2021, 4).unwrap()).unwrap()).unwrap(),
        2745
    );
    assert_eq!(
        part2(parse(crate::util::data(2021, 4).unwrap()).unwrap()).unwrap(),
        6594
    );
}
