#[derive(Clone)]
pub struct Game {
    p1_pos: i64,
    p2_pos: i64,
    p1_score: i64,
    p2_score: i64,
    rolls: i64,
    die_state: i64,
}

impl Game {
    fn new(p1: i64, p2: i64) -> Self {
        Self {
            p1_pos: p1,
            p2_pos: p2,
            p1_score: 0,
            p2_score: 0,
            rolls: 0,
            die_state: 0,
        }
    }

    fn roll_deterministic(&mut self) -> i64 {
        self.die_state += 3;
        self.rolls += 3;
        self.die_state * 3 - 3
    }

    fn score(&mut self, score: i64, p1: bool) {
        if p1 {
            self.p1_pos += score;
            while self.p1_pos > 10 {
                self.p1_pos -= 10;
            }
            self.p1_score += self.p1_pos;
        } else {
            self.p2_pos += score;
            while self.p2_pos > 10 {
                self.p2_pos -= 10;
            }
            self.p2_score += self.p2_pos;
        }
    }

    fn value(&self, threshold: i64) -> Option<i64> {
        if self.p1_score >= threshold {
            Some(self.p2_score * self.rolls)
        } else if self.p2_score >= threshold {
            Some(self.p1_score * self.rolls)
        } else {
            None
        }
    }

    fn run_dirac(&self, p1: bool) -> (i64, i64) {
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        {
            let mut clone = self.clone();
            clone.score(3, p1);
            if clone.value(21).is_some() {
                if p1 {
                    p1_wins += 1;
                } else {
                    p2_wins += 1;
                }
            } else {
                let wins = clone.run_dirac(!p1);
                p1_wins += wins.0;
                p2_wins += wins.1;
            }
        }
        {
            let mut clone = self.clone();
            clone.score(4, p1);
            if clone.value(21).is_some() {
                if p1 {
                    p1_wins += 3;
                } else {
                    p2_wins += 3;
                }
            } else {
                let wins = clone.run_dirac(!p1);
                p1_wins += wins.0 * 3;
                p2_wins += wins.1 * 3;
            }
        }
        {
            let mut clone = self.clone();
            clone.score(5, p1);
            if clone.value(21).is_some() {
                if p1 {
                    p1_wins += 6;
                } else {
                    p2_wins += 6;
                }
            } else {
                let wins = clone.run_dirac(!p1);
                p1_wins += wins.0 * 6;
                p2_wins += wins.1 * 6;
            }
        }
        {
            let mut clone = self.clone();
            clone.score(6, p1);
            if clone.value(21).is_some() {
                if p1 {
                    p1_wins += 7;
                } else {
                    p2_wins += 7;
                }
            } else {
                let wins = clone.run_dirac(!p1);
                p1_wins += wins.0 * 7;
                p2_wins += wins.1 * 7;
            }
        }
        {
            let mut clone = self.clone();
            clone.score(7, p1);
            if clone.value(21).is_some() {
                if p1 {
                    p1_wins += 6;
                } else {
                    p2_wins += 6;
                }
            } else {
                let wins = clone.run_dirac(!p1);
                p1_wins += wins.0 * 6;
                p2_wins += wins.1 * 6;
            }
        }
        {
            let mut clone = self.clone();
            clone.score(8, p1);
            if clone.value(21).is_some() {
                if p1 {
                    p1_wins += 3;
                } else {
                    p2_wins += 3;
                }
            } else {
                let wins = clone.run_dirac(!p1);
                p1_wins += wins.0 * 3;
                p2_wins += wins.1 * 3;
            }
        }
        {
            let mut clone = self.clone();
            clone.score(9, p1);
            if clone.value(21).is_some() {
                if p1 {
                    p1_wins += 1;
                } else {
                    p2_wins += 1;
                }
            } else {
                let wins = clone.run_dirac(!p1);
                p1_wins += wins.0;
                p2_wins += wins.1;
            }
        }
        (p1_wins, p2_wins)
    }
}

pub fn parse(fh: std::fs::File) -> anyhow::Result<Game> {
    let mut lines = crate::util::parse::lines(fh);
    let p1 = lines
        .next()
        .unwrap()
        .strip_prefix("Player 1 starting position: ")
        .unwrap()
        .parse()
        .unwrap();
    let p2 = lines
        .next()
        .unwrap()
        .strip_prefix("Player 2 starting position: ")
        .unwrap()
        .parse()
        .unwrap();
    Ok(Game::new(p1, p2))
}

pub fn part1(mut game: Game) -> anyhow::Result<i64> {
    let mut p1 = true;
    loop {
        if let Some(value) = game.value(1000) {
            return Ok(value);
        }
        let score = game.roll_deterministic();
        game.score(score, p1);
        p1 = !p1;
    }
}

pub fn part2(game: Game) -> anyhow::Result<i64> {
    let (p1, p2) = game.run_dirac(true);
    Ok(p1.max(p2))
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(crate::util::data(2021, 21).unwrap()).unwrap()).unwrap(),
        1004670
    );
    assert_eq!(
        part2(parse(crate::util::data(2021, 21).unwrap()).unwrap()).unwrap(),
        492043106122795
    );
}
